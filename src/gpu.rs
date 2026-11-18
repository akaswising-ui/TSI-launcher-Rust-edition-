//! Runtime GPU selection for TSI Orbit.
//!
//! Vulkan is preferred, but a real device + surface must pass adapter selection.
//! OpenGL ES remains available as a compatibility backend, including for Mali
//! devices whose current Vulkan driver cannot create a usable wgpu device.

use std::sync::{Arc, Mutex};

use eframe::egui_wgpu::{
    WgpuSetup, WgpuSetupCreateNew,
    wgpu::{self, Adapter, Backend, Backends, DeviceDescriptor, Features, Limits, Surface},
};

#[derive(Clone, Debug)]
pub struct GpuAdapterReport {
    pub name: String,
    pub vendor_id: u32,
    pub vendor_label: String,
    pub driver: String,
    pub driver_info: String,
    pub backend: String,
    pub device_type: String,
    pub is_mali: bool,
    pub mali_model: String,
    pub mali_architecture: String,
    pub is_mali_g52: bool,
    pub recommended_profile: String,
    pub surface_compatible: bool,
    pub device_request_ok: bool,
    pub selected: bool,
    pub texture_compression: String,
    pub supports_astc: bool,
    pub supports_etc2: bool,
    pub supports_bc: bool,
    pub max_texture_dimension_2d: u32,
    pub max_buffer_size: u64,
    pub max_storage_buffer_binding_size: u64,
    pub shader_model: String,
    pub downlevel_flags: String,
    pub webgpu_compliant: bool,
    pub mali_profile: String,
}

#[derive(Clone, Debug)]
pub struct GpuRuntimeSnapshot {
    pub status: String,
    pub selected_backend: String,
    pub selected_gpu: String,
    pub error: Option<String>,
    pub adapters: Vec<GpuAdapterReport>,
}

impl Default for GpuRuntimeSnapshot {
    fn default() -> Self {
        Self {
            status: "Waiting for GPU adapter selection".to_owned(),
            selected_backend: "Not selected".to_owned(),
            selected_gpu: "Not selected".to_owned(),
            error: None,
            adapters: Vec::new(),
        }
    }
}

#[derive(Clone, Default)]
pub struct GpuRuntimeState {
    inner: Arc<Mutex<GpuRuntimeSnapshot>>,
}

impl GpuRuntimeState {
    pub fn snapshot(&self) -> GpuRuntimeSnapshot {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn set_snapshot(&self, snapshot: GpuRuntimeSnapshot) {
        *self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = snapshot;
    }
}

/// Configures the actual eframe UI renderer to prefer Vulkan and fall back to GL/GLES.
/// Adapter selection validates surface support and opens a real wgpu device before
/// returning an adapter to eframe.
pub fn configure_native_options(
    options: &mut eframe::NativeOptions,
    runtime_state: GpuRuntimeState,
) {
    options.renderer = eframe::Renderer::Wgpu;

    let mut setup = WgpuSetupCreateNew::without_display_handle();
    setup.instance_descriptor.backends = Backends::VULKAN | Backends::GL;
    setup.power_preference = wgpu::PowerPreference::HighPerformance;

    let selector_state = runtime_state.clone();
    setup.native_adapter_selector = Some(Arc::new(move |adapters, surface| {
        let reports = adapters
            .iter()
            .map(|adapter| adapter_report(adapter, surface, false, false))
            .collect::<Vec<_>>();

        let mut candidates = adapters
            .iter()
            .filter(|adapter| surface.is_none_or(|surface| adapter.is_surface_supported(surface)))
            .collect::<Vec<_>>();

        candidates.sort_by_key(|adapter| {
            let info = adapter.get_info();
            let backend_rank = match info.backend {
                Backend::Vulkan => 0_u8,
                Backend::Gl => 1_u8,
                _ => 2_u8,
            };
            let device_type = format!("{:?}", info.device_type).to_ascii_lowercase();
            let device_rank = if device_type.contains("cpu") {
                2_u8
            } else if device_type.contains("virtual") {
                1_u8
            } else {
                0_u8
            };
            (backend_rank, device_rank)
        });

        let mut failures = Vec::new();
        for adapter in candidates {
            let info = adapter.get_info();
            let descriptor = device_descriptor(adapter);
            match pollster::block_on(adapter.request_device(&descriptor)) {
                Ok((_device, _queue)) => {
                    let selected_key = adapter_key(adapter);
                    let selected_reports = adapters
                        .iter()
                        .map(|candidate| {
                            let selected = adapter_key(candidate) == selected_key;
                            adapter_report(candidate, surface, selected, selected)
                        })
                        .collect::<Vec<_>>();

                    selector_state.set_snapshot(GpuRuntimeSnapshot {
                        status: "GPU device initialized".to_owned(),
                        selected_backend: backend_label(info.backend).to_owned(),
                        selected_gpu: info.name.clone(),
                        error: None,
                        adapters: selected_reports,
                    });

                    return Ok(adapter.clone());
                }
                Err(error) => {
                    failures.push(format!(
                        "{} ({}): {error}",
                        info.name,
                        backend_label(info.backend)
                    ));
                }
            }
        }

        selector_state.set_snapshot(GpuRuntimeSnapshot {
            status: "No compatible GPU device".to_owned(),
            selected_backend: "Unavailable".to_owned(),
            selected_gpu: "No adapter passed validation".to_owned(),
            error: Some(if failures.is_empty() {
                "No Vulkan or OpenGL ES adapter can present to this app surface.".to_owned()
            } else {
                failures.join("\n")
            }),
            adapters: reports,
        });

        Err("TSI Orbit could not open a surface-compatible Vulkan or OpenGL ES device.".to_owned())
    }));

    setup.device_descriptor = Arc::new(device_descriptor);
    options.wgpu_options = eframe::WgpuConfiguration {
        wgpu_setup: WgpuSetup::CreateNew(setup),
        ..Default::default()
    };
}

fn device_descriptor(adapter: &Adapter) -> DeviceDescriptor<'static> {
    let info = adapter.get_info();
    let limits = if info.backend == Backend::Gl {
        Limits::downlevel_webgl2_defaults()
    } else if is_mali_adapter(info.vendor, &info.name, &info.driver, &info.driver_info) {
        // Keep the Android Mali baseline conservative. Optional texture formats are
        // enabled only when the driver advertises them; no unsupported feature is forced.
        Limits::downlevel_defaults()
    } else {
        Limits::default()
    };

