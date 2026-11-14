# Rust port implementation coverage

Original full-port reference: `88eeb54a262eff78ac2857995d8d9d1dd533dd22`. Rendering fixes reviewed through `abfe5ae9c46dac0fdcfa09f46c89598db791eead` (2026-11-14). The later world-effect filters and separate screen-manager UI are new features, not part of this bug-fix update.

The 0.2.3 runtime is entirely Rust, compiled into Horizon through declarative native source mixins. Python is confined to developer tooling. See VALIDATION.md for version-specific checks actually run.

| Area | Native implementation |
| --- | --- |
| Configuration | All 100 fields from the original reference, JSON5 import, categorized settings with scroll/forward/backward mouse controls, bounds and key rebinding |
| Renderer | Offscreen native GUI extraction, forward/backward curvature in both perspectives, world and local player/hand silhouettes, configurable overlap opacity, filtering, shadows, lightmap tint, independent render resolution and GUI scale, world-only blur and HDR HUD coverage |
| Camera | Anchored first/third person on the renderer's interpolated player position, player-relative opening/closing transitions, collision fallback, mirror/parallax, elytra offsets, FOV scaling and saved-perspective restoration |
| Input | Crosshair/free cursor, relative mouse look, inverse ray mapping, scrolling/dragging and logical controller navigation with projected cursor |
| Animation | Seven easing modes, fade/scale/open/close, hand poke/swing and local avatar body/head posing |
| Editor | Mouse gizmos and sliders, screen movement and camera orbit, wheel scaling, undo/redo/undo-all, preview, GUI scale, save/default |
| Profiles | Named create/rename/delete, screen/category/global rules, linked default, picker, JSON5 reload and upstream file import |
| Screens | Inventory, creative, containers, optional pause, other in-world screens, books/signs/dialogs, native mod screens and recipe panel |
| Native interoperability | Horizon's native camera/input/hand/screen hooks and shared HUD pipelines, pinned to the declared client revision |

Java integration classes (Fabric/Forge/NeoForge, EMI/JEI/REI/Punchy-specific mixins) are replaced by the corresponding Horizon native systems. They are not loaded or emulated. Title/loading screens have no world anchor. Hardware-controller, shader-pack and third-party native-mod combinations require separate runtime verification; they are not implied by the core tests.

## Configuration coverage

