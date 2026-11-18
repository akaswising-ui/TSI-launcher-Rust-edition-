# Capability matrix

This matrix separates present functionality from the engine/platform work still required.

| Area | 0.1.0 status | Next implementation |
|---|---|---|
| Rust-native UI shell | Implemented | Accessibility, tablet layout and polish |
| Navigation workspaces | Implemented | Connect each workspace to backend services |
| Local player profiles | Local create/select and persistence | Production Microsoft OAuth and secure token storage |
| Server address book | Local add/remove and persistence | Protocol client, status ping and join |
| Touch configuration | Four schemes and saved sliders/toggles | Apply settings to live in-game input |
| Native mod catalogue | HTTPS .hmod URL queue only | Download, hash/signature verification, sandbox runtime and permissions |
| Resource packs | Local named entries only | Import, manifest validation, hashes and activation |
| Downloads | UI/status only | Resumable download queue, pinned hashes and retries |
| Worlds and backups | Workspace placeholder | Safe document-picker import/export and backup |
| UI renderer / Vulkan | wgpu adapter enumeration, surface compatibility check, real device request, Vulkan-first selection and OpenGL ES fallback | Physical-device testing across Mali, Adreno and PowerVR; game-specific swapchain, frame pacing and 3D renderer integration |
| Mali GPU diagnostics | Runtime detection by ARM vendor ID / GPU-driver strings; selected model, backend, driver, ASTC/ETC2/BC support and wgpu limits shown in Settings | Confirm on physical Mali devices and tune per-driver workarounds from tested results |
| Texture compression | Request ASTC / ETC2 / BC only when the selected adapter reports the feature | Connect verified pack transcoding / format selection to the game renderer |
| Audio/haptics/clipboard | Preference/UI surfaces only | Android Rust platform bridges and tests |
| Game engine | Not included | Separate engine decision, legal source review and independent implementation/integration |

The UI renderer's Vulkan selection does not imply a playable 3D game renderer. Physical Android Mali driver testing is not covered by hosted CI. The user interface does not imply game-engine or backend parity. Nothing in this prototype downloads Minecraft binaries or includes Minecraft assets.
