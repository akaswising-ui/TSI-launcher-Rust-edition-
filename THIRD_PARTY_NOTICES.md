# Third-party notices

## TSI Orbit

The original TSI Orbit launcher shell, GPU selection module, build integration and project documentation are licensed under the MIT License in the repository root `LICENSE`.

## Horizon Android Rust game client

The Horizon Android Rust Java Edition client is built from the MIT-licensed `MocLG/horizon-android` repository, pinned as Git submodule `vendor/horizon-android` at commit:

`1788803beebf105abe58de72752d08a5f0e0ee76`

The submodule's own `LICENSE` and `THIRD_PARTY_NOTICES.md` are preserved. The build process applies revision-guarded TSI branding, a separate Android package ID (`com.tsi.orbit.game`), and Android GPU compatibility patches. The TSI launcher app uses `com.tsi.orbit` and starts the companion with an Android Intent. The game engine remains the pinned upstream client; TSI does not replace it with a voxel clone.

## Dependencies and bundled content in the pinned engine

The pinned engine's `THIRD_PARTY_NOTICES.md` identifies, among other items:

- wgpu-hal: MIT or Apache-2.0.
- Pixelify Sans: SIL Open Font License 1.1.
- Pixelarticons: MIT.
- A default touch-control layout originating from Zalith Launcher 2: GNU GPL-3.0.
- OpenJDK 25 runtime for Android from Zalith Launcher 2: GNU GPL-2.0 with the Classpath Exception. It is used by the resource-preparation tooling bundled with that Android port.

The Android APK build copies the engine's license, its third-party notices, this TSI notice, and the TSI root license into APK assets. The dependency owners' copyright and license terms continue to apply; MIT on the Android port does not relicense its third-party dependencies.

## Minecraft and Mojang

Minecraft is a trademark of Mojang Synergies AB. TSI Orbit is an independent project and is not an official Minecraft product or approved by or associated with Mojang or Microsoft. The build does not redistribute Minecraft game binaries, textures, sounds or account entitlements. Players must use a valid Minecraft: Java Edition entitlement and authorised game files.
