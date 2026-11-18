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
| Renderer | egui/wgpu UI renderer | Game renderer integration, Vulkan capability/fallback and frame pacing |
| Audio/haptics/clipboard | Preference/UI surfaces only | Android Rust platform bridges and tests |
| Game engine | Not included | Separate engine decision, legal source review and independent implementation/integration |

The user interface does not imply game-engine or backend parity. Nothing in this prototype downloads Minecraft binaries or includes Minecraft assets.