    let required_features = requested_texture_compression_features(adapter.features());

    DeviceDescriptor {
        label: Some("TSI Orbit WGPU device"),
        required_features,
        required_limits: limits,
        ..Default::default()
    }
}

fn requested_texture_compression_features(available: Features) -> Features {
    [
        Features::TEXTURE_COMPRESSION_ASTC,
        Features::TEXTURE_COMPRESSION_ETC2,
        Features::TEXTURE_COMPRESSION_BC,
    ]
    .into_iter()
    .filter(|feature| available.contains(*feature))
    .fold(Features::empty(), |enabled, feature| enabled | feature)
}

fn adapter_report(
    adapter: &Adapter,
    surface: Option<&Surface<'_>>,
    selected: bool,
    device_request_ok: bool,
) -> GpuAdapterReport {
    let info = adapter.get_info();
    let features = adapter.features();
    let limits = adapter.limits();
    let downlevel = adapter.get_downlevel_capabilities();
    let mali = classify_mali_gpu(info.vendor, &info.name, &info.driver, &info.driver_info);
    let is_mali = mali.is_some();
    let mali_model = mali.as_ref().map_or_else(|| "Not reported".to_owned(), |gpu| gpu.model.clone());
    let mali_architecture = mali.as_ref().map_or_else(|| "Not reported".to_owned(), |gpu| gpu.architecture.clone());
    let is_mali_g52 = mali.as_ref().is_some_and(|gpu| gpu.is_g52);
    let recommended_profile = mali.as_ref().map_or_else(
        || "No model-specific profile".to_owned(),
        |gpu| gpu.recommended_profile.clone(),
    );
    let supports_astc = features.contains(Features::TEXTURE_COMPRESSION_ASTC);
    let supports_etc2 = features.contains(Features::TEXTURE_COMPRESSION_ETC2);
    let supports_bc = features.contains(Features::TEXTURE_COMPRESSION_BC);

    let texture_compression = if supports_astc {
        "ASTC"
    } else if supports_etc2 {
        "ETC2"
    } else if supports_bc {
        "BC"
    } else {
        "Uncompressed fallback"
    };

    let surface_compatible = surface.is_none_or(|surface| adapter.is_surface_supported(surface));

    GpuAdapterReport {
        name: info.name,
        vendor_id: info.vendor,
        vendor_label: if info.vendor == 0x13B5 {
            "ARM".to_owned()
        } else {
            format!("0x{:04X}", info.vendor)
        },
        driver: info.driver,
        driver_info: info.driver_info,
        backend: backend_label(info.backend).to_owned(),
        device_type: format!("{:?}", info.device_type),
        is_mali,
        mali_model,
        mali_architecture,
        is_mali_g52,
        recommended_profile,
        surface_compatible,
        device_request_ok,
        selected,
        texture_compression: texture_compression.to_owned(),
        supports_astc,
        supports_etc2,
        supports_bc,
        max_texture_dimension_2d: limits.max_texture_dimension_2d,
        max_buffer_size: limits.max_buffer_size,
        max_storage_buffer_binding_size: limits.max_storage_buffer_binding_size,
        shader_model: format!("{:?}", downlevel.shader_model),
        downlevel_flags: format!("{:?}", downlevel.flags),
        webgpu_compliant: downlevel.is_webgpu_compliant(),
        mali_profile: if is_mali_g52 {
            "Bifrost G52: conservative limits, driver-verified compression, Auto Vulkan → GLES"
                .to_owned()
        } else if is_mali {
            "Mali family: conservative limits, optional features only when reported"
                .to_owned()
        } else {
            "Standard adapter limits".to_owned()
        },
    }
}

