use glam::{DVec3, Quat, Vec2, Vec3};
use spatial_gui_native::{
    config::{Config, Profiles, SETTINGS},
    geometry::Surface,
};

#[test]
fn upstream_schema_has_one_setting_per_serialized_field() {
    let config = Config::default();
    let value = config.value();
    assert_eq!(value.as_object().unwrap().len(), 100);
    assert_eq!(SETTINGS.len(), 100);
    for spec in SETTINGS {
        assert!(value.get(spec.key).is_some(), "{}", spec.key);
    }
    assert_eq!(
        Config::parse(&serde_json::to_string(&config).unwrap()).unwrap(),
        config
    );
}

#[test]
fn imports_upstream_json5_and_bounds_invalid_settings() {
    let config = Config::parse("/* original config */ {viewMode:'FIRST_PERSON', guiScale:99, screenAlpha:0xff, firstPersonScreenScale:.8, enabledScreens:['inventory',], animationEasing:'invalid',}").unwrap();
    assert_eq!(config.view_mode, "FIRST_PERSON");
    assert_eq!(config.gui_scale, 6);
    assert_eq!(config.screen_alpha, 255);
    assert_eq!(config.first_person_screen_scale, 0.8);
    assert_eq!(config.animation_easing, "Back");
    assert_eq!(config.enabled_screens, ["inventory"]);
    assert!(Config::parse("{ enabled: true").is_err());
    assert!(Config::parse("[]").is_err());
}

