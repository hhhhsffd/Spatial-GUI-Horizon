# Changelog

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