fn adapter_key(adapter: &Adapter) -> (Backend, u32, u32, String) {
    let info = adapter.get_info();
    (info.backend, info.vendor, info.device, info.name)
}

fn backend_label(backend: Backend) -> &'static str {
    match backend {
        Backend::Vulkan => "Vulkan",
        Backend::Gl => "OpenGL ES / GL",
        Backend::Metal => "Metal",
        Backend::Dx12 => "Direct3D 12",
        Backend::BrowserWebGpu => "WebGPU",
        Backend::Noop => "No-op",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MaliGpuDetails {
    model: String,
    architecture: String,
    is_g52: bool,
    recommended_profile: String,
}

/// Classify the model only when the adapter/driver reports enough information.
/// Vendor ID alone establishes the ARM GPU family, never a specific Mali model.
fn classify_mali_gpu(
    vendor_id: u32,
    name: &str,
    driver: &str,
    driver_info: &str,
) -> Option<MaliGpuDetails> {
    let identity = format!("{name} {driver} {driver_info}").to_ascii_lowercase();
    let is_mali = vendor_id == 0x13B5
        || identity.contains("mali")
        || identity.contains("panfrost")
        || identity.contains("arm limited");
    if !is_mali {
        return None;
    }

    let g52 = identity.contains("mali-g52") || identity.contains("mali g52");
    if g52 {
        let has_mc2 = identity.contains("mc2");
        return Some(MaliGpuDetails {
            model: if has_mc2 { "Mali-G52 MC2" } else { "Mali-G52" }.to_owned(),
            architecture: "Bifrost".to_owned(),
            is_g52: true,
            recommended_profile: if has_mc2 {
                "Mali-G52 MC2 Performance: 75% render scale, 8/6 chunks, 60 FPS, VSync, Auto API"
            } else {
                "Mali-G52 Performance: 75% render scale, 8/6 chunks, 60 FPS, VSync, Auto API"
            }
            .to_owned(),
        });
    }

    Some(MaliGpuDetails {
        model: "Mali (model not reported)".to_owned(),
        architecture: "Not identified".to_owned(),
        is_g52: false,
        recommended_profile: "Generic Mali-safe limits; keep renderer API on Auto".to_owned(),
    })
}

fn is_mali_adapter(vendor_id: u32, name: &str, driver: &str, driver_info: &str) -> bool {
    classify_mali_gpu(vendor_id, name, driver, driver_info).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_arm_vendor_id_as_mali_family_without_guessing_a_model() {
        let gpu = classify_mali_gpu(0x13B5, "Unknown GPU", "", "").unwrap();
        assert_eq!(gpu.model, "Mali (model not reported)");
        assert_eq!(gpu.architecture, "Not identified");
        assert!(!gpu.is_g52);
    }

    #[test]
    fn recognizes_mali_g52_mc2_and_bifrost_profile() {
        let gpu = classify_mali_gpu(0x13B5, "ARM Mali-G52 MC2", "ARM", "r0p0").unwrap();
        assert_eq!(gpu.model, "Mali-G52 MC2");
        assert_eq!(gpu.architecture, "Bifrost");
        assert!(gpu.is_g52);
        assert!(gpu.recommended_profile.contains("75% render scale"));
        assert!(gpu.recommended_profile.contains("Auto API"));
    }

    #[test]
    fn recognizes_g52_without_inventing_the_mc_variant() {
        let gpu = classify_mali_gpu(0, "Mali-G52", "ARM", "").unwrap();
        assert_eq!(gpu.model, "Mali-G52");
        assert!(gpu.is_g52);
        assert!(!gpu.model.contains("MC2"));
    }

    #[test]
    fn recognizes_pan_vk_driver_as_mali_family() {
        assert!(is_mali_adapter(0, "Unknown", "Panfrost", ""));
    }

    #[test]
    fn does_not_mark_adreno_as_mali() {
        assert!(!is_mali_adapter(0x5143, "Adreno 610", "Qualcomm", ""));
    }

    #[test]
    fn requests_only_reported_texture_compression_features() {
        let available =
            Features::TEXTURE_COMPRESSION_ASTC | Features::TEXTURE_COMPRESSION_ETC2;
        let requested = requested_texture_compression_features(available);

        assert!(requested.contains(Features::TEXTURE_COMPRESSION_ASTC));
        assert!(requested.contains(Features::TEXTURE_COMPRESSION_ETC2));
        assert!(!requested.contains(Features::TEXTURE_COMPRESSION_BC));
    }

    #[test]
    fn requests_no_texture_features_when_driver_reports_none() {
        assert_eq!(
            requested_texture_compression_features(Features::empty()),
            Features::empty()
        );
    }
}
