# Changelog

## 0.2.5

- Migrate to the flat `.hmod` format: one `horizon-mod.toml`, root Cargo crate and native hook factory, `src/` payloads, native mixin/module declarations, license and notices.
- Remove the legacy package wrapper and `horizon-native.json`; generate the older source-transform manifest only in disposable developer staging.
- Preserve all existing renderer/input/editor/profile behavior and the November 18 game/tool pins.
- Document the new launcher's renderer-module restriction: package validation passes, but native Build remains unsupported on this pin. Retain the Windows Import Build and complete-source rebuild paths.

## 0.2.4

- Target Horizon `nightly-2026-11-18-0b14f8a`, revision `0b14f8acfd97fa2c750e42795249dcc981178ca4`.
- Adapt mod settings and key bindings to Horizon's typed native settings pages while retaining forward clicks, reverse clicks, wheel controls, categories, and editor/profile actions.
- Move frame, key-release, hand, FOV, GUI-layer and renderer integration to the current hook modules and render slots.
- Replace removed global GUI scale state with a scoped override of explicit batch scale inputs; retain Unicode scaling and independent offscreen batches.
- Materialize generated game modules in the developer build helper. Include a disposable-source workaround for the pinned tool's refusal of Horizon's moved creature module paths.
- Restore editor/profile labels and tips by supplying the explicit alpha channel now required by Horizon's native text helper.
- Provide a precompiled Windows x64 game installable through Import Build, bypassing the source-staging bug without modifying the launcher or its caches. Include a legacy level-zero import marker for published launchers.
- Provide complete corresponding source, vendored dependencies, licenses and offline rebuild/relink instructions alongside the compiled game. Standard launcher building from the `.hnmod` alone remains blocked on this nightly.

## 0.2.3

- Fixed avatar wobble when opening menus while running or falling: camera and panel now follow the same interpolated player position as the rendered model.
- Opening and closing transition anchors move with the player instead of lagging at their old world positions.
- Updated compatibility to the latest published Horizon nightly, `nightly-2026-11-13-9f42064`, and horizon-mixins `e59da9094f0c`.
- Fixed configurable screen opacity over first-person hands and current Horizon's local-player geometry identification. `renderPlayerInFront` and `screenOverPlayerAlpha` remain configurable.
- Applied upstream backward/third-person curvature and scaled item/glint clipping fixes; kept native picture and glint state during GUI extraction.
- Added HDR HUD coverage for spatial panels and preserved the world's HDR alpha marks during blur.

## 0.2.2

- Added mouse-wheel changes to settings and categories: up increases/next, down decreases/previous.
- Added right-click to decrease values or choose the previous setting/category; left-click still moves forward.
- Fixed backward choice cycling and floating-point rounding near numeric limits.
- Preserved list scrolling outside adjustable buttons, clipping of hidden rows, and mouse key rebinding.

## 0.2.1

- Fixed the third-person avatar facing away from the GUI by removing an extra half-turn.
- Applied body and head offsets separately, including mirrored layouts, as in upstream.
- Kept head tracking independent of camera parallax and preserved flight head pitch.
- Updated the pinned Horizon version to `nightly-2026-11-05-66f3981`.

## 0.2.0

- Completed the native Rust world-space renderer, camera/input integration, editor, and upstream configuration/profile support.
