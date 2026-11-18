# Capability matrix

This table describes code that exists in the TSI repository, not planned buttons or upstream features.

| Area | Current implementation | Limitations / validation still required |
|---|---|---|
| TSI Orbit launcher | Rust eframe/egui app; responsive workspaces; local profile and preference storage | Physical device UI and accessibility testing |
| Launcher GPU diagnostics | wgpu adapter/device selection; Vulkan-first then OpenGL ES fallback; vendor/driver/Mali reporting | Describes the launcher UI renderer only |
| TSI Rust Edition game | Separate native Rust Android ARM64 game APK using Macroquad | First playable vertical slice, not a finished survival game |
| 3D game renderer | Macroquad 3D camera and cube rendering on Android OpenGL ES | Vulkan game renderer is not implemented; test OEM GLES drivers |
| World generation | Deterministic 40×40 block terrain and simple trees | World currently lives in memory; no persistent save/load |
| Player | First-person camera, horizontal movement, stepping, gravity, jump and sprint | No advanced collision mesh, swimming, fall damage or animations |
| Block interaction | Surface raycast, mine/place blocks, five hotbar block types | No item drops, tool durability, crafting or block inventory counts |
| Android input | On-screen movement pad, look drag, jump, mine and place buttons | Touch latency/layout needs physical phone testing |
| Desktop input | WASD, mouse look/click interaction, Space jump, number-key hotbar | Desktop is for development/testing |
| Worlds workspace | Launcher workspace preserved | Save browser/import/export is not connected |
| Servers | Address book preserved | Rust game multiplayer/network protocol is not implemented |
| Native mods | Local .hmod URL list preserved | Download, signatures, sandbox and mod execution are not implemented |
| Resource packs | Local catalogue list preserved | Runtime asset-pack import/validation is not implemented |
| Accounts | Offline local profiles preserved | No account service is required for the current offline game slice |
| Java Edition client | Removed from active build and repository submodule configuration | No Java runtime, JVM, Java .jar compatibility or Minecraft server compatibility |

## Release validation gates

- Launcher: formatting, desktop check, tests and Clippy.
- Game logic: formatting, host compile and terrain/block-editing tests.
- Android game: separate ARM64 APK build, package identity check and native library presence check.
- Physical test: install and play on a real Mali device; CI alone cannot certify touch behavior or GPU stability.