- `enabled` -> `Config::enabled`
- `editorPausesGame` -> `Config::editor_pauses_game`
- `autoDetectCameraMode` -> `Config::auto_detect_camera_mode`
- `viewMode` -> `Config::view_mode`
- `useCrosshairForFirstPerson` -> `Config::use_crosshair_for_first_person`
- `hideHud` -> `Config::hide_hud`
- `hideHotbar` -> `Config::hide_hotbar`
- `overrideGuiScale` -> `Config::override_gui_scale`
- `guiScale` -> `Config::gui_scale`
- `inventory` -> `Config::inventory`
- `containers` -> `Config::containers`
- `pauseScreen` -> `Config::pause_screen`
- `allScreens` -> `Config::all_screens`
- `enabledScreens` -> `Config::enabled_screens`
- `disabledScreens` -> `Config::disabled_screens`
- `seenScreens` -> `Config::seen_screens`
- `disableFirstPerson` -> `Config::disable_first_person`
- `disableThirdPerson` -> `Config::disable_third_person`
- `renderScalePercent` -> `Config::render_scale_percent`
- `recipeBookShrinkFactor` -> `Config::recipe_book_shrink_factor`
- `dropShadowEnabled` -> `Config::drop_shadow_enabled`
- `dropShadowLayers` -> `Config::drop_shadow_layers`
- `dropShadowAlpha` -> `Config::drop_shadow_alpha`
- `dropShadowScaleMultiplier` -> `Config::drop_shadow_scale_multiplier`
- `dropShadowHorizontalOffset` -> `Config::drop_shadow_horizontal_offset`
- `dropShadowVerticalOffset` -> `Config::drop_shadow_vertical_offset`
- `dropShadowDepthOffset` -> `Config::drop_shadow_depth_offset`
- `enableWorldLightingTint` -> `Config::enable_world_lighting_tint`
- `lightingTintStrength` -> `Config::lighting_tint_strength`
- `hideHandsInFirstPerson` -> `Config::hide_hands_in_first_person`
- `hideShieldInFirstPerson` -> `Config::hide_shield_in_first_person`
- `armSwing` -> `Config::arm_swing`
- `handPokeAnimation` -> `Config::hand_poke_animation`
- `handPokeStrength` -> `Config::hand_poke_strength`
- `handPokeDurationMs` -> `Config::hand_poke_duration_ms`
- `screenAlpha` -> `Config::screen_alpha`
- `renderPlayerInFront` -> `Config::render_player_in_front`
- `screenOverPlayerAlpha` -> `Config::screen_over_player_alpha`
- `blurWorldBackground` -> `Config::blur_world_background`
- `worldBlurStrength` -> `Config::world_blur_strength`
- `useLinearFiltering` -> `Config::use_linear_filtering`
- `useAnisotropicFiltering` -> `Config::use_anisotropic_filtering`
- `screenDistance` -> `Config::screen_distance`
- `screenSideOffset` -> `Config::screen_side_offset`
- `screenHeightOffset` -> `Config::screen_height_offset`
- `screenYawOffset` -> `Config::screen_yaw_offset`
- `screenPitchOffset` -> `Config::screen_pitch_offset`
- `screenScale` -> `Config::screen_scale`
- `recipeViewerScaleMultiplier` -> `Config::recipe_viewer_scale_multiplier`
- `elytraFlyingSideOffset` -> `Config::elytra_flying_side_offset`
- `elytraFlyingHeightOffset` -> `Config::elytra_flying_height_offset`
- `cameraDistance` -> `Config::camera_distance`
- `cameraSideOffset` -> `Config::camera_side_offset`
- `cameraHeightOffset` -> `Config::camera_height_offset`
- `cameraTargetPitch` -> `Config::camera_target_pitch`
- `lerpCameraPitch` -> `Config::lerp_camera_pitch`
- `disableThirdPersonParallax` -> `Config::disable_third_person_parallax`
- `fallbackToFirstPersonOnBlockCollision` -> `Config::fallback_to_first_person_on_block_collision`
- `mirrorThirdPerson` -> `Config::mirror_third_person`
- `mirrorHeadMovement` -> `Config::mirror_head_movement`
- `avatarBaseYawOffset` -> `Config::avatar_base_yaw_offset`
- `avatarBodyRotationOffset` -> `Config::avatar_body_rotation_offset`
- `avatarMaxYawOffset` -> `Config::avatar_max_yaw_offset`
- `avatarMaxPitchOffset` -> `Config::avatar_max_pitch_offset`
- `thirdPersonMouseSensitivityYaw` -> `Config::third_person_mouse_sensitivity_yaw`
- `thirdPersonMouseSensitivityPitch` -> `Config::third_person_mouse_sensitivity_pitch`
- `transitionDurationMs` -> `Config::transition_duration_ms`
- `transitionSkipPercentage` -> `Config::transition_skip_percentage`
- `curvedScreenEnabled` -> `Config::curved_screen_enabled`
- `curvedScreenArcDegrees` -> `Config::curved_screen_arc_degrees`
- `firstPersonScreenDistance` -> `Config::first_person_screen_distance`
- `firstPersonScreenSideOffset` -> `Config::first_person_screen_side_offset`
- `firstPersonScreenHeightOffset` -> `Config::first_person_screen_height_offset`
- `firstPersonScreenYawOffset` -> `Config::first_person_screen_yaw_offset`
- `firstPersonScreenPitchOffset` -> `Config::first_person_screen_pitch_offset`
- `firstPersonScreenScale` -> `Config::first_person_screen_scale`
- `keepFirstPersonCameraAngle` -> `Config::keep_first_person_camera_angle`
- `disableFirstPersonParallax` -> `Config::disable_first_person_parallax`
- `firstPersonMouseSensitivityYaw` -> `Config::first_person_mouse_sensitivity_yaw`
- `firstPersonMouseSensitivityPitch` -> `Config::first_person_mouse_sensitivity_pitch`
- `firstPersonPitchClamp` -> `Config::first_person_pitch_clamp`
- `lockFirstPersonYaw` -> `Config::lock_first_person_yaw`
- `enableCloseAnimation` -> `Config::enable_close_animation`
- `enableFadeAnimation` -> `Config::enable_fade_animation`
- `fadeDurationMs` -> `Config::fade_duration_ms`
- `enableScaleAnimation` -> `Config::enable_scale_animation`
- `openAnimationDurationMs` -> `Config::open_animation_duration_ms`
- `animationStartScalePercent` -> `Config::animation_start_scale_percent`
- `animationEasing` -> `Config::animation_easing`
- `overrideFov` -> `Config::override_fov`
- `targetFov` -> `Config::target_fov`
- `fovLerpSpeed` -> `Config::fov_lerp_speed`
- `autoScaleByFov` -> `Config::auto_scale_by_fov`
- `autoScaleBaselineFov` -> `Config::auto_scale_baseline_fov`
- `autoScaleScreenPower` -> `Config::auto_scale_screen_power`
- `autoScaleSideOffsetMultiplier` -> `Config::auto_scale_side_offset_multiplier`
- `autoScaleDistanceMultiplier` -> `Config::auto_scale_distance_multiplier`
- `autoScaleThirdPersonScreenMultiplier` -> `Config::auto_scale_third_person_screen_multiplier`
- `editorIntroSeen` -> `Config::editor_intro_seen`
- `profilePickerTipSeen` -> `Config::profile_picker_tip_seen`
