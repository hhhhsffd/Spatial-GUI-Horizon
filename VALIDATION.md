# Validation of Spatial GUI 0.2.3

Checked on 2026-11-14 against **Horizon `nightly-2026-11-13-9f42064`**, revision `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0` (latest published game nightly at the time), and horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.

## Build and automated checks

- `python tools/port.py test`: **17 tests passed**. The movement regression covers render partial ticks, running/falling motion, player-relative opening/closing transitions, large world coordinates, and yaw wrapping. Ray tests also cover backward curvature up to ±180°.
- Core/editor `cargo clippy --locked --all-targets -- -D warnings`: passed. Changed Rust files pass rustfmt.
- Declarative staging: **50 injections and 10 support modules**.
- Optimized `fast` build of the official game executable (`horizon-official`, the current launcher game entry point): passed. Source staging preserves the original downloaded Horizon source.
- Final compiler check of `horizon-official` against the packaged source: passed.
- **Three renderer tests passed**: two native settings hit-target/clipping tests and a GPU regression which renders the actual avatar/hand mask and panel shaders. The GPU test checks opacity 0/50/255, the player-in-front toggle, terrain occlusion, and both SDR and HDR coverage outputs.

## Runtime checks

The official game ran offscreen on an NVIDIA RTX 2080 (Vulkan), at 1280×720, with fresh test worlds and isolated game/launcher/config/home directories. Existing player data and the selected launcher version were not changed.

- Opened the creative GUI while sprinting (confirmed horizontal velocity about 0.15 blocks/tick) and while falling from Y=220. The player fell to the test platform at Y=121 with the GUI open. Captured the opening and successive falling frames; the camera, panel, and avatar remained aligned instead of separating at tick boundaries.
- Set `screenOverPlayerAlpha` to 0 and 255 with an overlapping third-person layout: the avatar remained unobstructed at 0, while the panel covered its overlapping pixels at 255. Disabling `renderPlayerInFront` restored full panel rendering over the avatar.
- Reloaded JSON5 settings during the session. First-person mode with −60° backward curvature, visible held sword, and screen-over-player opacity rendered successfully.
- Opened and closed the visual editor with the native key path; its arc slider displayed −60°. Closing the GUI returned to ordinary gameplay.
- The final runtime log contained no panic or GPU validation error, and the test world was saved and stopped.

Runtime captures and command/state logs remain in `target/runtime-0.2.3-bf2a8cda`. These checks do not establish compatibility with every native mod, resource/shader pack, hardware controller, or HDR display.

## Bundle verification

The 0.2.3 `.hnmod` contains **63 entries**, is **64,305 bytes**, and includes a SHA-256 sidecar. ZIP integrity, source equality, native metadata, and absence of Python/Java/JAR runtime files were checked. Packaging is deterministic.

---

## Historical validation of Spatial GUI 0.2.2

Targets Horizon `66f3981f522b24c250b21523a70080965b940424` (`nightly-2026-11-05-66f3981`).

- Added scroll-up/left-click forward and scroll-down/right-click backward settings controls.
- `python tools/port.py test`: **16 tests passed**, including backward choice cycling, numeric stepping/wrapping across every numeric setting, boolean toggles, and zero/nonfinite input.
- Two focused Horizon renderer tests passed, covering scrolled/clipped button hit targets, key-rebinding state, other mod pages, and scrollbar dragging.
- Declarative staging succeeded against the pinned revision: **50 injections and 9 support modules**.
- `python tools/port.py check --target-dir <launcher-native-target>`: passed against the pinned Horizon source with the bundled Rust 1.99.1 and MinGW toolchain.
- The `.hnmod` ZIP passed integrity checks, and every packaged Rust/manifest file matched the local source byte-for-byte. The archive contains **62 entries**, is **61,063 bytes**, and includes a SHA-256 sidecar.
- No game was launched or tested for this update.

---

## Historical validation of Spatial GUI 0.2.1

The 0.2.1 fix targets Horizon `66f3981f522b24c250b21523a70080965b940424` (`nightly-2026-11-05-66f3981`).

- Removed the unintended 180-degree avatar turn and separated body rotation from the relative head offset, following upstream's `AvatarRendererMixin`.
- Mirrored third-person layouts now mirror both configured pose offsets. Head look uses pointer position independently of camera parallax; mirroring head movement affects horizontal look only. Elytra flight retains its existing head pitch.
- `python tools/port.py test`: **13 tests passed**, including heading checks over multiple anchored rotations, mirrored body/head offsets, bounded pointer look, and elytra pitch preservation.

