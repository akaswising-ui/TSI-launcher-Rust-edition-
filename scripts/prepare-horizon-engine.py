#!/usr/bin/env python3
"""Apply TSI branding and Android GPU compatibility to the pinned MIT engine snapshot.

This is a build-time patch only. It refuses an unreviewed upstream revision so that
patches cannot silently land on a different source tree.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

PINNED_ENGINE_COMMIT = "1788803beebf105abe58de72752d08a5f0e0ee76"


def replace_once(root: Path, relative: str, old: str, new: str, expected: int = 1) -> None:
    path = root / relative
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count == 0 and new in text:
        return
    if count != expected:
        raise SystemExit(
            f"Patch guard failed for {relative}: expected {expected} occurrence(s), found {count}"
        )
    path.write_text(text.replace(old, new), encoding="utf-8")


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: prepare-horizon-engine.py PATH_TO_PINNED_ENGINE")

    root = Path(sys.argv[1]).resolve()
    actual = subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "HEAD"], text=True
    ).strip()
    if actual != PINNED_ENGINE_COMMIT:
        raise SystemExit(
            f"Engine revision mismatch: expected {PINNED_ENGINE_COMMIT}, got {actual}"
        )

    # TSI package identity. Keep the original Java namespace because the upstream
    # NativeActivity bridge is declared in that package; Android install identity
    # is independently set to com.tsi.orbit.game.
    replace_once(
        root,
        "android/app/build.gradle",
        "applicationId 'com.lukagejak.horizon'",
        "applicationId 'com.tsi.orbit.game'",
    )
    replace_once(
        root,
        "android/app/src/main/res/values/strings.xml",
        "<string name=\"app_name\">Horizon</string>",
        "<string name=\"app_name\">TSI Orbit Game</string>",
    )
    replace_once(
        root,
        "android/app/src/main/AndroidManifest.xml",
        '<uses-feature android:name="android.hardware.vulkan.version" android:version="0x401000" android:required="true" />',
        '<uses-feature android:name="android.hardware.vulkan.version" android:version="0x400000" android:required="false" />',
    )
    replace_once(
        root,
        "android/app/src/main/AndroidManifest.xml",
        'android:authorities="com.lukagejak.horizon.update"',
        'android:authorities="com.tsi.orbit.update"',
    )
    replace_once(
        root,
        "android/app/src/main/java/com/lukagejak/horizon/ApkProvider.java",
        'static final String AUTHORITY = "com.lukagejak.horizon.update";',
        'static final String AUTHORITY = "com.tsi.orbit.update";',
    )
    replace_once(
        root,
        "android/app/src/main/AndroidManifest.xml",
        'android:icon="@mipmap/ic_launcher"',
        'android:icon="@drawable/ic_tsi_orbit"',
    )
    replace_once(
        root,
        "android/app/src/main/AndroidManifest.xml",
        'android:roundIcon="@mipmap/ic_launcher"',
        'android:roundIcon="@drawable/ic_tsi_orbit"',
    )

    # Never impersonate the port author's Microsoft OAuth application. The
    # player supplies their own approved Azure application ID in Settings.
    replace_once(
        root,
        "android/build.sh",
        'export HORIZON_MSA_CLIENT_ID="${HORIZON_MSA_CLIENT_ID:-5be5d759-7f2f-4790-be62-438977cdc7a5}"',
        'export HORIZON_MSA_CLIENT_ID="${HORIZON_MSA_CLIENT_ID:-}"',
    )

    # TSI release feed, not the upstream engine maintainer's APK releases.
    replace_once(
        root,
        "crates/horizon-client/src/android/update.rs",
        'const LATEST: &str = "https://api.github.com/repos/MocLG/horizon-android/releases/latest";',
        'const LATEST: &str = "https://api.github.com/repos/akaswising-ui/TSI-launcher-Rust-edition-/releases/latest";',
    )

    # Rebrand visible Android launcher text only; retain internal file formats,
    # package namespace, and engine identifiers for compatibility.
    launcher_path = "crates/horizon-client/src/android/launcher.rs"
    replace_once(
        root,
        launcher_path,
        'ui.text(left, 18.0, "Horizon", Face::Medium, 17.0, hex(P.ink, 1.0));',
        'ui.text(left, 18.0, "TSI ORBIT", Face::Medium, 17.0, hex(P.ink, 1.0));',
    )
    replace_once(
        root,
        launcher_path,
        'ui.c.measure("Horizon", Face::Medium, size)',
        'ui.c.measure("TSI Orbit", Face::Medium, size)',
    )
    replace_once(
        root,
        launcher_path,
        '            "Horizon",\n            Face::Medium,\n            size,',
        '            "TSI Orbit",\n            Face::Medium,\n            size,',
    )
    replace_once(
        root,
        launcher_path,
        'ui.text(r.x, y, "Horizon", Face::Medium, display, hex(P.ink, 1.0));',
        'ui.text(r.x, y, "TSI Orbit", Face::Medium, display, hex(P.ink, 1.0));',
    )
    replace_once(
        root,
        launcher_path,
        '&format!("Horizon {}", super::BUILD),',
        '&format!("TSI Orbit {}", super::BUILD),',
    )

    # The launcher's own wgpu renderer must test a real device on Vulkan before
    # falling back to GLES. An adapter existing is not enough: request_device
    # can still fail on a vendor driver.
    draw_path = "crates/horizon-client/src/android/draw.rs"
    replace_once(
        root,
        draw_path,
        "/// The window's surface and the launcher's pipeline.\npub struct Renderer {",
        """/// Runtime GPU details shown in TSI Orbit Settings.
