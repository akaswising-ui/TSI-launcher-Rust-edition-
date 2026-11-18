# Capability matrix

This matrix distinguishes tested source/build work from features supplied by the pinned game engine.

| Area | Current implementation | Limitations / validation still required |
|---|---|---|
| Original TSI launcher UI | Rust eframe/egui shell; responsive navigation; local preferences | Device UX and accessibility testing |
| TSI GPU diagnostics | Surface-compatible wgpu adapter/device selection; Vulkan-first then GLES; vendor/driver/Mali reporting | Physical device driver verification |
| Pinned game engine | Actual MIT Horizon Android Rust game-client port pinned at `1788803beebf105abe58de72752d08a5f0e0ee76` | Supports the pinned Java Edition 26.3 snapshot; not Bedrock or a general version switcher |
| Game renderer | Pinned client renderer with its Auto Vulkan → GLES fallback | Per-device testing, frame pacing and driver workarounds |
| Android Vulkan install requirement | Manifest advertises Vulkan 1.0 as optional | Devices with broken Vulkan and GLES drivers still cannot render |
| Mali identification | Vendor ID and adapter/driver string detection in TSI launcher renderer | Mali names/driver behaviour vary by SoC/OEM; no one-size-fits-all profile |
| Mali GPU model / architecture | Recognizes reported Midgard T-series, Bifrost G31/G51/G52/G71/G72/G76, Valhall G57/G68/G77/G78/G310/G510/G610/G615/G710/G715 and 5th Gen G620/G625/G720/G725 labels; reports Bifrost/Valhall/Midgard/5th Gen plus shader/downlevel/limits/compression support | OEM drivers may hide exact model; model label comes from reported adapter/driver string, physical testing still required |
| Mali-G52 game preset | Opt-in native Settings action writes the real game options: Performance preset (8 render / 6 simulation chunks), 75% world scale, 60 FPS, VSync and Graphics API Auto | Applies on next game launch; no FPS guarantee; Auto tries Vulkan and falls back to GLES when initialization fails |
| Touch controls | Pinned game has touch overlay, layouts and settings import/export | Verify across screen sizes and Android versions |
| Gamepad/controller | Included in pinned Android client | Physical Bluetooth/USB controller test matrix |
| Worlds/saves | Included in pinned game client | Storage migration/backup tests on real devices |
| Multiplayer | Included in pinned Java client | Server compatibility and LAN/multicast tests |
| Microsoft account | Pinned OAuth integration; TSI patch removes the port author's default client ID | Configure an approved application client ID; sign-in validation required |
| Resource packs | Game client pack loading and pack UI | Test pack formats and large pack memory use |
| Native mods | Engine's native mod mechanism | Do not assume Java .jar/Fabric/Forge compatibility |
| Downloads/preparation | Pinned Android installer downloads/validates player-entitled Java game resources from Mojang and prepares resources | Requires valid entitlement, network and first-run storage |
| Android OS bridge | Small Java NativeActivity bridge for soft keyboard, clipboard, file picker and sensors; Rust core | Not a strictly Java-free application |
| Launcher-to-game launch | TSI Orbit (`com.tsi.orbit`) starts installed Horizon game companion (`com.tsi.orbit.game`) with Android PackageManager Intent | Both APKs must be installed; physical Android launch test still required |
| APK/CI | Separate TSI launcher and playable Horizon client ARM64 APKs built and uploaded by GitHub Actions | Hosted CI does not prove phone GPU support or first-run entitlement flow |

The pinned Android engine is a Java Edition client, not Bedrock. Its included OpenJDK archive is used by the resource-preparation tools and does not execute the game client. Engine and third-party license notices remain with the APK.
