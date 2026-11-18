# Capability matrix

This matrix distinguishes tested source/build work from features supplied by the pinned game engine.

| Area | Current implementation | Limitations / validation still required |
|---|---|---|
| Original TSI launcher UI | Rust eframe/egui shell; responsive navigation; local preferences | Device UX and accessibility testing |
| TSI GPU diagnostics | Surface-compatible wgpu adapter/device selection; Vulkan-first then GLES; vendor/driver/Mali reporting | Physical device driver verification |
| Pinned game engine | MIT Android Rust Java Edition client pinned at `1788803beebf105abe58de72752d08a5f0e0ee76` | One game version: Java Edition 26.3; no Bedrock |
| Game renderer | Pinned client renderer with its Auto Vulkan → GLES fallback | Per-device testing, frame pacing and driver workarounds |
| Android Vulkan install requirement | Manifest advertises Vulkan 1.0 as optional | Devices with broken Vulkan and GLES drivers still cannot render |
| Mali identification | Vendor ID and adapter/driver string detection in TSI launcher renderer | Mali names/driver behaviour vary by SoC/OEM; no one-size-fits-all profile |
| Touch controls | Pinned game has touch overlay, layouts and settings import/export | Verify across screen sizes and Android versions |
| Gamepad/controller | Included in pinned Android client | Physical Bluetooth/USB controller test matrix |
| Worlds/saves | Included in pinned game client | Storage migration/backup tests on real devices |
| Multiplayer | Included in pinned Java client | Server compatibility and LAN/multicast tests |
| Microsoft account | Pinned OAuth integration; TSI patch removes the port author's default client ID | Configure an approved application client ID; sign-in validation required |
| Resource packs | Game client pack loading and pack UI | Test pack formats and large pack memory use |
| Native mods | Engine's native mod mechanism | Do not assume Java .jar/Fabric/Forge compatibility |
| Downloads/preparation | Pinned Android installer downloads/validates player-entitled Java game resources from Mojang and prepares resources | Requires valid entitlement, network and first-run storage |
| Android OS bridge | Small Java NativeActivity bridge for soft keyboard, clipboard, file picker and sensors; Rust core | Not a strictly Java-free application |
| APK/CI | UI preview builds in GitHub Actions | Playable engine APK needs separate engine build job; hosted CI does not prove phone GPU support |

The pinned Android engine is a Java Edition client, not Bedrock. Its included OpenJDK archive is used by the resource-preparation tools and does not execute the game client. Engine and third-party license notices remain with the APK.