#[derive(Clone, Debug, Default)]
pub struct GpuInfo {
    pub name: String,
    pub vendor_id: u32,
    pub driver: String,
    pub driver_info: String,
    pub backend: String,
    pub is_mali: bool,
    pub mali_model: String,
    pub mali_architecture: String,
    pub is_mali_g52: bool,
    pub supports_astc: bool,
    pub supports_etc2: bool,
    pub supports_bc: bool,
    pub max_texture_dimension_2d: u32,
    pub shader_model: String,
    pub downlevel_flags: String,
}

/// The window's surface and the launcher's pipeline.
pub struct Renderer {
    _instance: wgpu::Instance,
    gpu: GpuInfo,""",
    )
    replace_once(
        root,
        draw_path,
        """        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        descriptor.backends = wgpu::Backends::VULKAN | wgpu::Backends::GL;
        let instance = wgpu::Instance::new(descriptor);
        let scale = window.scale_factor() as f32;
        let size = window.inner_size();
        let surface = instance.create_surface(window)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Horizon launcher"),
                ..Default::default()
            })
            .await?;""",
        """        let scale = window.scale_factor() as f32;
        let size = window.inner_size();
        let mut selected = None;
        let mut failures = Vec::new();

        for (backend_name, backend) in [
            ("Vulkan", wgpu::Backends::VULKAN),
            ("OpenGL ES", wgpu::Backends::GL),
        ] {
            let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
            descriptor.backends = backend;
            let instance = wgpu::Instance::new(descriptor);
            let surface = match instance.create_surface(window.clone()) {
                Ok(surface) => surface,
                Err(error) => {
                    failures.push(format!("{backend_name} surface: {error}"));
                    continue;
                }
            };
            let adapter = match instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                    ..Default::default()
                })
                .await
            {
                Ok(adapter) => adapter,
                Err(error) => {
                    failures.push(format!("{backend_name} adapter: {error}"));
                    continue;
                }
            };
            let info = adapter.get_info();
            let identity = format!("{} {} {}", info.name, info.driver, info.driver_info)
                .to_ascii_lowercase();
            let is_mali = info.vendor == 0x13B5
                || identity.contains("mali")
                || identity.contains("panfrost")
                || identity.contains("arm limited");
            let compact: String = identity
                .chars()
                .filter(|character| character.is_ascii_alphanumeric())
                .collect();
            let mali_models = [
                ("Mali-G725", "malig725", "5th Gen"),
                ("Mali-G720", "malig720", "5th Gen"),
                ("Mali-G625", "malig625", "5th Gen"),
                ("Mali-G620", "malig620", "5th Gen"),
                ("Mali-G715", "malig715", "Valhall"),
                ("Mali-G710", "malig710", "Valhall"),
                ("Mali-G615", "malig615", "Valhall"),
                ("Mali-G610", "malig610", "Valhall"),
                ("Mali-G510", "malig510", "Valhall"),
                ("Mali-G310", "malig310", "Valhall"),
                ("Mali-G78", "malig78", "Valhall"),
                ("Mali-G77", "malig77", "Valhall"),
                ("Mali-G68", "malig68", "Valhall"),
                ("Mali-G57", "malig57", "Valhall"),
                ("Mali-G76", "malig76", "Bifrost"),
                ("Mali-G72", "malig72", "Bifrost"),
                ("Mali-G71", "malig71", "Bifrost"),
                ("Mali-G52", "malig52", "Bifrost"),
                ("Mali-G51", "malig51", "Bifrost"),
                ("Mali-G31", "malig31", "Bifrost"),
                ("Mali-T880", "malit880", "Midgard"),
                ("Mali-T860", "malit860", "Midgard"),
                ("Mali-T830", "malit830", "Midgard"),
                ("Mali-T820", "malit820", "Midgard"),
                ("Mali-T760", "malit760", "Midgard"),
                ("Mali-T720", "malit720", "Midgard"),
            ];
            let found_mali = mali_models.iter()
                .copied()
                .find(|(_, token, _)| compact.contains(*token));
            let is_mali_g52 = found_mali.is_some_and(|(_, token, _)| token == "malig52");
            let (mali_model, mali_architecture) = match found_mali {
                Some((_, "malig52", _)) if identity.contains("mc2") =>
                    ("Mali-G52 MC2".to_owned(), "Bifrost".to_owned()),
                Some((model, _, architecture)) =>
                    (model.to_owned(), architecture.to_owned()),
                None if is_mali => (
                    "Mali (model not reported)".to_owned(),
                    "Not identified".to_owned(),
                ),
                None => ("Not Mali".to_owned(), "Not applicable".to_owned()),
            };
            let adapter_features = adapter.features();
            let adapter_limits = adapter.limits();
            let downlevel = adapter.get_downlevel_capabilities();
            let required_features = adapter_features
                & (wgpu::Features::TEXTURE_COMPRESSION_ASTC
                    | wgpu::Features::TEXTURE_COMPRESSION_ETC2
                    | wgpu::Features::TEXTURE_COMPRESSION_BC);
            let required_limits = if is_mali {
                wgpu::Limits::downlevel_defaults().using_resolution(adapter_limits.clone())
            } else {
                adapter_limits.clone()
            };
            let (device, queue) = match adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: Some("TSI Orbit launcher"),
                    required_features,
                    required_limits,
                    ..Default::default()
                })
                .await
            {
                Ok(device_queue) => device_queue,
                Err(error) => {
                    failures.push(format!("{backend_name} device: {error}"));
                    continue;
                }
            };
            let gpu = GpuInfo {
                name: info.name,
                vendor_id: info.vendor,
                driver: info.driver,
                driver_info: info.driver_info,
                backend: backend_name.to_owned(),
                is_mali,
                mali_model: mali_model.to_owned(),
                mali_architecture: mali_architecture.to_owned(),
                is_mali_g52,
                supports_astc: adapter_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC),
                supports_etc2: adapter_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2),
                supports_bc: adapter_features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC),
                max_texture_dimension_2d: adapter_limits.max_texture_dimension_2d,
                shader_model: format!("{:?}", downlevel.shader_model),
                downlevel_flags: format!("{:?}", downlevel.flags),
            };
            selected = Some((instance, surface, device, queue, gpu));
            break;
        }

        let (_instance, surface, device, queue, gpu) = selected.ok_or_else(|| {
            format!(
                "No working Vulkan or OpenGL ES GPU device. {}",
                failures.join("; ")
            )
        })?;
        log::info!(
            "TSI Orbit GPU: {} / {} / {} / {} / Mali={}",
            gpu.name,
            gpu.backend,
            gpu.driver,
            gpu.driver_info,
            gpu.is_mali
        );""",
    )
    replace_once(
        root,
        draw_path,
        """        Ok(Self {
            surface,
            device,""",
        """        Ok(Self {
            _instance,
            gpu,
            surface,
            device,""",
    )
    replace_once(
        root,
        draw_path,
        """    pub fn resize(&mut self, width: u32, height: u32, scale: f32) {""",
        """    pub fn gpu_info(&self) -> &GpuInfo {
        &self.gpu
    }

    pub fn resize(&mut self, width: u32, height: u32, scale: f32) {""",
    )

    # Show the selected GPU, active backend and Mali detection on the Android
    # launcher Settings page. The game's own Graphics menu exposes Auto/Vulkan/GL.
    replace_once(
        root,
        launcher_path,
        """        // Microsoft account.
        let (row, i) = next(&mut y);""",
        """        // GPU identity and verified device capabilities from the launch screen's renderer.
        let row_report = Rect::new(r.x, y, r.w, ROW);
        let gpu_report = self.gpu_info.clone();
        if let Some(gpu) = gpu_report {
            ui.row(
                row_report,
                0,
                "GPU / renderer",
                &format!("{} · {} · {} · {}", gpu.name, gpu.backend, gpu.driver, gpu.driver_info),
                0.0,
            );
            y += ROW;
            let row = Rect::new(r.x, y, r.w, ROW);
            ui.row(row, 0, "GPU model / architecture", &format!("{} · {}", gpu.mali_model, gpu.mali_architecture), 0.0);
            y += ROW;
            let row = Rect::new(r.x, y, r.w, ROW);
            ui.row(
                row,
                0,
                "GPU capabilities",
                &format!(
                    "ASTC {} · ETC2 {} · BC {} · max texture {} · shader {} · downlevel {}",
                    if gpu.supports_astc { "yes" } else { "no" },
                    if gpu.supports_etc2 { "yes" } else { "no" },
                    if gpu.supports_bc { "yes" } else { "no" },
                    gpu.max_texture_dimension_2d,
                    gpu.shader_model,
                    gpu.downlevel_flags
                ),
                0.0,
            );
            y += ROW;
            if gpu.is_mali_g52 {
                let row = Rect::new(r.x, y, r.w, ROW);
                ui.row(row, 0, "Mali-G52 Performance", "8/6 chunks · 75% render scale · 60 FPS · VSync · Auto API", 74.0);
                if ui.row_button(row, "Apply", false, !self.busy()) {
                    self.apply_mali_g52_performance_profile();
                }
                y += ROW;
                if let Some(notice) = self.gpu_profile_notice.clone() {
                    let row = Rect::new(r.x, y, r.w, ROW);
                    ui.row(row, 0, "Profile status", &notice, 0.0);
                    y += ROW;
                }
            } else if gpu.is_mali {
                let row = Rect::new(r.x, y, r.w, ROW);
                ui.row(row, 0, "Mali compatibility", "Generic Mali detected; G52-specific tuning is hidden because the model was not reported.", 0.0);
                y += ROW;
            }
        } else {
            ui.row(
                row_report,
                0,
                "GPU / renderer",
                "Waiting for a surface-compatible Vulkan or OpenGL ES device.",
                0.0,
            );
            y += ROW;
        }

        // Microsoft account.
        let (row, i) = next(&mut y);""",
    )

    # The launcher temporarily takes Renderer out of self while drawing a frame.
    # Keep its immutable GPU report separately so Settings displays real values.
    replace_once(
        root,
        launcher_path,
        "    renderer: Option<Renderer>,\n    pointer: Pointer,",
        "    renderer: Option<Renderer>,\n    gpu_info: Option<super::draw::GpuInfo>,\n    gpu_profile_notice: Option<String>,\n    pointer: Pointer,",
    )
    replace_once(
        root,
        launcher_path,
        "            renderer: None,\n            pointer: Pointer::default(),",
        "            renderer: None,\n            gpu_info: None,\n            gpu_profile_notice: None,\n            pointer: Pointer::default(),",
    )
    replace_once(
        root,
        launcher_path,
        """            match Renderer::new(window.clone()) {
                Ok(r) => self.renderer = Some(r),
                Err(e) => log::error!("Horizon launcher: no renderer: {e}"),
            }""",
        """            match Renderer::new(window.clone()) {
                Ok(r) => {
                    self.gpu_info = Some(r.gpu_info().clone());
                    self.renderer = Some(r);
                }
                Err(e) => log::error!("TSI Orbit: no renderer: {e}"),
            }""",
    )
    # GPU diagnostics use an owned clone from Launcher.gpu_info, allowing Apply
    # to mutate the profile notice without borrowing the renderer.

    # Apply the Mali-G52 profile to the pinned game's own settings files.
    # This is opt-in; it changes the real game settings and takes effect next launch.
    replace_once(
        root,
        launcher_path,
        """impl Launcher {
    pub fn new(app: AndroidApp, paths: Paths, crash: Option<String>) -> Self {""",
        """impl Launcher {
    fn apply_mali_g52_performance_profile(&mut self) {
        let result = (|| -> std::result::Result<(), String> {
            std::fs::create_dir_all(&self.paths.home)
                .map_err(|error| format!("Cannot open the game settings folder: {error}"))?;
            let options_path = self.paths.home.join("options.txt");
            let mut settings = horizon_client_render::settings::Settings::load(&options_path);
            horizon_client_render::graphics::apply_preset(
                horizon_client_render::graphics::Preset::Performance,
                &mut settings,
            );
            settings.horizon.resolution_scale = 75;
            settings.max_fps = 60;
            settings.vsync = true;
            // Auto: Vulkan first, then GLES if the device/driver cannot initialize Vulkan.
            settings.horizon.api = horizon_client_render::graphics::GraphicsApi::Auto;
            settings.save(&options_path);

            let options = std::fs::read_to_string(&options_path)
                .map_err(|error| format!("Could not verify options.txt: {error}"))?;
            let graphics = std::fs::read_to_string(self.paths.home.join("horizon-graphics.txt"))
                .map_err(|error| format!("Could not verify horizon-graphics.txt: {error}"))?;
            if !options.contains("maxFps:60") || !graphics.contains("resolutionScale:75") {
                return Err("Settings were written but the profile could not be verified.".to_owned());
            }
            Ok(())
        })();
        self.gpu_profile_notice = Some(match result {
            Ok(()) => "Mali-G52 Performance saved: 75% scale, 8/6 chunks, 60 FPS, VSync, Auto Vulkan → GLES. Applies next game launch.".to_owned(),
            Err(error) => format!("Mali-G52 profile was not confirmed: {error}"),
        });
    }

    pub fn new(app: AndroidApp, paths: Paths, crash: Option<String>) -> Self {""",
    )

    # Record Mali model/architecture and real wgpu downlevel capabilities from
    # the selected 3D game adapter, not only the small launcher renderer.
    replace_once(
        root,
        "crates/horizon-client-render/src/gfx.rs",
        """fn describe_adapter(adapter: &wgpu::Adapter) {
    let info = adapter.get_info();
    log::info!(
        "Horizon renderer: {} ({:?}, {:?})",
        info.name,
        info.backend,
        info.device_type
    );""",
        """pub(crate) fn mali_gpu_details(info: &wgpu::AdapterInfo) -> Option<(String, String)> {
    let identity = format!("{} {} {}", info.name, info.driver, info.driver_info)
        .to_ascii_lowercase();
    let is_mali = info.vendor == 0x13B5
        || identity.contains("mali")
        || identity.contains("panfrost")
        || identity.contains("arm limited");
    if !is_mali {
        return None;
    }
    let compact: String = identity
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect();
    let models = [
        ("Mali-G725", "malig725", "5th Gen"),
        ("Mali-G720", "malig720", "5th Gen"),
        ("Mali-G625", "malig625", "5th Gen"),
        ("Mali-G620", "malig620", "5th Gen"),
        ("Mali-G715", "malig715", "Valhall"),
        ("Mali-G710", "malig710", "Valhall"),
        ("Mali-G615", "malig615", "Valhall"),
        ("Mali-G610", "malig610", "Valhall"),
        ("Mali-G510", "malig510", "Valhall"),
        ("Mali-G310", "malig310", "Valhall"),
        ("Mali-G78", "malig78", "Valhall"),
        ("Mali-G77", "malig77", "Valhall"),
        ("Mali-G68", "malig68", "Valhall"),
        ("Mali-G57", "malig57", "Valhall"),
        ("Mali-G76", "malig76", "Bifrost"),
        ("Mali-G72", "malig72", "Bifrost"),
        ("Mali-G71", "malig71", "Bifrost"),
        ("Mali-G52", "malig52", "Bifrost"),
        ("Mali-G51", "malig51", "Bifrost"),
        ("Mali-G31", "malig31", "Bifrost"),
        ("Mali-T880", "malit880", "Midgard"),
        ("Mali-T860", "malit860", "Midgard"),
        ("Mali-T830", "malit830", "Midgard"),
        ("Mali-T820", "malit820", "Midgard"),
        ("Mali-T760", "malit760", "Midgard"),
        ("Mali-T720", "malit720", "Midgard"),
    ];
    if let Some((model, token, architecture)) = models
        .iter()
        .copied()
        .find(|(_, token, _)| compact.contains(*token))
    {
        let name = if token == "malig52" && identity.contains("mc2") {
            "Mali-G52 MC2"
        } else {
            model
        };
        return Some((name.to_owned(), architecture.to_owned()));
    }
    Some(("Mali (model not reported)".to_owned(), "Not identified".to_owned()))
}

fn describe_adapter(adapter: &wgpu::Adapter) {
    let info = adapter.get_info();
    let mali = mali_gpu_details(&info);
    if let Some((model, architecture)) = &mali {
        log::info!(
            "Horizon renderer: ARM GPU profile: {model} / {architecture} / driver {} {}",
            info.driver,
            info.driver_info
        );
    }
    let downlevel = adapter.get_downlevel_capabilities();
    log::info!(
        "Horizon renderer: {} ({:?}, {:?}) / shader model {:?} / downlevel {:?}",
        info.name,
        info.backend,
        info.device_type,
        downlevel.shader_model,
        downlevel.flags
    );""",
    )

    # Compressed texture formats are enabled only when the actual game adapter
    # advertises support. This keeps ASTC/ETC2/BC optional on mobile GPUs.
    replace_once(
        root,
        "crates/horizon-client-render/src/gfx/surface_setup.rs",
        """                | wgpu::Features::CLEAR_TEXTURE
                | wgpu::Features::TEXTURE_ATOMIC
                | wgpu::Features::FLOAT32_FILTERABLE));""",
        """                | wgpu::Features::CLEAR_TEXTURE
                | wgpu::Features::TEXTURE_ATOMIC
                | wgpu::Features::FLOAT32_FILTERABLE
                | wgpu::Features::TEXTURE_COMPRESSION_ASTC
                | wgpu::Features::TEXTURE_COMPRESSION_ETC2
                | wgpu::Features::TEXTURE_COMPRESSION_BC));""",
    )

    # Rebrand the app's startup logs, without renaming Minecraft data paths.
    replace_once(
        root,
        "crates/horizon-client/src/android/mod.rs",
        'log::info!("Horizon {BUILD} on {}"',
        'log::info!("TSI Orbit {BUILD} on {}"',
    )

    # Ship required notices in the APK's assets along with the game's prepared data.
    replace_once(
        root,
        "android/build.sh",
        '(cd "$root" && zip -qr -X "$stage/assets/horizon.zip" .)\n\necho "== The APK"',
        """(cd "$root" && zip -qr -X "$stage/assets/horizon.zip" .)
tsi_repo=$(cd "$repo/../.." && pwd)
cp "$tsi_repo/LICENSE" "$stage/assets/tsi-orbit-license.txt"
cp "$repo/LICENSE" "$stage/assets/horizon-engine-license.txt"
cp "$tsi_repo/THIRD_PARTY_NOTICES.md" "$stage/assets/third-party-notices.txt"
cp "$repo/THIRD_PARTY_NOTICES.md" "$stage/assets/horizon-third-party-notices.txt"

echo "== The APK" """,
    )

    # Original vector identity, not Horizon artwork.
    icon = root / "android/app/src/main/res/drawable/ic_tsi_orbit.xml"
    icon.parent.mkdir(parents=True, exist_ok=True)
    icon.write_text(
        """<?xml version="1.0" encoding="utf-8"?>
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="108dp" android:height="108dp"
    android:viewportWidth="108" android:viewportHeight="108">
    <path android:fillColor="#0B0F16" android:pathData="M0,0h108v108h-108z"/>
    <path android:fillColor="#16353E" android:pathData="M54,9a45,45 0,1 0,0.1 0z"/>
    <path android:fillColor="#00000000" android:strokeColor="#46E2C7"
        android:strokeWidth="4" android:pathData="M17,54C17,32 37,19 54,19s37,13 37,35 -20,35 -37,35S17,76 17,54z"/>
    <path android:fillColor="#00000000" android:strokeColor="#5CA0FF"
        android:strokeWidth="3" android:pathData="M26,39C40,19 75,22 84,46 93,70 70,89 46,83 22,77 14,55 26,39z"/>
    <path android:fillColor="#46E2C7" android:pathData="M54,43a11,11 0,1 0,0.1 0z"/>
    <path android:fillColor="#F6B958" android:pathData="M82,23a5,5 0,1 0,0.1 0z"/>
</vector>
""",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
