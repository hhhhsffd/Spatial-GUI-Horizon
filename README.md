# Spatial GUI for Horizon

Native Rust port of [tastytrash/Spatial-GUI](https://github.com/tastytrash/Spatial-GUI), distributed as a **`.hnmod`** source bundle. Horizon compiles the Rust into its client. Java, Fabric, a JVM, and Python are not required to run the mod.

Targets Horizon **`nightly-2026-11-05-66f3981`**, Minecraft data version **26.3**. Private renderer and input integrations require Horizon revision `66f3981f522b24c250b21523a70080965b940424` and horizon-mixins revision `e8cd0aa38cffef49cb2e088ef716baa107148c02`.

## Install

Download [Spatial GUI 0.2.1](https://github.com/hhhhsffd/Spatial-GUI-Horizon/releases/tag/v0.2.1), or use [dist/spatial_gui-0.2.1.hnmod](dist/spatial_gui-0.2.1.hnmod). Import it through a Horizon launcher with native mod support. It contains source, a manifest, and the upstream MIT license; the launcher builds the native client.

Alternatively, build directly from this folder:

```powershell
horizon-launcher native-build ./mixins/spatial_gui --instance "YOUR INSTANCE" --version nightly-2026-11-05-66f3981 --trust-native-code
```

Select the resulting native version for the instance if the launcher does not select it automatically. For an extracted archive, use its `mixins/spatial_gui` directory. Multiplayer servers and other players do not need the mod.

## Features

- Actual Horizon menus rendered to a transparent texture on a world-space flat or curved panel, with perspective-correct sampling, world depth occlusion, player overlap opacity, filtering, lighting tint, shadows, render resolution, and optional world blur.
- Anchored first-person and third-person cameras, automatic perspective selection, offsets, parallax, mirroring, collision fallback, FOV scaling, elytra offsets, and opening/closing transitions.
- Ray-based input for slots, dragging, tooltips, scrollbars, text fields, and recipe panels. First-person crosshair interaction supports relative mouse look and a free-cursor toggle. Controller-generated logical navigation retains its original menu coordinates and gets a projected cursor.
- Fade and scale animations with all seven upstream easing modes, optional hand poke and arm swing, and local avatar body/head posing.
- Mouse-operated visual editor with translation/rotation/scale gizmos, sliders, orbit, preview, GUI scale, undo, redo, undo all, and immediate layout persistence.
- Named profiles with create, rename, delete, default linking, and assignment to a screen, category, or all screens. All 100 upstream configuration fields and the upstream JSON5 profile format are supported.

Inventory, creative, and container menus are enabled by default. Pause presentation is optional. Other in-world screens can be enabled using the screen lists or `allScreens`, including books, signs, chat, options, dialogs, and native mod screens. Title and loading screens use ordinary rendering because there is no player/world anchor.

## Controls

Open **Options > Mods > Spatial GUI** for categorized settings and key rebinding.

| Key | Action |
| --- | --- |
| F8 | Open/save and close the visual editor |
| Backtick | Open the profile picker |
| F7 | Open mod configuration |
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

The helper discovers Horizon's installed source and mixin tool on Windows. Override them with `--horizon-source PATH` and `--mixins PATH`; use `--target-dir PATH` for a shared Cargo cache. All transformations happen in `target/`, preserving the downloaded Horizon source. `tools/wire.py` regenerates the declarative manifest and typed adapters when hook definitions change. Packaging creates a deterministic ZIP and SHA-256 sidecar.

See [PORTING_STATUS.md](PORTING_STATUS.md) for implementation coverage and [VALIDATION.md](VALIDATION.md) for checks actually performed.

## Credits

Original mod: **tastytrash and Spatial-GUI contributors**, reference `88eeb54a262eff78ac2857995d8d9d1dd533dd22`. MIT; the original license is retained in [LICENSE](LICENSE) and in the bundle. No Minecraft game code or assets are distributed.
