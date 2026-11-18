# TSI Orbit + TSI Rust Edition

TSI Orbit is the Rust launcher shell. **TSI Rust Edition** is a separate, native Rust 3D voxel sandbox game built for Android ARM64. The active game build no longer pulls or packages a Java Edition client, a JVM, or Minecraft game files.

This is an original TSI game project. It is not an official Minecraft edition, is not a Minecraft Java client, and does not claim compatibility with Minecraft worlds, servers, resource packs, or Java mods.

## Playable in v0.1.0

- First-person 3D camera and movement.
- Procedurally generated block terrain and simple trees.
- Jumping, sprinting, surface mining, and placing blocks.
- Five selectable block types in a hotbar.
- On-screen Android movement/action controls.
- Keyboard and mouse controls on desktop.
- Native Rust game logic and rendering through Macroquad.

Not implemented in this first playable slice: persistent world saves, multiplayer, mobs, crafting, inventory progression, mod loading, and a Vulkan game renderer.

## Graphics and devices

- TSI Orbit's launcher UI has its own wgpu GPU probing and Vulkan-first / OpenGL ES fallback.
- The standalone TSI Rust Edition game currently renders through Macroquad's Android OpenGL ES path. The launcher's Vulkan diagnostics are not a Vulkan renderer inside the game.
- Mali devices are targeted through the Android OpenGL ES driver path. Real phone testing is still required for frame pacing, thermals, touch latency and OEM driver behavior.

## Build the launcher APK

Prerequisites: Rust stable, Android SDK/NDK, cargo-apk, and JDK 17.

```sh
rustup target add aarch64-linux-android
cargo install cargo-apk --version 0.10.0
cargo apk build --lib --target aarch64-linux-android
```

## Build the playable Rust game APK

Prerequisites: Rust stable, Android SDK/NDK, JDK 17, and the Android ARM64 Rust target.

```sh
rustup target add aarch64-linux-android
cargo install cargo-quad-apk --locked
cd rust-game
cargo quad-apk build --release
```

The game APK is emitted under rust-game/target/android-artifacts/release/apk/. GitHub Actions uploads it as TSI-Rust-Edition-ARM64.

## Repository layout

- src/: TSI Orbit launcher UI and GPU diagnostics.
- rust-game/src/main.rs: native Rust 3D game loop, renderer, camera, controls and block interaction.
- rust-game/src/world.rs: deterministic terrain generation and block editing logic with unit tests.
- docs/FEATURE_MATRIX.md: implemented versus not-yet-implemented features.

TSI code is MIT-licensed under LICENSE. Macroquad and its transitive dependencies retain their own licences. No Minecraft binaries, assets, or game data are bundled. TSI projects are independent and are not affiliated with Mojang or Microsoft.
