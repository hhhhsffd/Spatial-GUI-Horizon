# Spatial GUI for Horizon

Native Rust port of [tastytrash/Spatial-GUI](https://github.com/tastytrash/Spatial-GUI), distributed as a **`.hnmod`** source bundle and an optional **precompiled Windows x64 game**. Java, Fabric, a JVM, and Python are not required to play.

Release **0.2.4** targets Horizon **`nightly-2026-11-18-0b14f8a`**, the latest published game nightly checked on 2026-11-18, Minecraft data version **26.3**. Private renderer and input integrations require Horizon revision `0b14f8acfd97fa2c750e42795249dcc981178ca4` and horizon-mixins revision `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.

## Install 0.2.4 on Windows x64

Download **`spatial_gui-0.2.4-windows-x64.zip`** from [the release](https://github.com/hhhhsffd/Spatial-GUI-Horizon/releases/tag/v0.2.4), extract it, and use the launcher's **Versions > Import Build** to import `horizon-client.exe` on `nightly-2026-11-18-0b14f8a`. Prepare that nightly's resources through the launcher first; older prepared resources lack new required tables.

This is an **unsigned, independently modified Horizon game**, with only Spatial GUI compiled in. It runs with your account's permissions. Import only code you trust. See [installation instructions](docs/INSTALL-PREBUILT.md), including PowerShell commands.

The compiled build bypasses a **launcher source-staging bug**: the pinned mixin tool rejects Horizon's own `../../creatures/biome_facts.rs` module path with `E_INVALID_PATH`. The `.hnmod` alone still cannot be built through an unmodified launcher on this nightly. Neither your launcher nor its source caches are patched. The executable includes a documented level-zero legacy import marker so published launchers recognize it.

## Source builds and older installations

The release also provides [the 0.2.4 `.hnmod`](dist/spatial_gui-0.2.4.hnmod) and a **complete corresponding-source archive**, containing the transformed Horizon tree, locked dependency sources and licenses, and [offline rebuild/relink instructions](docs/REBUILD.md). The game is LGPL-3.0-or-later; Spatial GUI's own code remains MIT. No Minecraft game code or assets are distributed.

To build the mod from the original pinned source, the developer helper prepares a disposable copy, materializes Horizon's generated modules and inlines its moved creature modules before applying mixins:

```powershell
python tools/port.py build --horizon-source PATH_TO_NOVEMBER_18_SOURCE --mixins PATH_TO_PINNED_MIXIN_TOOL --target-dir target/horizon-cargo
```

Import `target/horizon-cargo/fast/horizon-official.exe` with a launcher that supports Horizon's new executable identity, or use the provided corresponding-source helper to produce a legacy-import-compatible executable. Non-Windows platforms require their own source build and are not runtime-tested for 0.2.4.

[Spatial GUI 0.2.3](https://github.com/hhhhsffd/Spatial-GUI-Horizon/releases/tag/v0.2.3) remains available for its older `nightly-2026-11-13-9f42064` pin and its normal launcher-native build path. Multiplayer servers and other players do not need the mod.

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
- `screenOverPlayerAlpha` (0–255, default `50`): panel opacity over the avatar, or over visible first-person hands. Set to `0` for no panel over the player; `255` for full panel opacity.
- `curvedScreenEnabled` and `curvedScreenArcDegrees`: curvature works in both perspectives, from −180° to 180° (negative values bend backward).

Profile rules resolve in this order: exact native screen key, original Java screen class alias, category (`#inventory`, `#containers`, `#pause`, `#other`), then `#all`. Native menu keys include `inventory:minecraft:generic_9x3`; native mod screens use `mod:<owner>`. The `seenScreens` setting records encountered aliases. `disabledScreens` wins over enable rules. A profile's `guiScale` implies `overrideGuiScale` unless explicitly overridden. The linked default follows the global configuration.

## Native compatibility

This is an independent Rust implementation of the mod's behavior. Java-specific mixins for EMI, JEI, REI, Punchy, and other Java renderers cannot execute in Horizon. Their equivalents use Horizon's own recipe panel, native screen, camera, input, and hand hooks. Compatibility with another native mod that replaces those same systems depends on that mod's integration. The bundle pins the full Horizon revision because it uses private renderer integration.

## Build and verify

Development requires Python 3.11+, Rust 1.99.1, Git, and a C linker. On Windows GNU, put MinGW on PATH. The helper adds Rust's bundled GNU libraries to `LIBRARY_PATH`.

```powershell
python tools/port.py test
python tools/port.py plan
python tools/port.py check
python tools/port.py build
python tools/port.py package
```

The helper discovers Horizon's installed source and mixin tool on Windows. On current revisions it first materializes generated module declarations in a disposable copy, with the developer staging workaround described above. Override them with `--horizon-source PATH` and `--mixins PATH`; use `--target-dir PATH` for a shared Cargo cache. All transformations happen in `target/`, preserving the downloaded Horizon source. `tools/wire.py` regenerates the declarative manifest and typed adapters when hook definitions change. Packaging creates a deterministic ZIP and SHA-256 sidecar.

See [PORTING_STATUS.md](PORTING_STATUS.md) for implementation coverage and [VALIDATION.md](VALIDATION.md) for checks actually performed.

## Credits

Original mod: **tastytrash and Spatial-GUI contributors**, original port reference `88eeb54a262eff78ac2857995d8d9d1dd533dd22`; rendering fixes reviewed through `abfe5ae9c46dac0fdcfa09f46c89598db791eead`. New upstream features such as world-effect filters and the separate screen-manager UI are not included in this bug-fix release. MIT; the original license is retained in [LICENSE](LICENSE) and in the bundle. No Minecraft game code or assets are distributed.
