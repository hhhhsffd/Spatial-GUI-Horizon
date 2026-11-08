# Validation of Spatial GUI 0.2.2

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
