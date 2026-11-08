// Complete Spatial-GUI configuration, preserving upstream JSON field names and defaults.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Config {
    pub enabled: bool,
    pub editor_pauses_game: bool,
    pub auto_detect_camera_mode: bool,
    pub view_mode: String,
    pub use_crosshair_for_first_person: bool,
    pub hide_hud: bool,
    pub hide_hotbar: bool,
    pub override_gui_scale: bool,
    pub gui_scale: i32,
    pub inventory: bool,
    pub containers: bool,
    pub pause_screen: bool,
    pub all_screens: bool,
    pub enabled_screens: Vec<String>,
    pub disabled_screens: Vec<String>,
    pub seen_screens: Vec<String>,
    pub disable_first_person: bool,
    pub disable_third_person: bool,
    pub render_scale_percent: i32,
    pub recipe_book_shrink_factor: f32,
    pub drop_shadow_enabled: bool,
    pub drop_shadow_layers: i32,
    pub drop_shadow_alpha: i32,
    pub drop_shadow_scale_multiplier: f32,
    pub drop_shadow_horizontal_offset: f32,
    pub drop_shadow_vertical_offset: f32,
    pub drop_shadow_depth_offset: f32,
    pub enable_world_lighting_tint: bool,
    pub lighting_tint_strength: f32,
    pub hide_hands_in_first_person: bool,
    pub hide_shield_in_first_person: bool,
    pub arm_swing: bool,
    pub hand_poke_animation: bool,
    pub hand_poke_strength: f32,
    pub hand_poke_duration_ms: i32,
    pub screen_alpha: i32,
    pub render_player_in_front: bool,
    pub screen_over_player_alpha: i32,
    pub blur_world_background: bool,
    pub world_blur_strength: i32,
    pub use_linear_filtering: bool,
    pub use_anisotropic_filtering: bool,
    pub screen_distance: f32,
    pub screen_side_offset: f32,
    pub screen_height_offset: f32,
    pub screen_yaw_offset: f32,
    pub screen_pitch_offset: f32,
    pub screen_scale: f32,
    pub recipe_viewer_scale_multiplier: f32,
    pub elytra_flying_side_offset: f32,
    pub elytra_flying_height_offset: f32,
    pub camera_distance: f32,
    pub camera_side_offset: f32,
    pub camera_height_offset: f32,
    pub camera_target_pitch: f32,
    pub lerp_camera_pitch: bool,
    pub disable_third_person_parallax: bool,
    pub fallback_to_first_person_on_block_collision: bool,
    pub mirror_third_person: bool,
    pub mirror_head_movement: bool,
    pub avatar_base_yaw_offset: f32,
    pub avatar_body_rotation_offset: f32,
    pub avatar_max_yaw_offset: i32,
    pub avatar_max_pitch_offset: i32,
    pub third_person_mouse_sensitivity_yaw: f32,
    pub third_person_mouse_sensitivity_pitch: f32,
    pub transition_duration_ms: i32,
    pub transition_skip_percentage: i32,
    pub curved_screen_enabled: bool,
    pub curved_screen_arc_degrees: i32,
    pub first_person_screen_distance: f32,
    pub first_person_screen_side_offset: f32,
    pub first_person_screen_height_offset: f32,
    pub first_person_screen_yaw_offset: f32,
    pub first_person_screen_pitch_offset: f32,
    pub first_person_screen_scale: f32,
    pub keep_first_person_camera_angle: bool,
    pub disable_first_person_parallax: bool,
    pub first_person_mouse_sensitivity_yaw: f32,
    pub first_person_mouse_sensitivity_pitch: f32,
    pub first_person_pitch_clamp: i32,
    pub lock_first_person_yaw: bool,
    pub enable_close_animation: bool,
    pub enable_fade_animation: bool,
    pub fade_duration_ms: i32,
    pub enable_scale_animation: bool,
    pub open_animation_duration_ms: i32,
    pub animation_start_scale_percent: i32,
    pub animation_easing: String,
    pub override_fov: bool,
    pub target_fov: i32,
    pub fov_lerp_speed: i32,
    pub auto_scale_by_fov: bool,
    pub auto_scale_baseline_fov: f32,
    pub auto_scale_screen_power: f32,
    pub auto_scale_side_offset_multiplier: f32,
    pub auto_scale_distance_multiplier: f32,
    pub auto_scale_third_person_screen_multiplier: f32,
    pub editor_intro_seen: bool,
    pub profile_picker_tip_seen: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            editor_pauses_game: false,
            auto_detect_camera_mode: false,
            view_mode: "THIRD_PERSON".into(),
            use_crosshair_for_first_person: true,
            hide_hud: false,
            hide_hotbar: false,
            override_gui_scale: false,
            gui_scale: 4,
            inventory: true,
            containers: true,
            pause_screen: false,
            all_screens: false,
            enabled_screens: vec![
                "net.minecraft.client.gui.screens.inventory.BookViewScreen".into(),
                "net.minecraft.client.gui.screens.inventory.BookEditScreen".into(),
                "net.minecraft.client.gui.screens.inventory.BookSignScreen".into(),
                "net.minecraft.client.gui.screens.inventory.CraftingScreen".into(),
                "net.minecraft.client.gui.screens.inventory.FurnaceScreen".into(),
                "net.minecraft.client.gui.screens.inventory.SmokerScreen".into(),
                "net.minecraft.client.gui.screens.inventory.BlastFurnaceScreen".into(),
                "net.minecraft.client.gui.screens.inventory.AnvilScreen".into(),
                "net.minecraft.client.gui.screens.inventory.EnchantmentScreen".into(),
                "net.minecraft.client.gui.screens.inventory.BeaconScreen".into(),
                "net.minecraft.client.gui.screens.inventory.BrewingStandScreen".into(),
                "net.minecraft.client.gui.screens.inventory.MerchantScreen".into(),
                "net.minecraft.client.gui.screens.inventory.ContainerScreen".into(),
                "net.minecraft.client.gui.screens.inventory.ShulkerBoxScreen".into(),
                "net.minecraft.client.gui.screens.inventory.HopperScreen".into(),
                "net.minecraft.client.gui.screens.inventory.DispenserScreen".into(),
                "net.minecraft.client.gui.screens.inventory.GrindstoneScreen".into(),
                "net.minecraft.client.gui.screens.inventory.SmithingScreen".into(),
                "net.minecraft.client.gui.screens.inventory.CartographyTableScreen".into(),
                "net.minecraft.client.gui.screens.inventory.LoomScreen".into(),
                "net.minecraft.client.gui.screens.inventory.StonecutterScreen".into(),
                "net.minecraft.client.gui.screens.inventory.LecternScreen".into(),
                "net.minecraft.client.gui.screens.inventory.InventoryScreen".into(),
                "net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen".into(),
                "dev.emi.emi.screen.RecipeScreen".into(),
                "mezz.jei.gui.recipes.RecipesGui".into(),
                "me.shedaniel.rei.impl.client.gui.screen.DefaultDisplayViewingScreen".into(),
                "cc.cassian.rrv.common.recipe.inventory.RecipeViewScreen".into(),
            ],
            disabled_screens: Vec::new(),
            seen_screens: Vec::new(),
            disable_first_person: false,
            disable_third_person: false,
            render_scale_percent: 100,
            recipe_book_shrink_factor: 1.5,
            drop_shadow_enabled: true,
            drop_shadow_layers: 1,
            drop_shadow_alpha: 120,
            drop_shadow_scale_multiplier: 1.0,
            drop_shadow_horizontal_offset: 0.008,
            drop_shadow_vertical_offset: -0.012,
            drop_shadow_depth_offset: 0.002,
            enable_world_lighting_tint: false,
            lighting_tint_strength: 50.0,
            hide_hands_in_first_person: false,
            hide_shield_in_first_person: true,
            arm_swing: false,
            hand_poke_animation: false,
            hand_poke_strength: 1.0,
            hand_poke_duration_ms: 150,
            screen_alpha: 255,
            render_player_in_front: true,
            screen_over_player_alpha: 50,
            blur_world_background: false,
            world_blur_strength: 5,
            use_linear_filtering: true,
            use_anisotropic_filtering: true,
            screen_distance: 2.5,
            screen_side_offset: -0.6,
            screen_height_offset: -0.4,
            screen_yaw_offset: 160.0,
            screen_pitch_offset: 0.0,
            screen_scale: 3.5,
            recipe_viewer_scale_multiplier: 0.65,
            elytra_flying_side_offset: -0.5,
            elytra_flying_height_offset: -0.4,
            camera_distance: 1.7,
            camera_side_offset: -1.2,
            camera_height_offset: 1.5,
            camera_target_pitch: 10.0,
            lerp_camera_pitch: true,
            disable_third_person_parallax: false,
            fallback_to_first_person_on_block_collision: false,
            mirror_third_person: false,
            mirror_head_movement: false,
            avatar_base_yaw_offset: 30.0,
            avatar_body_rotation_offset: 20.0,
            avatar_max_yaw_offset: 40,
            avatar_max_pitch_offset: 25,
            third_person_mouse_sensitivity_yaw: 0.1,
            third_person_mouse_sensitivity_pitch: 0.03,
            transition_duration_ms: 300,
            transition_skip_percentage: 0,
            curved_screen_enabled: false,
            curved_screen_arc_degrees: 30,
            first_person_screen_distance: 1.5,
            first_person_screen_side_offset: 0.0,
            first_person_screen_height_offset: 0.0,
            first_person_screen_yaw_offset: 180.0,
            first_person_screen_pitch_offset: 0.0,
            first_person_screen_scale: 1.8,
            keep_first_person_camera_angle: true,
            disable_first_person_parallax: false,
            first_person_mouse_sensitivity_yaw: 0.4,
            first_person_mouse_sensitivity_pitch: 0.12,
            first_person_pitch_clamp: 40,
            lock_first_person_yaw: false,
            enable_close_animation: true,
            enable_fade_animation: true,
            fade_duration_ms: 150,
            enable_scale_animation: true,
            open_animation_duration_ms: 300,
            animation_start_scale_percent: 75,
            animation_easing: "Back".into(),
            override_fov: false,
            target_fov: 70,
            fov_lerp_speed: 10,
            auto_scale_by_fov: true,
            auto_scale_baseline_fov: 70.0,
            auto_scale_screen_power: 1.2,
            auto_scale_side_offset_multiplier: -0.3,
            auto_scale_distance_multiplier: 0.8,
            auto_scale_third_person_screen_multiplier: 0.82,
            editor_intro_seen: false,
            profile_picker_tip_seen: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Setting {
    pub key: &'static str,
    pub label: &'static str,
    pub category: &'static str,
    pub kind: &'static str,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}
pub const SETTINGS: &[Setting] = &[
    Setting {
        key: "enabled",
        label: "Enabled",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "editorPausesGame",
        label: "Editor pauses game",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "autoDetectCameraMode",
        label: "Auto detect camera mode",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "viewMode",
        label: "View mode",
        category: "general",
        kind: "choice",
        min: -100.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "useCrosshairForFirstPerson",
        label: "Use crosshair for first person",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "hideHud",
        label: "Hide hud",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "hideHotbar",
        label: "Hide hotbar",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "overrideGuiScale",
        label: "Override gui scale",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "guiScale",
        label: "Gui scale",
        category: "general",
        kind: "integer",
        min: 1.000000000,
        max: 6.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "inventory",
        label: "Inventory",
        category: "screens",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "containers",
        label: "Containers",
        category: "screens",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "pauseScreen",
        label: "Pause screen",
        category: "screens",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "allScreens",
        label: "All screens",
        category: "screens",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "enabledScreens",
        label: "Enabled screens",
        category: "screens",
        kind: "list",
        min: -100.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "disabledScreens",
        label: "Disabled screens",
        category: "screens",
        kind: "list",
        min: -100.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "seenScreens",
        label: "Seen screens",
        category: "screens",
        kind: "list",
        min: -100.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "disableFirstPerson",
        label: "Disable first person",
        category: "screens",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "disableThirdPerson",
        label: "Disable third person",
        category: "screens",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "renderScalePercent",
        label: "Render scale percent",
        category: "appearance",
        kind: "integer",
        min: 25.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "recipeBookShrinkFactor",
        label: "Recipe book shrink factor",
        category: "appearance",
        kind: "number",
        min: 1.000000000,
        max: 3.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "dropShadowEnabled",
        label: "Drop shadow enabled",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "dropShadowLayers",
        label: "Drop shadow layers",
        category: "appearance",
        kind: "integer",
        min: 1.000000000,
        max: 6.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "dropShadowAlpha",
        label: "Drop shadow alpha",
        category: "appearance",
        kind: "integer",
        min: 0.000000000,
        max: 255.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "dropShadowScaleMultiplier",
        label: "Drop shadow scale multiplier",
        category: "appearance",
        kind: "number",
        min: 0.800000000,
        max: 1.200000000,
        step: 0.010000000,
    },
    Setting {
        key: "dropShadowHorizontalOffset",
        label: "Drop shadow horizontal offset",
        category: "appearance",
        kind: "number",
        min: -0.050000000,
        max: 0.050000000,
        step: 0.001000000,
    },
    Setting {
        key: "dropShadowVerticalOffset",
        label: "Drop shadow vertical offset",
        category: "appearance",
        kind: "number",
        min: -0.050000000,
        max: 0.050000000,
        step: 0.001000000,
    },
    Setting {
        key: "dropShadowDepthOffset",
        label: "Drop shadow depth offset",
        category: "appearance",
        kind: "number",
        min: 0.000000000,
        max: 0.050000000,
        step: 0.001000000,
    },
    Setting {
        key: "enableWorldLightingTint",
        label: "Enable world lighting tint",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "lightingTintStrength",
        label: "Lighting tint strength",
        category: "appearance",
        kind: "number",
        min: 0.000000000,
        max: 120.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "hideHandsInFirstPerson",
        label: "Hide hands in first person",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "hideShieldInFirstPerson",
        label: "Hide shield in first person",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "armSwing",
        label: "Arm swing",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "handPokeAnimation",
        label: "Hand poke animation",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "handPokeStrength",
        label: "Hand poke strength",
        category: "appearance",
        kind: "number",
        min: 0.250000000,
        max: 2.500000000,
        step: 0.050000000,
    },
    Setting {
        key: "handPokeDurationMs",
        label: "Hand poke duration ms",
        category: "appearance",
        kind: "integer",
        min: 50.000000000,
        max: 500.000000000,
        step: 10.000000000,
    },
    Setting {
        key: "screenAlpha",
        label: "Screen alpha",
        category: "appearance",
        kind: "integer",
        min: 0.000000000,
        max: 255.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "renderPlayerInFront",
        label: "Render player in front",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "screenOverPlayerAlpha",
        label: "Screen over player alpha",
        category: "appearance",
        kind: "integer",
        min: 0.000000000,
        max: 255.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "blurWorldBackground",
        label: "Blur world background",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "worldBlurStrength",
        label: "World blur strength",
        category: "appearance",
        kind: "integer",
        min: 1.000000000,
        max: 10.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "useLinearFiltering",
        label: "Use linear filtering",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "useAnisotropicFiltering",
        label: "Use anisotropic filtering",
        category: "appearance",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "screenDistance",
        label: "Screen distance",
        category: "thirdPerson",
        kind: "number",
        min: 0.100000000,
        max: 10.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "screenSideOffset",
        label: "Screen side offset",
        category: "thirdPerson",
        kind: "number",
        min: -5.000000000,
        max: 5.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "screenHeightOffset",
        label: "Screen height offset",
        category: "thirdPerson",
        kind: "number",
        min: -3.000000000,
        max: 3.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "screenYawOffset",
        label: "Screen yaw offset",
        category: "thirdPerson",
        kind: "number",
        min: -360.000000000,
        max: 360.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "screenPitchOffset",
        label: "Screen pitch offset",
        category: "thirdPerson",
        kind: "number",
        min: -90.000000000,
        max: 90.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "screenScale",
        label: "Screen scale",
        category: "thirdPerson",
        kind: "number",
        min: 0.500000000,
        max: 10.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "recipeViewerScaleMultiplier",
        label: "Recipe viewer scale multiplier",
        category: "general",
        kind: "number",
        min: -100.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "elytraFlyingSideOffset",
        label: "Elytra flying side offset",
        category: "thirdPerson",
        kind: "number",
        min: -2.000000000,
        max: 2.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "elytraFlyingHeightOffset",
        label: "Elytra flying height offset",
        category: "thirdPerson",
        kind: "number",
        min: -2.000000000,
        max: 2.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "cameraDistance",
        label: "Camera distance",
        category: "thirdPerson",
        kind: "number",
        min: 0.500000000,
        max: 6.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "cameraSideOffset",
        label: "Camera side offset",
        category: "thirdPerson",
        kind: "number",
        min: -4.000000000,
        max: 4.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "cameraHeightOffset",
        label: "Camera height offset",
        category: "thirdPerson",
        kind: "number",
        min: -1.000000000,
        max: 4.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "cameraTargetPitch",
        label: "Camera target pitch",
        category: "thirdPerson",
        kind: "number",
        min: -90.000000000,
        max: 90.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "lerpCameraPitch",
        label: "Lerp camera pitch",
        category: "thirdPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "disableThirdPersonParallax",
        label: "Disable third person parallax",
        category: "thirdPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "fallbackToFirstPersonOnBlockCollision",
        label: "Fallback to first person on block collision",
        category: "thirdPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "mirrorThirdPerson",
        label: "Mirror third person",
        category: "thirdPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "mirrorHeadMovement",
        label: "Mirror head movement",
        category: "thirdPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "avatarBaseYawOffset",
        label: "Avatar base yaw offset",
        category: "thirdPerson",
        kind: "number",
        min: -180.000000000,
        max: 180.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "avatarBodyRotationOffset",
        label: "Avatar body rotation offset",
        category: "thirdPerson",
        kind: "number",
        min: -180.000000000,
        max: 180.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "avatarMaxYawOffset",
        label: "Avatar max yaw offset",
        category: "thirdPerson",
        kind: "integer",
        min: 0.000000000,
        max: 90.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "avatarMaxPitchOffset",
        label: "Avatar max pitch offset",
        category: "thirdPerson",
        kind: "integer",
        min: 0.000000000,
        max: 90.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "thirdPersonMouseSensitivityYaw",
        label: "Third person mouse sensitivity yaw",
        category: "thirdPerson",
        kind: "number",
        min: 0.010000000,
        max: 1.000000000,
        step: 0.010000000,
    },
    Setting {
        key: "thirdPersonMouseSensitivityPitch",
        label: "Third person mouse sensitivity pitch",
        category: "thirdPerson",
        kind: "number",
        min: 0.010000000,
        max: 1.000000000,
        step: 0.010000000,
    },
    Setting {
        key: "transitionDurationMs",
        label: "Transition duration ms",
        category: "thirdPerson",
        kind: "integer",
        min: 1.000000000,
        max: 5000.000000000,
        step: 10.000000000,
    },
    Setting {
        key: "transitionSkipPercentage",
        label: "Transition skip percentage",
        category: "thirdPerson",
        kind: "integer",
        min: 0.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "curvedScreenEnabled",
        label: "Curved screen enabled",
        category: "firstPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "curvedScreenArcDegrees",
        label: "Curved screen arc degrees",
        category: "firstPerson",
        kind: "integer",
        min: 5.000000000,
        max: 150.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "firstPersonScreenDistance",
        label: "First person screen distance",
        category: "firstPerson",
        kind: "number",
        min: 0.100000000,
        max: 10.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "firstPersonScreenSideOffset",
        label: "First person screen side offset",
        category: "firstPerson",
        kind: "number",
        min: -3.000000000,
        max: 3.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "firstPersonScreenHeightOffset",
        label: "First person screen height offset",
        category: "firstPerson",
        kind: "number",
        min: -3.000000000,
        max: 3.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "firstPersonScreenYawOffset",
        label: "First person screen yaw offset",
        category: "firstPerson",
        kind: "number",
        min: -360.000000000,
        max: 360.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "firstPersonScreenPitchOffset",
        label: "First person screen pitch offset",
        category: "firstPerson",
        kind: "number",
        min: -90.000000000,
        max: 90.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "firstPersonScreenScale",
        label: "First person screen scale",
        category: "firstPerson",
        kind: "number",
        min: 0.500000000,
        max: 10.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "keepFirstPersonCameraAngle",
        label: "Keep first person camera angle",
        category: "firstPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "disableFirstPersonParallax",
        label: "Disable first person parallax",
        category: "firstPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "firstPersonMouseSensitivityYaw",
        label: "First person mouse sensitivity yaw",
        category: "firstPerson",
        kind: "number",
        min: 0.010000000,
        max: 1.000000000,
        step: 0.010000000,
    },
    Setting {
        key: "firstPersonMouseSensitivityPitch",
        label: "First person mouse sensitivity pitch",
        category: "firstPerson",
        kind: "number",
        min: 0.010000000,
        max: 1.000000000,
        step: 0.010000000,
    },
    Setting {
        key: "firstPersonPitchClamp",
        label: "First person pitch clamp",
        category: "firstPerson",
        kind: "integer",
        min: 0.000000000,
        max: 90.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "lockFirstPersonYaw",
        label: "Lock first person yaw",
        category: "firstPerson",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "enableCloseAnimation",
        label: "Enable close animation",
        category: "animations",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "enableFadeAnimation",
        label: "Enable fade animation",
        category: "animations",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "fadeDurationMs",
        label: "Fade duration ms",
        category: "animations",
        kind: "integer",
        min: 0.000000000,
        max: 1000.000000000,
        step: 10.000000000,
    },
    Setting {
        key: "enableScaleAnimation",
        label: "Enable scale animation",
        category: "animations",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "openAnimationDurationMs",
        label: "Open animation duration ms",
        category: "animations",
        kind: "integer",
        min: 50.000000000,
        max: 1000.000000000,
        step: 10.000000000,
    },
    Setting {
        key: "animationStartScalePercent",
        label: "Animation start scale percent",
        category: "animations",
        kind: "integer",
        min: 1.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "animationEasing",
        label: "Animation easing",
        category: "animations",
        kind: "choice",
        min: -100.000000000,
        max: 100.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "overrideFov",
        label: "Override fov",
        category: "fov",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "targetFov",
        label: "Target fov",
        category: "fov",
        kind: "integer",
        min: 30.000000000,
        max: 110.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "fovLerpSpeed",
        label: "Fov lerp speed",
        category: "fov",
        kind: "integer",
        min: 0.000000000,
        max: 50.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "autoScaleByFov",
        label: "Auto scale by fov",
        category: "fov",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "autoScaleBaselineFov",
        label: "Auto scale baseline fov",
        category: "fov",
        kind: "number",
        min: 30.000000000,
        max: 110.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "autoScaleScreenPower",
        label: "Auto scale screen power",
        category: "fov",
        kind: "number",
        min: 0.100000000,
        max: 3.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "autoScaleSideOffsetMultiplier",
        label: "Auto scale side offset multiplier",
        category: "fov",
        kind: "number",
        min: -2.000000000,
        max: 2.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "autoScaleDistanceMultiplier",
        label: "Auto scale distance multiplier",
        category: "fov",
        kind: "number",
        min: 0.100000000,
        max: 3.000000000,
        step: 0.050000000,
    },
    Setting {
        key: "autoScaleThirdPersonScreenMultiplier",
        label: "Auto scale third person screen multiplier",
        category: "fov",
        kind: "number",
        min: 0.100000000,
        max: 2.000000000,
        step: 0.010000000,
    },
    Setting {
        key: "editorIntroSeen",
        label: "Editor intro seen",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
    Setting {
        key: "profilePickerTipSeen",
        label: "Profile picker tip seen",
        category: "general",
        kind: "bool",
        min: 0.000000000,
        max: 1.000000000,
        step: 1.000000000,
    },
];

pub const CATEGORIES: &[&str] = &[
    "general",
    "screens",
    "appearance",
    "thirdPerson",
    "firstPerson",
    "animations",
    "fov",
];
pub const LAYOUT_FIELDS: &[&str] = &[
    "viewMode",
    "firstPersonScreenScale",
    "screenScale",
    "firstPersonScreenSideOffset",
    "screenSideOffset",
    "firstPersonScreenHeightOffset",
    "screenHeightOffset",
    "firstPersonScreenDistance",
    "screenDistance",
    "firstPersonScreenYawOffset",
    "screenYawOffset",
    "firstPersonScreenPitchOffset",
    "screenPitchOffset",
    "curvedScreenEnabled",
    "curvedScreenArcDegrees",
    "guiScale",
    "overrideGuiScale",
];

impl Config {
    pub fn from_value(value: Value) -> Result<Self, String> {
        let mut base = serde_json::to_value(Self::default()).map_err(|e| e.to_string())?;
        if let Some(values) = value.as_object() {
            for spec in SETTINGS {
                let Some(v) = values.get(spec.key) else {
                    continue;
                };
                let clean = match spec.kind {
                    "bool" => v.as_bool().map(Value::Bool),
                    "integer" => v
                        .as_f64()
                        .filter(|v| v.is_finite())
                        .map(|v| Value::from(v.clamp(spec.min, spec.max).round() as i64)),
                    "number" => v
                        .as_f64()
                        .filter(|v| v.is_finite())
                        .map(|v| Value::from(v.clamp(spec.min, spec.max))),
                    "choice" => v.as_str().map(|v| Value::String(v.into())),
                    "list" => v.as_array().map(|a| {
                        Value::Array(a.iter().filter(|v| v.is_string()).cloned().collect())
                    }),
                    _ => None,
                };
                if let Some(v) = clean {
                    base[spec.key] = v;
                }
            }
        } else {
            return Err("configuration must be an object".into());
        }
        let mut result: Self = serde_json::from_value(base).map_err(|e| e.to_string())?;
        if !["FIRST_PERSON", "THIRD_PERSON"].contains(&result.view_mode.as_str()) {
            result.view_mode = "THIRD_PERSON".into();
        }
        if ![
            "Cubic",
            "Elastic",
            "Bounce",
            "Back",
            "Exponential",
            "Quadratic",
            "Quartic",
        ]
        .contains(&result.animation_easing.as_str())
        {
            result.animation_easing = "Back".into();
        }
        Ok(result)
    }
    pub fn parse(text: &str) -> Result<Self, String> {
        Self::from_value(parse_json5(text)?)
    }
    pub fn value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
    pub fn edit(&mut self, key: &str, direction: f64) {
        if !direction.is_finite() || direction == 0.0 {
            return;
        }
        let Some(spec) = SETTINGS.iter().find(|s| s.key == key) else {
            return;
        };
        let mut v = self.value();
        match spec.kind {
            "bool" => v[key] = Value::Bool(!v[key].as_bool().unwrap_or(false)),
            "integer" | "number" => {
                let mut n = v[key].as_f64().unwrap_or(spec.min) + spec.step * direction;
                // Stored floats are f32. Ignore their rounding error at the
                // endpoints so a reverse step back to the minimum cannot wrap.
                let epsilon = spec.step.abs() * 0.0001;
                if n > spec.max + epsilon {
                    n = spec.min;
                }
                if n < spec.min - epsilon {
                    n = spec.max;
                }
                n = n.clamp(spec.min, spec.max);
                v[key] = if spec.kind == "integer" {
                    Value::from(n.round() as i64)
                } else {
                    Value::from(n)
                };
            }
            "choice" => {
                let choices = if key == "viewMode" {
                    vec!["FIRST_PERSON", "THIRD_PERSON"]
                } else {
                    vec![
                        "Cubic",
                        "Elastic",
                        "Bounce",
                        "Back",
                        "Exponential",
                        "Quadratic",
                        "Quartic",
                    ]
                };
                let i = choices
                    .iter()
                    .position(|s| Some(*s) == v[key].as_str())
                    .unwrap_or(0);
                let step = if direction < 0.0 {
                    choices.len() - 1
                } else {
                    1
                };
                v[key] = Value::String(choices[(i + step) % choices.len()].into());
            }
            _ => {}
        }
        if let Ok(c) = Self::from_value(v) {
            *self = c;
        }
    }
    pub fn label(&self, spec: &Setting) -> String {
        let values = self.value();
        let v = &values[spec.key];
        let text = match spec.kind {
            "bool" => {
                if v.as_bool().unwrap_or(false) {
                    "ON".into()
                } else {
                    "OFF".into()
                }
            }
            "list" => format!("{} entries", v.as_array().map_or(0, Vec::len)),
            "choice" => v.as_str().unwrap_or("").replace('_', " "),
            _ => v.to_string(),
        };
        format!("{}: {}", spec.label, text)
    }
    pub fn easing(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        let x = t - 1.0;
        match self.animation_easing.as_str() {
            "Elastic" => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    2.0_f32.powf(-10.0 * t)
                        * ((t * 10.0 - 0.75) * std::f32::consts::TAU / 3.0).sin()
                        + 1.0
                }
            }
            "Bounce" => {
                let (x, b) = if t < 1.0 / 2.75 {
                    (t, 0.0)
                } else if t < 2.0 / 2.75 {
                    (t - 1.5 / 2.75, 0.75)
                } else if t < 2.5 / 2.75 {
                    (t - 2.25 / 2.75, 0.9375)
                } else {
                    (t - 2.625 / 2.75, 0.984375)
                };
                7.5625 * x * x + b
            }
            "Back" => 1.0 + 2.70158 * x * x * x + 1.70158 * x * x,
            "Exponential" => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }
            "Quadratic" => 1.0 - x * x,
            "Quartic" => 1.0 - x * x * x * x,
            _ => 1.0 + x * x * x,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Profiles {
    pub profiles: BTreeMap<String, Value>,
    pub rules: BTreeMap<String, String>,
    #[serde(rename = "default")]
    pub linked: Option<String>,
}
impl Profiles {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut profiles: Self =
            serde_json::from_value(parse_json5(text)?).map_err(|e| e.to_string())?;
        if let Some(name) = profiles.linked.clone() {
            if profiles.profiles.contains_key(&name) {
                profiles.rules.insert("#all".into(), name);
            } else {
                profiles.linked = None;
            }
        }
        Ok(profiles)
    }
    pub fn resolve(
        &self,
        screen: &str,
        alias: &str,
        category: &str,
        base: &Config,
    ) -> (Config, Option<String>) {
        let chosen = self
            .rules
            .get(screen)
            .or_else(|| self.rules.get(alias))
            .or_else(|| self.rules.get(category))
            .or_else(|| self.rules.get("#all"))
            .or(self.linked.as_ref());
        let Some(name) = chosen else {
            return (base.clone(), None);
        };
        if self.linked.as_deref() == Some(name) {
            return (base.clone(), Some(name.clone()));
        }
        let Some(overrides) = self.profiles.get(name).and_then(Value::as_object) else {
            return (base.clone(), None);
        };
        let mut value = base.value();
        for (key, v) in overrides {
            value[key] = v.clone();
        }
        if overrides.contains_key("guiScale") && !overrides.contains_key("overrideGuiScale") {
            value["overrideGuiScale"] = true.into();
        }
        (
            Config::from_value(value).unwrap_or_else(|_| base.clone()),
            Some(name.clone()),
        )
    }
    pub fn create(&mut self, config: &Config) -> String {
        let mut i = self.profiles.len() + 1;
        while self.profiles.contains_key(&format!("Profile {i}")) {
            i += 1;
        }
        let name = format!("Profile {i}");
        self.update(&name, config);
        name
    }
    pub fn update(&mut self, name: &str, config: &Config) {
        let all = config.value();
        let mut values = self
            .profiles
            .get(name)
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        for key in LAYOUT_FIELDS {
            values.insert((*key).into(), all[*key].clone());
        }
        self.profiles.insert(name.into(), Value::Object(values));
    }
    pub fn rename(&mut self, old: &str, new: &str) -> bool {
        if new.trim().is_empty() || self.profiles.contains_key(new) {
            return false;
        }
        let Some(v) = self.profiles.remove(old) else {
            return false;
        };
        self.profiles.insert(new.into(), v);
        for name in self.rules.values_mut() {
            if name == old {
                *name = new.into();
            }
        }
        if self.linked.as_deref() == Some(old) {
            self.linked = Some(new.into());
        }
        true
    }
    pub fn remove(&mut self, name: &str) -> bool {
        if self.linked.as_deref() == Some(name) {
            return false;
        }
        self.rules.retain(|_, v| v != name);
        self.profiles.remove(name).is_some()
    }
    pub fn assign(&mut self, target: &str, name: Option<&str>) {
        if let Some(name) = name {
            if self.profiles.contains_key(name) {
                self.rules.insert(target.into(), name.into());
            }
        } else {
            self.rules.remove(target);
        }
    }
}

/// A bounded recursive-descent JSON5 reader. Used only for the mod's own config;
/// supports comments, quoted/unquoted keys, single quotes, trailing commas and hex.
pub fn parse_json5(text: &str) -> Result<Value, String> {
    struct Parser {
        s: Vec<char>,
        i: usize,
        depth: u16,
    }
    impl Parser {
        fn skip(&mut self) -> Result<(), String> {
            loop {
                while self
                    .s
                    .get(self.i)
                    .is_some_and(|c| c.is_whitespace() || *c == '\u{feff}')
                {
                    self.i += 1;
                }
                if self.s.get(self.i) == Some(&'/') && self.s.get(self.i + 1) == Some(&'/') {
                    while self.s.get(self.i).is_some_and(|c| *c != '\n') {
                        self.i += 1;
                    }
                    continue;
                }
                if self.s.get(self.i) == Some(&'/') && self.s.get(self.i + 1) == Some(&'*') {
                    self.i += 2;
                    while self.i + 1 < self.s.len() && self.s[self.i..self.i + 2] != ['*', '/'] {
                        self.i += 1;
                    }
                    if self.i + 1 >= self.s.len() {
                        return Err("unterminated comment".into());
                    }
                    self.i += 2;
                    continue;
                }
                break;
            }
            Ok(())
        }
        fn string(&mut self) -> Result<String, String> {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            while let Some(&c) = self.s.get(self.i) {
                self.i += 1;
                if c == q {
                    return Ok(out);
                }
                if c != '\\' {
                    out.push(c);
                    continue;
                }
                let c = *self.s.get(self.i).ok_or("unterminated escape")?;
                self.i += 1;
                match c {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    'v' => out.push('\u{b}'),
                    '0' => out.push('\0'),
                    '\n' => {}
                    '\r' => {
                        if self.s.get(self.i) == Some(&'\n') {
                            self.i += 1;
                        }
                    }
                    'u' | 'x' => {
                        let n = if c == 'u' { 4 } else { 2 };
                        let end = self.i + n;
                        if end > self.s.len() {
                            return Err("short unicode escape".into());
                        }
                        let hex: String = self.s[self.i..end].iter().collect();
                        let code =
                            u32::from_str_radix(&hex, 16).map_err(|_| "invalid unicode escape")?;
                        self.i = end;
                        if (0xd800..=0xdbff).contains(&code)
                            && self.s.get(self.i..self.i + 2) == Some(&['\\', 'u'])
                        {
                            self.i += 2;
                            let end = self.i + 4;
                            if end > self.s.len() {
                                return Err("short surrogate".into());
                            }
                            let hex: String = self.s[self.i..end].iter().collect();
                            let low =
                                u32::from_str_radix(&hex, 16).map_err(|_| "invalid surrogate")?;
                            self.i = end;
                            let cp = 0x10000
                                + ((code - 0xd800) << 10)
                                + (low
                                    .checked_sub(0xdc00)
                                    .filter(|v| *v < 1024)
                                    .ok_or("invalid surrogate")?);
                            out.push(char::from_u32(cp).ok_or("invalid character")?);
                        } else {
                            out.push(char::from_u32(code).ok_or("invalid character")?);
                        }
                    }
                    _ => out.push(c),
                }
            }
            Err("unterminated string".into())
        }
        fn word(&mut self) -> String {
            let start = self.i;
            while self
                .s
                .get(self.i)
                .is_some_and(|c| !c.is_whitespace() && ![':', ',', '}', ']', '/'].contains(c))
            {
                self.i += 1;
            }
            self.s[start..self.i].iter().collect()
        }
        fn value(&mut self) -> Result<Value, String> {
            self.skip()?;
            self.depth += 1;
            if self.depth > 64 {
                return Err("JSON5 nesting exceeds 64".into());
            }
            let c = *self.s.get(self.i).ok_or("expected value")?;
            let v = match c {
                '{' => {
                    self.i += 1;
                    let mut m = serde_json::Map::new();
                    loop {
                        self.skip()?;
                        if self.s.get(self.i) == Some(&'}') {
                            self.i += 1;
                            break;
                        }
                        let key = if self.s.get(self.i).is_some_and(|c| *c == '\'' || *c == '"') {
                            self.string()?
                        } else {
                            self.word()
                        };
                        self.skip()?;
                        if key.is_empty() || self.s.get(self.i) != Some(&':') {
                            return Err(format!("expected ':' at {}", self.i));
                        }
                        self.i += 1;
                        m.insert(key, self.value()?);
                        self.skip()?;
                        match self.s.get(self.i) {
                            Some(',') => self.i += 1,
                            Some('}') => {}
                            _ => return Err("expected ',' or '}'".into()),
                        }
                    }
                    Value::Object(m)
                }
                '[' => {
                    self.i += 1;
                    let mut a = Vec::new();
                    loop {
                        self.skip()?;
                        if self.s.get(self.i) == Some(&']') {
                            self.i += 1;
                            break;
                        }
                        a.push(self.value()?);
                        self.skip()?;
                        match self.s.get(self.i) {
                            Some(',') => self.i += 1,
                            Some(']') => {}
                            _ => return Err("expected ',' or ']'".into()),
                        }
                    }
                    Value::Array(a)
                }
                '\'' | '"' => Value::String(self.string()?),
                _ => {
                    let word = self.word();
                    match word.as_str() {
                        "true" => true.into(),
                        "false" => false.into(),
                        "null" | "Infinity" | "-Infinity" | "+Infinity" | "NaN" => Value::Null,
                        _ => {
                            let (negative, digits) = if let Some(s) = word.strip_prefix('-') {
                                (true, s)
                            } else {
                                (false, word.trim_start_matches('+'))
                            };
                            if let Some(hex) = digits
                                .strip_prefix("0x")
                                .or_else(|| digits.strip_prefix("0X"))
                            {
                                let n = i64::from_str_radix(hex, 16)
                                    .map_err(|_| "invalid hex number")?;
                                Value::from(if negative { -n } else { n })
                            } else {
                                let n = word
                                    .parse::<f64>()
                                    .map_err(|_| format!("invalid value {word:?}"))?;
                                serde_json::Number::from_f64(n)
                                    .map(Value::Number)
                                    .ok_or("nonfinite number")?
                            }
                        }
                    }
                }
            };
            self.depth -= 1;
            Ok(v)
        }
    }
    if text.len() > 4 * 1024 * 1024 {
        return Err("configuration exceeds 4 MiB".into());
    }
    let mut p = Parser {
        s: text.chars().collect(),
        i: 0,
        depth: 0,
    };
    let v = p.value()?;
    p.skip()?;
    if p.i != p.s.len() {
        return Err(format!("unexpected input at {}", p.i));
    }
    Ok(v)
}