#[test]
fn json5_unicode_and_untrusted_nesting() {
    let config = Config::parse(r#"{ enabledScreens: ['\uD83D\uDE80', 'café', '\x41'], }"#).unwrap();
    assert_eq!(config.enabled_screens, ["🚀", "café", "A"]);
    assert!(Config::parse(&format!("{}0{}", "[".repeat(65), "]".repeat(65))).is_err());
}

#[test]
fn profiles_follow_screen_category_and_global_precedence() {
    let mut p = Profiles::parse("{profiles:{screen:{screenScale:1},category:{screenScale:2},global:{screenScale:3}},rules:{inventory:'screen','#inventory':'category','#all':'global'}}").unwrap();
    let base = Config::default();
    assert_eq!(
        p.resolve("inventory", "InventoryScreen", "#inventory", &base)
            .0
            .screen_scale,
        1.0
    );
    assert_eq!(
        p.resolve("creative", "CreativeScreen", "#inventory", &base)
            .0
            .screen_scale,
        2.0
    );
    assert_eq!(
        p.resolve("chat", "ChatScreen", "#other", &base)
            .0
            .screen_scale,
        3.0
    );
    assert!(p.rename("category", "compact"));
    assert_eq!(p.rules["#inventory"], "compact");
    p.linked = Some("global".into());
    assert!(!p.remove("global"));
    assert_eq!(p.resolve("chat", "ChatScreen", "#other", &base).0, base);
    assert!(p.remove("compact"));
    assert!(!p.rules.contains_key("#inventory"));
}

#[test]
fn profile_gui_override_and_roundtrip() {
    let p = Profiles::parse("{profiles:{compact:{guiScale:2}},rules:{'#all':'compact'}}").unwrap();
    let (config, name) = p.resolve("inventory", "", "#inventory", &Config::default());
    assert!(config.override_gui_scale);
    assert_eq!(config.gui_scale, 2);
    assert_eq!(name.as_deref(), Some("compact"));
    let reloaded = Profiles::parse(&serde_json::to_string(&p).unwrap()).unwrap();
    assert_eq!(
        reloaded
            .resolve("inventory", "", "#inventory", &Config::default())
            .0,
        config
    );
}

#[test]
fn animation_endpoints_and_finite_overshoot() {
    for easing in [
        "Cubic",
        "Elastic",
        "Bounce",
        "Back",
        "Exponential",
        "Quadratic",
        "Quartic",
    ] {
        let c = Config {
            animation_easing: easing.into(),
            ..Config::default()
        };
        assert!(c.easing(0.0).abs() < 0.00001, "{easing}");
        assert!((c.easing(1.0) - 1.0).abs() < 0.00001, "{easing}");
        for i in 0..=100 {
            assert!(c.easing(i as f32 / 100.0).is_finite());
        }
    }
}

#[test]
fn world_surface_ray_mapping_matches_rendered_vertices() {
    // Camera-relative arithmetic must retain precision far from spawn.
    for center in [DVec3::ZERO, DVec3::new(20_000_000.0, 100.0, -20_000_000.0)] {
        for angle in [0.0_f32, 0.5, 1.8] {
            for arc in [0.0_f32, 0.2, 0.9, 2.5] {
                let rotation = Quat::from_euler(glam::EulerRot::YXZ, angle, 0.3, -0.2);
                let surface = Surface {
                    center,
                    rotation,
                    width: 3.2,
                    height: 1.8,
                    arc,
                    gui: [854.0, 480.0],
                };
                let eye = center + (rotation * Vec3::new(0.0, 0.0, 4.0)).as_dvec3();
                for x in [0.1, 0.3, 0.5, 0.7, 0.9] {
                    for y in [0.1, 0.5, 0.9] {
                        let point = surface.point(Vec2::new(x, y));
                        let hit = surface
                            .intersect(eye, (point - eye).as_vec3().normalize())
                            .unwrap();
                        assert!(
                            (hit[0] - x * surface.gui[0]).abs() < 0.02,
                            "{hit:?}, {x}, {arc}"
                        );
                        assert!((hit[1] - y * surface.gui[1]).abs() < 0.02);
                    }
                }
            }
        }
    }
}

#[test]
fn surface_rejects_parallel_and_backward_rays() {
    let surface = Surface {
        center: DVec3::ZERO,
        rotation: Quat::IDENTITY,
        width: 2.0,
        height: 1.0,
        arc: 0.0,
        gui: [640.0, 360.0],
    };
    assert!(surface.intersect(DVec3::Z, Vec3::X).is_none());
    assert!(surface.intersect(DVec3::Z, Vec3::Z).is_none());
    assert_eq!(
        surface.intersect(DVec3::Z, Vec3::NEG_Z),
        Some([320.0, 180.0])
    );
}

#[test]
fn editor_drag_undo_redo_and_branching_history() {
    use spatial_gui_native::editor::{Drag, Editor};
    let mut editor = Editor::default();
    let mut config = Config::default();
    editor.begin(&config);
    let original = config.clone();
    editor.checkpoint(&config);
    editor.drag = Some(Drag::Move { last: Vec2::ZERO });
    editor.drag_to(Vec2::new(50.0, -25.0), &mut config, false);
    assert!((config.screen_side_offset - original.screen_side_offset + 0.5).abs() < 0.0001);
    assert!((config.screen_height_offset - original.screen_height_offset - 0.25).abs() < 0.0001);
    let moved = config.clone();
    editor.undo(&mut config);
    assert_eq!(config, original);
    editor.redo(&mut config);
    assert_eq!(config, moved);
    editor.undo(&mut config);
    editor.scroll(1.0, &mut config, false);
    assert!(editor.redo.is_empty());
    assert!(config.screen_scale > original.screen_scale);
    editor.end();
    assert!(!editor.active);
    assert!(editor.drag.is_none());
}

#[test]
fn linked_profile_import_restores_global_rule() {
    let profiles =
        Profiles::parse("{profiles:{main:{},other:{}},rules:{'#all':'other'},default:'main'}")
            .unwrap();
    assert_eq!(profiles.rules["#all"], "main");
    let invalid = Profiles::parse("{profiles:{},rules:{},default:'missing'}").unwrap();
    assert!(invalid.linked.is_none());
}

#[test]
fn third_person_avatar_keeps_the_anchored_facing_direction() {
    let config = Config::default();
    for anchor in [-180.0_f32, -90.0, 0.0, 90.0, 359.0] {
        let [body, head, pitch] =
            spatial_gui_native::avatar::pose(&config, anchor, [0.0, 0.0], None);
        assert!((body - anchor - config.avatar_body_rotation_offset).abs() < 0.001);
        assert!((head - body - config.avatar_base_yaw_offset).abs() < 0.001);
        // A half-turn regression points the model against the anchored heading.
        assert!((body - anchor).to_radians().cos() > 0.0);
        assert_eq!(pitch, 0.0);
    }
}

#[test]
fn third_person_avatar_mirroring_keeps_body_and_head_offsets_separate() {
    let config = Config {
        mirror_third_person: true,
        mirror_head_movement: true,
        ..Config::default()
    };
    let [body, head, pitch] = spatial_gui_native::avatar::pose(&config, 90.0, [1.0, 1.0], None);
    assert_eq!(body, 90.0 - config.avatar_body_rotation_offset);
    assert_eq!(
        head,
        body - config.avatar_base_yaw_offset - config.avatar_max_yaw_offset as f32
    );
    assert_eq!(pitch, config.avatar_max_pitch_offset as f32);
}

#[test]
fn avatar_looks_at_the_pointer_even_with_camera_parallax_disabled() {
    let config = Config {
        disable_third_person_parallax: true,
        ..Config::default()
    };
    let [body, head, pitch] = spatial_gui_native::avatar::pose(&config, 0.0, [2.0, -2.0], None);
    assert_eq!(
        head - body,
        config.avatar_base_yaw_offset + config.avatar_max_yaw_offset as f32
    );
    assert_eq!(pitch, -(config.avatar_max_pitch_offset as f32));
    assert_eq!(
        spatial_gui_native::avatar::pose(&config, 0.0, [1.0, 1.0], Some(-35.0))[2],
        -35.0
    );
}

#[test]
fn numeric_settings_step_back_and_wrap_at_both_limits() {
    for spec in SETTINGS
        .iter()
        .filter(|s| s.kind == "number" || s.kind == "integer")
    {
        let mut value = Config::default().value();
        value[spec.key] = serde_json::Value::from(spec.min);
        let mut config = Config::from_value(value).unwrap();
        config.edit(spec.key, -1.0);
        let at = config.value()[spec.key].as_f64().unwrap();
        assert!((at - spec.max).abs() < 0.0001, "{}: {at}", spec.key);
        config.edit(spec.key, 1.0);
        let at = config.value()[spec.key].as_f64().unwrap();
        assert!((at - spec.min).abs() < 0.0001, "{}: {at}", spec.key);
        config.edit(spec.key, 1.0);
        let at = config.value()[spec.key].as_f64().unwrap();
        assert!(
            (at - spec.min - spec.step).abs() < 0.0001,
            "{}: {at}",
            spec.key
        );
        config.edit(spec.key, -1.0);
        let at = config.value()[spec.key].as_f64().unwrap();
        assert!((at - spec.min).abs() < 0.0001, "{}: {at}", spec.key);
    }
}

#[test]
fn settings_choices_cycle_backwards_with_wrapping() {
    let mut config = Config::default();
    config.animation_easing = "Cubic".into();
    config.edit("animationEasing", -1.0);
    assert_eq!(config.animation_easing, "Quartic");
    config.edit("animationEasing", -1.0);
    assert_eq!(config.animation_easing, "Quadratic");
    config.edit("animationEasing", 1.0);
    config.edit("animationEasing", 1.0);
    assert_eq!(config.animation_easing, "Cubic");
    let original = config.view_mode.clone();
    config.edit("viewMode", -1.0);
    assert_ne!(config.view_mode, original);
    config.edit("viewMode", 1.0);
    assert_eq!(config.view_mode, original);
}

#[test]
fn reverse_boolean_settings_toggle_and_empty_scroll_does_nothing() {
    let mut config = Config::default();
    let original = config.clone();
    config.edit("enabled", -1.0);
    assert_eq!(config.enabled, !original.enabled);
    config.edit("enabled", 1.0);
    assert_eq!(config, original);
    for spec in SETTINGS {
        for amount in [0.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            config.edit(spec.key, amount);
            assert_eq!(config, original, "{}: {amount}", spec.key);
        }
    }
}
