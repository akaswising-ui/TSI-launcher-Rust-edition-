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
    # is independently set to com.tsi.orbit.
    replace_once(
        root,
        "android/app/build.gradle",
        "applicationId 'com.lukagejak.horizon'",
        "applicationId 'com.tsi.orbit'",
    )
    replace_once(
        root,
        "android/app/src/main/res/values/strings.xml",
        "<string name=\"app_name\">Horizon</string>",
        "<string name=\"app_name\">TSI Orbit</string>",
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
            let (device, queue) = match adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: Some("TSI Orbit launcher"),
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
            let identity = format!("{} {} {}", info.name, info.driver, info.driver_info)
                .to_ascii_lowercase();
            let gpu = GpuInfo {
                name: info.name,
                vendor_id: info.vendor,
                driver: info.driver,
                driver_info: info.driver_info,
                backend: backend_name.to_owned(),
                is_mali: info.vendor == 0x13B5
                    || identity.contains("mali")
                    || identity.contains("panfrost")
                    || identity.contains("arm limited"),
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
        """        // GPU identity from the launcher's real surface/device open.
        let (row, i) = next(&mut y);
        if let Some(renderer) = self.renderer.as_ref() {
            let gpu = renderer.gpu_info();
            let detail = format!(
                "{} · {} · {} · {} · Mali {}",
                gpu.name,
                gpu.backend,
                gpu.driver,
                gpu.driver_info,
                if gpu.is_mali { "detected" } else { "not detected" }
            );
            ui.row(row, i, "GPU / renderer", &detail, 0.0);
        } else {
            ui.row(
                row,
                i,
                "GPU / renderer",
                "Waiting for a surface-compatible Vulkan or OpenGL ES device.",
                0.0,
            );
        }

        // Microsoft account.
        let (row, i) = next(&mut y);""",
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
