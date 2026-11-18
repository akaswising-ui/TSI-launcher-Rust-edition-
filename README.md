# TSI Launcher Rust Edition

**TSI Orbit** is an independent Android launcher and game-client project. The UI uses an original graphite/teal design system, original vector artwork and TSI branding. It does not copy Horizon branding, screenshots or exact launcher layout.

## Android game client

The repository pins the MIT-licensed Android Rust game-client port in `vendor/horizon-android` at commit `1788803beebf105abe58de72752d08a5f0e0ee76`. This is a specific Android port snapshot, not the newer upstream Horizon source tree and not a promise of every upstream feature.

The pinned engine implements **Minecraft: Java Edition 26.3** as a native Rust game client. It is not Bedrock Edition and does not provide an arbitrary multi-version selector. Its Android port contains a small Java `NativeActivity` bridge for Android-only facilities such as the IME/soft keyboard, clipboard, document picker and sensors. The game logic, game renderer, launcher drawing, input handling and most application code are Rust. Therefore this Android application is not accurately described as 100% Rust source.

The port includes a Java runtime archive used by its first-run resource preparation pipeline; it is not a Java VM used to run the game client. The actual playable client is Rust.

## Mali / Vulkan / OpenGL ES

Two renderers are involved:

1. The TSI Orbit launcher UI renderer probes a real surface-compatible wgpu device, tries Vulkan first and tries OpenGL ES if Vulkan surface, adapter or device initialization fails. It reports the selected GPU, vendor, driver/backend and Mali-family detection in launcher Settings.
2. The native game renderer has its own graphics API selection. Its Auto mode tries Vulkan first and OpenGL ES second on Android.

The Android manifest does not require a Vulkan 1.1 device. Vulkan is optional and advertised at a 1.0 baseline so that devices with older Vulkan support can still install and attempt the OpenGL ES fallback. A driver must still be able to create a working Android surface and wgpu device. A hosted CI build cannot certify a particular Mali driver; physical device testing is required.

GPU support is not determined by the word “Mali” alone. The runtime uses adapter/vendor and driver information. The same GPU family can behave differently across Android vendor driver versions. If the Vulkan path fails, the app should continue on GLES when that driver supports it.

## Features in the pinned Android client

The pinned Android client includes the game renderer, worlds and saves, touch controls and layouts, controller/gamepad input, multiplayer/server code, Microsoft account integration, resource packs and the engine's native mod path. The Android launcher UI exposes game preparation, sign-in, update, touch layout import/export and settings. TSI's original launcher shell also exposes its own profiles, version/workspace pages, downloads and diagnostics.

Feature availability follows the pinned engine snapshot. Do not assume Java `.jar` mod compatibility, Fabric/Forge/NeoForge compatibility, Bedrock support, or all historic Minecraft versions unless separately tested and implemented.

## Copyright and licence

- TSI-owned code is MIT-licensed under `LICENSE`.
- The Android game client is pinned to the MIT-licensed port snapshot noted above. Its own license and third-party notices are preserved.
- Third-party components retain their original licences. See `THIRD_PARTY_NOTICES.md` and `vendor/horizon-android/THIRD_PARTY_NOTICES.md`.
- No Minecraft game binaries, textures, sounds or account entitlements are redistributed by this repository.

Minecraft is a trademark of Mojang Synergies AB. TSI Orbit is independent and is not an official Minecraft product or approved by or associated with Mojang or Microsoft.

## Build the playable Android ARM64 APK

Prerequisites:
- Rust stable and the `aarch64-linux-android` target.
- Android SDK and NDK.
- JDK 17.
- `python3`, `xz`, `zip`, `unzip`, `curl` and standard build tools.

Clone the repository including the pinned game engine:

```sh
git clone --recurse-submodules https://github.com/akaswising-ui/TSI-launcher-Rust-edition-.git
cd TSI-launcher-Rust-edition-
rustup target add aarch64-linux-android
bash scripts/build-android.sh
```

The debug-signed APK is written to `build/TSI-Orbit-Rust-ARM64.apk`. To use a different Rust build profile, set `HORIZON_ANDROID_PROFILE` before building. Production signing should use a separately managed keystore; never commit the keystore or its password.

Microsoft sign-in requires a Microsoft identity app registration configured for the Minecraft/Mojang authorization flow. The player/developer supplies their own approved client ID in the launcher's account settings. The project does not reuse the port author's OAuth application ID.

## UI preview

The separate eframe UI preview can be run on a desktop with:

```sh
cargo run
```
