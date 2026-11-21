# Spatial GUI for Horizon

Native Rust port of [tastytrash/Spatial-GUI](https://github.com/tastytrash/Spatial-GUI), distributed in Horizon's new **`.hmod`** source format and as a **precompiled Windows x64 game**. Java, a JVM, and Python are not required to play.

Release **0.2.5** targets Horizon **`nightly-2026-11-20-0a875da`**, revision `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, Minecraft **26.3**. It preserves the 0.2.4 feature set while migrating the project and archive to the [unified mod format](https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/introduction.md).

## Install 0.2.5 on Windows x64

Download **`spatial_gui-0.2.5-windows-x64.zip`** from [the release](https://github.com/hhhhsffd/Spatial-GUI-Horizon/releases/tag/v0.2.5), extract it, and use **Versions > Import Build** to import `horizon-client.exe` on `nightly-2026-11-20-0a875da`. Prepare that nightly's resources through the launcher first. See [installation instructions](docs/INSTALL-PREBUILT.md).

**The `.hmod` passes Horizon's package validator, but the new launcher's native Build cannot build this mod on the pinned nightly.** Its function-replacement route restricts support modules to `horizon_client`; Spatial GUI needs the renderer's private GPU pipelines in `horizon_client_render`. The Windows import build and full-source developer build retain those integrations. This release does not claim launcher-native compatibility. See [the migration and exact blocker](docs/HMOD-MIGRATION.md).

The Windows download is an unsigned, independently modified Horizon game containing Spatial GUI. It includes licenses and a complete corresponding-source archive with locked dependency sources and offline rebuild/relink instructions. Horizon is LGPL-3.0-or-later; Spatial GUI's code remains MIT. No Minecraft code or assets are distributed.

## Unified source package

[**`spatial_gui-0.2.5.hmod`**](dist/spatial_gui-0.2.5.hmod) is a flat ZIP with root `horizon-mod.toml`, `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/native/`, README, license and notices. Mod metadata and exact game/tool pins live in `horizon-mod.toml`; support modules use `[[native.module]]` and source edits use `[[native.mixin]]`. There is no `horizon-native.json`, bundled `horizon-mixin.toml`, or `mixins/<id>/` wrapper.

The root crate exposes the native hook factory. The world-space implementation remains in native-route support modules, preserving the renderer, input, editor and profile behavior. A fresh checkout fetches the pinned SDK with any `cargo horizon-mod` command before compiling the root crate.

## Features

- Actual Horizon menus rendered to a transparent texture on a world-space flat or curved panel, with perspective-correct sampling, world depth occlusion, player overlap opacity, filtering, lighting tint, shadows, render resolution, and optional world blur.
- Anchored first-person and third-person cameras, automatic perspective selection, offsets, parallax, mirroring, collision fallback, FOV scaling, elytra offsets, and opening/closing transitions.
- Ray-based input for slots, dragging, tooltips, scrollbars, text fields, and recipe panels. First-person crosshair interaction supports relative mouse look and a free-cursor toggle. Controller-generated logical navigation retains its original menu coordinates and gets a projected cursor.
- Fade and scale animations with all seven upstream easing modes, optional hand poke and arm swing, and local avatar body/head posing.
- Mouse-operated visual editor with translation/rotation/scale gizmos, sliders, orbit, preview, GUI scale, undo, redo, undo all, and immediate layout persistence.
- Named profiles with create, rename, delete, default linking, and assignment to a screen, category, or all screens. All 100 configuration fields from the original port's upstream reference and the upstream JSON5 profile format are supported.

Inventory, creative, and container menus are enabled by default. Pause presentation is optional. Other in-world screens can be enabled using the screen lists or `allScreens`, including books, signs, chat, options, dialogs, and native mod screens. Title and loading screens use ordinary rendering because there is no player/world anchor.

## Controls

Open **Options > Mods**, select **Spatial GUI**, then choose **Config** for categorized settings. Rebind its keys under **Options > Controls > Key Binds**, in the Spatial GUI category.

Hover a setting or the category selector to adjust it:

- **Left-click / scroll up:** increase the value or select the next option.
- **Right-click / scroll down:** decrease the value or select the previous option.
- Boolean settings toggle in either direction. Numeric values and choices wrap at their limits.

Changes save immediately. Scroll over a gap, key-binding row, or action button to scroll the settings list. Key rebinding still accepts mouse buttons.

| Key | Action |
| --- | --- |
| F8 | Open/save and close the visual editor |
| Backtick | Open the profile picker |
| F7 | Open the Mods page (select Spatial GUI > Config) |
| F6 | Reload configuration and profiles |
| Tab | Toggle free cursor during first-person crosshair interaction |
| G / R / S | Editor translation / rotation / scale tool |
| V / C | Editor camera mode / curvature |
| Space (hold) | Preview the interactive screen |
| Ctrl+Z / Ctrl+Y | Undo / redo |
| Arrows | Move the screen |
| + / - | Resize the screen |
| Page Up / Page Down | Adjust screen distance |

Drag a gizmo axis/ring or a slider with the left mouse button. Middle drag moves the panel, right drag orbits the editor camera, and the wheel changes scale. The editor buttons expose undo all, default linking, profile management, and save/close. Escape leaves the current editor/picker. Profile dialogs accept text; Shift+Enter adds a line to a screen list.

## Configuration and profiles

Files live in the instance's standard Horizon `config` directory:

- `spatial-gui.json5`: all settings, using the original camelCase field names.
- `spatial-gui/profiles.json5`: named overrides and rules, compatible with upstream's file format.

Copy existing Java mod configuration into those locations, then reload with F6. Invalid field values are bounded or replaced with defaults. Comments, single quotes, unquoted keys, trailing commas, hexadecimal numbers, and escaped Unicode are accepted. Saving writes formatted JSON, which is valid JSON5.

- `renderPlayerInFront` (default `true`): keep the third-person avatar in front of the panel.
- `screenOverPlayerAlpha` (0â€“255, default `50`): panel opacity over the avatar, or over visible first-person hands. Set to `0` for no panel over the player; `255` for full panel opacity.
- `curvedScreenEnabled` and `curvedScreenArcDegrees`: curvature works in both perspectives, from âˆ’180Â° to 180Â° (negative values bend backward).

Profile rules resolve in this order: exact native screen key, original Java screen class alias, category (`#inventory`, `#containers`, `#pause`, `#other`), then `#all`. Native menu keys include `inventory:minecraft:generic_9x3`; native mod screens use `mod:<owner>`. The `seenScreens` setting records encountered aliases. `disabledScreens` wins over enable rules. A profile's `guiScale` implies `overrideGuiScale` unless explicitly overridden. The linked default follows the global configuration.

## Native compatibility

This is an independent Rust implementation of the mod's behavior. Java-specific mixins for EMI, JEI, REI, Punchy, and other Java renderers cannot execute in Horizon. Their equivalents use Horizon's own recipe panel, native screen, camera, input, and hand hooks. Compatibility with another native mod that replaces those same systems depends on that mod's integration. The bundle pins the full Horizon revision because it uses private renderer integration.

## Build and verify

Development requires Python 3.11+, Rust 1.99.1, Git, and a C linker. On Windows GNU, put MinGW on PATH. Install the pinned `cargo-horizon-mod` tool; any command fetches `.horizon/sdk/` (ignored by Git and packaging).

```powershell
cargo horizon-mod hooks screen
python tools/port.py test
python tools/port.py package
cargo horizon-mod check dist/spatial_gui-0.2.5.hmod --json
```

The package command produces a deterministic `.hmod` and SHA-256 sidecar. `cargo horizon-mod build` can also repack an extracted `.hmod` source tree. Use the helper in this development checkout, which also contains test/tool Rust files outside `src/`. Packaging validation does not mean the launcher can compile the renderer declarations.

To compile the complete mod, use the full-source developer build:

```powershell
python tools/port.py build --horizon-source PATH_TO_NOVEMBER_20_SOURCE --mixins PATH_TO_PINNED_MIXIN_TOOL --target-dir target/horizon-cargo
```

The helper reads the unified manifest, derives a legacy source-transform manifest **only under `target/`**, materializes Horizon's generated modules in a disposable copy, and handles its moved creature paths. It then applies all 52 injections and 11 support modules and compiles `horizon-official`. The downloaded Horizon tree is preserved. This is a full game compilation, separate from the new launcher's function-replacement route.

`tools/wire.py` regenerates the unified manifest and typed helpers when hook definitions change. See [PORTING_STATUS.md](PORTING_STATUS.md) for coverage and [VALIDATION.md](VALIDATION.md) for checks actually performed. Non-Windows platforms need their own source build and are not runtime-tested for 0.2.5.

## Credits

Original mod: **tastytrash and Spatial-GUI contributors**, original port reference `88eeb54a262eff78ac2857995d8d9d1dd533dd22`; rendering fixes reviewed through `abfe5ae9c46dac0fdcfa09f46c89598db791eead`. New upstream features such as world-effect filters and the separate screen-manager UI are not included in this format migration release. MIT; the original license is retained in [LICENSE](LICENSE) and in the bundle. No Minecraft game code or assets are distributed.