The 0.2.1 mixin stage applied successfully on the pinned revision. Client compilation was stopped at the user's request; no in-game testing was performed for this fix. The `.hnmod` source bundle was packaged and its ZIP integrity checked. The earlier full-port validation below describes 0.2.0 on the previous Horizon revision, rather than 0.2.1.

---

## Historical validation of Spatial GUI 0.2.0

Completed on 2026-11-05, after implementation of the full Rust runtime.

## Build and automated checks

- Horizon `6475f62f3c834b7489e1816bf47f47ea5eb2b879`, horizon-mixins `e8cd0aa38cffef49cb2e088ef716baa107148c02`, Rust 1.99.1, Windows GNU/MinGW.
- Declarative staging succeeded: **50 injections and 7 support modules**. The downloaded Horizon source was preserved; transformations ran in `target/horizon-stage`.
- Full optimized client build passed with `python tools/port.py build --target-dir <launcher-native-target>`, profile `fast`. The final compilation reported only the existing MinGW warnings from mimalloc's C source.
- `python tools/port.py test`: **10 tests passed**. Coverage includes all 100 serialized settings, JSON5 import/bounds/Unicode/nesting, profile precedence/default linking/rename/delete/round trips, seven easing modes, world-space ray mapping at large coordinates and multiple curvatures/rotations, parallel/backward rays, and editor movement/undo/redo/history branching.
- Core/editor `cargo clippy --locked --all-targets -- -D warnings`: passed.
- All four embedded WGSL shaders passed parsing and full validation using the pinned Horizon build's Naga library: world blur, world depth extraction, local avatar depth, and screen composition. The temporary validator and shader inputs are in `target/shader-validation`.

## Runtime checks

The native client ran on an NVIDIA RTX 2080, Vulkan, at 1280x720. All saves, options, configuration, logs, launcher paths, and control sockets were confined to `target/runtime-test`. No existing player world was modified. The client was saved and stopped when verification ended.

- Third-person inventory rendered as a textured world-space surface, with the local avatar posed beside it. World geometry occluded the panel. Screen opening and closing completed without GPU validation errors; closing restored the ordinary perspective.
- Window-coordinate `click-at` events used the inverse input mapping: picked up **16 diamonds from hotbar slot 0** on the tilted panel and placed all 16 in **inventory slot 9**. Total item count remained 16.
- Imported first-person settings with **60-degree curvature**, 75% render resolution, world blur, lighting tint, and HUD hiding rendered successfully. Window-coordinate input picked up the diamond stack from the curved panel.
- Native settings appeared under Options > Mods. Configuration reload applied JSON5 edits during the session.
- The visual editor rendered its controls and projected gizmos. Slider changes, undo, editor closing, and saved layout files worked. A profile was created and deleted through the picker; the linked default remained intact.
- Restarted with the final binary and existing test configuration/profile files. First-person mode and curvature persisted. F8 toggled the editor through Horizon's control-socket screen-key path; F6 reloaded settings. The free-cursor toggle and editor entry/exit executed with crosshair mode enabled.
- Disabling the configuration restored standard menu rendering; enabling it restored the spatial presentation. The final log contained no runtime panic or GPU validation error.

Screenshots: [editor](docs/editor.png), [curved first person](docs/curved-first-person.png), [third person](docs/third-person.png). Intermediate screenshots and the control log remain under `target/runtime-test`.

This pass does not establish compatibility with every shader pack or native mod. Hardware controller behavior, OS-level relative mouse capture, every screen/container variant, and every combination of camera/animation settings were not exhaustively exercised. Java-specific integration mixins are replaced by Horizon native systems, as described in README.md.

## Bundle verification

`dist/spatial_gui-0.2.0.hnmod` contains **60 entries**, including the native manifest, all current Rust sources/adapters, and the upstream MIT license. ZIP integrity and embedded version/pins were checked, and every packaged Rust file was compared byte-for-byte with the source. The bundle contains no Python, Java, class, or JAR runtime files.

Two independent packaging runs produced identical bytes:

```text
4d03ac470784aa2ab78ea4311c2aa39c509878a0eac637b70bf92fe11fe3580b  spatial_gui-0.2.0.hnmod
```

The archive is 58,004 bytes. A SHA-256 sidecar is included beside it.
