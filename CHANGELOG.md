# Changelog

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
