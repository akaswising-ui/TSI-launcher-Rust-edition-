# TSI Launcher Rust Edition

**TSI Orbit** is an independent Rust-native launcher foundation for Android and desktop development. Its visual direction takes broad inspiration from modern game launchers: dark surfaces, a focused game hero, clean navigation, profile controls and a compact library. It does not copy Horizon's source code, assets, logos, screenshots, text or exact layout.

## Language and platform

- Application code: **Rust only**.
- Android entry: Rust `android_main` + Winit NativeActivity backend.
- UI: eframe / egui, rendered through wgpu.
- No Java, Kotlin, C++, JavaScript, webview or embedded website code in the application.
- Android target in the first pass: `aarch64-linux-android` (arm64-v8a).
- Desktop preview: `cargo run`.

## Current 0.1.0 foundation

Implemented in Rust:
- Responsive launcher shell with desktop sidebar and compact mobile navigation.
- Home dashboard, client/runtime status, versions, worlds, native mods, resource packs, servers, touch controls, accounts, downloads and settings pages.
- Persistent local profiles, saved server addresses, mod URL queue, pack names and UI preferences.
- Four editable touch scheme presets with button scale / opacity, handedness, sprint options, 360-degree movement and vibration preferences.
- Honest not-connected states for engine, downloads, account auth and gameplay, rather than pretending these systems already work.
- Original vector-drawn Orbit artwork and a distinct teal / graphite design system.

## Horizon-inspired capability map

The public Horizon Android feature list describes a native Rust game client plus launcher integrations including Vulkan rendering, touch layouts, local profiles/accounts, verified resource downloads, resource packs, native .hmod mods, servers, sound, clipboard, gamepad / keyboard support, worlds and saves. TSI uses the same broad capability areas as a roadmap, but does not copy Horizon code, assets, text or exact UI layout.

## GPU and renderer selection

The launcher UI's wgpu renderer now uses this selection order:

1. Enumerate surface-compatible adapters and prefer Vulkan.
2. Request a real wgpu device with conservative baseline limits for detected Mali adapters.
3. If Vulkan adapter/device initialization fails, try the OpenGL ES / GL backend.
4. Enable ASTC, ETC2 or BC texture-compression features only when the selected adapter reports support.
5. Show the selected GPU model, vendor ID, driver strings, backend, device type, compression features and reported limits in Settings.

Mali detection uses runtime adapter information (ARM vendor ID and GPU/driver strings); the app does not assume every Mali model or vendor driver has the same Vulkan version or extensions. The report shows wgpu-visible capabilities rather than inventing a Vulkan API version that the wgpu adapter metadata does not expose.

**Important:** this currently configures the launcher UI renderer. It is not yet the separate 3D Minecraft game renderer, and a CI build cannot prove that a specific phone's proprietary GPU driver works. The Android device still needs a physical Vulkan/Mali test. If its Vulkan driver cannot create a usable surface/device, the UI selector tries OpenGL ES instead.

Planned engineering phases:
1. Game runtime and device foundation: independent Rust game-engine decision, game renderer, Vulkan surface/device integration, device lifecycle and physical Mali/Adreno/PowerVR validation. The launcher UI already probes Vulkan and has an OpenGL ES fallback.
2. Content pipeline: verified HTTPS downloads, resumable transfers, manifest pinning, resource cache and install/rollback.
3. Library and storage: profiles, worlds, backup/export, resource packs and import.
4. Mod system: sandbox design, capability manifest, signature/checksum verification and native module loading. Java .jar, Fabric / Forge / Mixin compatibility is not implied.
5. Multiplayer and accounts: server protocol support, production OAuth flow, token security and session handoff.
6. Input and platform: touch editor, keyboard/mouse, controllers, clipboard, AAudio and haptics.
7. Test and release: compatibility matrix, fuzzing, differential tests, thermal/frame-pacing tests and signed release APKs.

## Build

### Desktop UI preview

```sh
cargo run
```

### Android ARM64 APK

Install Rust stable and Android SDK / NDK, then:

```sh
rustup target add aarch64-linux-android
cargo install cargo-apk --version 0.10.0
cargo apk build --lib --target aarch64-linux-android
```

Debug APK output is produced under `target/debug/apk/`. A signed production release needs a separately managed release keystore; do not commit that key or its password to Git.

## Copyright and trademarks

TSI Orbit is an independent implementation. It does not contain Horizon source code or Horizon assets, and it does not bundle Minecraft code, sounds, textures, game binaries or account tokens. The UI is drawn with original vector primitives and text.

Minecraft is a trademark of Mojang Synergies AB. TSI Orbit is not an official Minecraft product and is not approved by or associated with Mojang or Microsoft. Users must supply a legally obtained game entitlement and only use game resources distributed through authorised channels.

Third-party Rust crates remain under their own licences; see the dependency licence metadata generated by Cargo. The TSI Orbit code in this repository is licensed under MIT.
