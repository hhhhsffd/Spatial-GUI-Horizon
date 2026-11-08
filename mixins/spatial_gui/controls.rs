// Native key/mouse/controller routing and the complete editor/profile UI.
use super::{
    Game, Screen,
    spatial_config::{CATEGORIES, Config, SETTINGS},
    spatial_editor::{Dialog, Drag, Tool},
    spatial_integration::{self as runtime, ID, STATE, Session},
};
use glam::{Vec2, Vec3};
use horizon_client::hud::{HudBatch, spatial_gui as gpu};
use std::time::Instant;
use winit::{
    event::{DeviceEvent, ElementState, Ime, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::PhysicalKey,
};

fn binding(game: &Game<'_>, name: &str, default: &str) -> String {
    game.native_mod_key(ID, name, default)
}
fn scope_target(s: &Session) -> String {
    s.identity.as_ref().map_or_else(
        || "#all".into(),
        |id| match s.editor.scope {
            0 => id.key.clone(),
            1 => id.category.clone(),
            _ => "#all".into(),
        },
    )
}
fn apply_profile(s: &mut Session) {
    if let Some(id) = &s.identity {
        let (c, name) = s
            .profiles
            .resolve(&id.key, &id.alias, &id.category, &s.global);
        s.config = c;
        s.selected = name;
        s.editor.selected = s.selected.clone();
    }
}
fn toggle_editor(game: &mut Game<'_>, s: &mut Session) {
    if s.editor.active {
        runtime::save_layout(s);
        s.editor.end();
    } else {
        if game.screen == Screen::Playing || game.screen == Screen::Options {
            game.screen = Screen::Inventory;
            game.capture(false);
        }
        if s.profiles.linked.is_none() {
            let global = s.global.clone();
            let name = s.profiles.create(&global);
            s.profiles.linked = Some(name.clone());
            s.profiles.rules.insert("#all".into(), name.clone());
            s.selected = Some(name);
            runtime::save_profiles(s);
        }
        let config = s.config.clone();
        s.editor.begin(&config);
        s.editor.selected = s.selected.clone();
        if !s.global.editor_intro_seen {
            s.editor.notice =
                "Drag the axes; middle drag moves, right drag orbits; hold Space to preview".into();
        }
        s.global.editor_intro_seen = true;
        runtime::save_global(s);
    }
}
fn action(game: &mut Game<'_>, s: &mut Session, id: &str) {
    if let Some(name) = id.strip_prefix("profile:") {
        let target = scope_target(s);
        s.profiles.assign(&target, Some(name));
        runtime::save_profiles(s);
        apply_profile(s);
        s.editor.picker = false;
        return;
    }
    match id {
        "undo" => s.editor.undo(&mut s.config),
        "redo" => s.editor.redo(&mut s.config),
        "undo_all" => {
            if let Some(initial) = s.editor.initial.clone() {
                s.editor.checkpoint(&s.config);
                s.config = initial;
            }
        }
        "view" => {
            s.editor.checkpoint(&s.config);
            s.config.auto_detect_camera_mode = false;
            s.config.view_mode = if s.first {
                "THIRD_PERSON"
            } else {
                "FIRST_PERSON"
            }
            .into();
        }
        "gui" => {
            s.editor.checkpoint(&s.config);
            s.config.override_gui_scale = true;
            s.config.gui_scale = s.config.gui_scale % 6 + 1;
        }
        "curve" => {
            s.editor.checkpoint(&s.config);
            s.config.curved_screen_enabled = !s.config.curved_screen_enabled;
        }
        "profiles" => {
            s.editor.picker = true;
            s.editor.selected = s.selected.clone();
        }
        "translate" => s.editor.tool = Tool::Translate,
        "rotate" => s.editor.tool = Tool::Rotate,
        "scale" => s.editor.tool = Tool::Scale,
        "done" => {
            runtime::save_layout(s);
            s.editor.end();
        }
        "picker_close" => s.editor.picker = false,
        "scope" => s.editor.scope = (s.editor.scope + 1) % 3,
        "unassign" => {
            let target = scope_target(s);
            s.profiles.assign(&target, None);
            runtime::save_profiles(s);
            apply_profile(s);
            s.editor.picker = false;
        }
        "create" => {
            let config = s.config.clone();
            let name = s.profiles.create(&config);
            s.editor.selected = Some(name);
            runtime::save_profiles(s);
        }
        "rename" => {
            if let Some(old) = s.editor.selected.clone() {
                s.editor.dialog = Some(Dialog::Rename {
                    text: old.clone(),
                    old,
                });
            }
        }
        "delete" => {
            if let Some(name) = s.editor.selected.clone() {
                if s.profiles.remove(&name) {
                    s.editor.selected = None;
                    runtime::save_profiles(s);
                    apply_profile(s);
                } else {
                    s.editor.notice = "The linked default cannot be deleted".into();
                }
            }
        }
        "default" => {
            let name = if let Some(name) = s.editor.selected.clone().or(s.selected.clone()) {
                name
            } else {
                let config = s.config.clone();
                s.profiles.create(&config)
            };
            let config = s.config.clone();
            s.profiles.update(&name, &config);
            s.profiles.linked = Some(name.clone());
            s.profiles.rules.insert("#all".into(), name.clone());
            s.selected = Some(name.clone());
            s.editor.selected = Some(name);
            s.global = config;
            runtime::save_global(s);
            runtime::save_profiles(s);
        }
        "dialog_cancel" => s.editor.dialog = None,
        "dialog_ok" => {
            if let Some(dialog) = s.editor.dialog.take() {
                match dialog {
                    Dialog::Rename { old, text } => {
                        if s.profiles.rename(&old, text.trim()) {
                            s.editor.selected = Some(text.trim().into());
                            if s.selected.as_deref() == Some(&old) {
                                s.selected = Some(text.trim().into());
                            }
                            runtime::save_profiles(s);
                        } else {
                            s.editor.notice = "Use a unique, nonempty profile name".into();
                        }
                    }
                    Dialog::List { key, text } => {
                        let mut value = s.global.value();
                        value[&key] = serde_json::Value::Array(
                            text.lines()
                                .map(str::trim)
                                .filter(|v| !v.is_empty())
                                .map(|v| serde_json::Value::String(v.into()))
                                .collect(),
                        );
                        if let Ok(config) = Config::from_value(value) {
                            s.global = config;
                            s.config = s.global.clone();
                            runtime::save_global(s);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if s.editor.active && !s.editor.picker && s.editor.dialog.is_none() {
        runtime::save_layout(s);
    }
    let _ = game;
}

pub(super) fn key_event(
    game: &mut Game<'_>,
    name: &str,
    down: bool,
    repeat: bool,
    text: Option<&str>,
) -> bool {
    runtime::load(game);
    if !game.native_on(ID) {
        return false;
    }
    let editor_key = binding(game, "editor", "key.keyboard.f8");
    let picker_key = binding(game, "profiles", "key.keyboard.grave.accent");
    let config_key = binding(game, "config", "key.keyboard.f7");
    let reload_key = binding(game, "reload", "key.keyboard.f6");
    let free_key = binding(game, "mouse_free", "key.keyboard.tab");
    let mut open_options = false;
    let claimed = STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        if !down {
            if name == "key.keyboard.space" && s.editor.active {
                s.editor.preview = false;
                return true;
            }
            return false;
        }
        if name == reload_key {
            if !repeat {
                runtime::reload(&mut s);
            }
            return true;
        }
        if name == config_key {
            if !repeat {
                open_options = true;
            }
            return true;
        }
        if !s.global.enabled || game.player.is_none() {
            return false;
        }
        if name == editor_key {
            if !repeat {
                toggle_editor(game, &mut s);
            }
            return true;
        }
        if name == picker_key && s.identity.is_some() {
            if !repeat {
                s.editor.picker = !s.editor.picker;
                s.editor.selected = s.selected.clone();
                s.global.profile_picker_tip_seen = true;
                runtime::save_global(&s);
            }
            return true;
        }
        if name == free_key
            && s.identity.is_some()
            && s.first
            && s.config.use_crosshair_for_first_person
        {
            if !repeat {
                s.mouse_free = !s.mouse_free;
            }
            return true;
        }
        if s.editor.dialog.is_some() {
            match name {
                "key.keyboard.escape" => s.editor.dialog = None,
                "key.keyboard.enter" | "key.keyboard.keypad.enter" => {
                    if game.modifiers.shift_key() {
                        if let Some(Dialog::List { text, .. }) = &mut s.editor.dialog {
                            text.push('\n');
                        }
                    } else {
                        action(game, &mut s, "dialog_ok");
                    }
                }
                "key.keyboard.backspace" => s.editor.backspace(),
                "key.keyboard.a" if game.modifiers.control_key() => {
                    if let Some(Dialog::Rename { text, .. } | Dialog::List { text, .. }) =
                        &mut s.editor.dialog
                    {
                        text.clear();
                    }
                }
                _ => {
                    if let Some(text) = text {
                        s.editor.text(text);
                    }
                }
            }
            return true;
        }
        if !s.editor.active && !s.editor.picker {
            return false;
        }
        if name == "key.keyboard.escape" {
            if s.editor.picker {
                s.editor.picker = false;
            } else {
                runtime::save_layout(&mut s);
                s.editor.end();
            }
            return true;
        }
        if name == "key.keyboard.space" {
            s.editor.preview = true;
            return true;
        }
        if s.editor.picker {
            return true;
        }
        if game.modifiers.control_key() {
            match name {
                "key.keyboard.z" => {
                    let Session { editor, config, .. } = &mut *s;
                    editor.undo(config);
                    runtime::save_layout(&mut s);
                    return true;
                }
                "key.keyboard.y" => {
                    let Session { editor, config, .. } = &mut *s;
                    editor.redo(config);
                    runtime::save_layout(&mut s);
                    return true;
                }
                _ => {}
            }
        }
        match name {
            "key.keyboard.v" => action(game, &mut s, "view"),
            "key.keyboard.g" => s.editor.tool = Tool::Translate,
            "key.keyboard.r" => s.editor.tool = Tool::Rotate,
            "key.keyboard.s" => s.editor.tool = Tool::Scale,
            "key.keyboard.c" => action(game, &mut s, "curve"),
            "key.keyboard.left"
            | "key.keyboard.right"
            | "key.keyboard.up"
            | "key.keyboard.down"
            | "key.keyboard.equal"
            | "key.keyboard.minus"
            | "key.keyboard.page.up"
            | "key.keyboard.page.down" => {
                let snapshot = s.config.clone();
                s.editor.checkpoint(&snapshot);
                let first = s.first;
                let c = &mut s.config;
                match name {
                    "key.keyboard.left" => {
                        if first {
                            c.first_person_screen_side_offset -= 0.05
                        } else {
                            c.screen_side_offset -= 0.05
                        }
                    }
                    "key.keyboard.right" => {
                        if first {
                            c.first_person_screen_side_offset += 0.05
                        } else {
                            c.screen_side_offset += 0.05
                        }
                    }
                    "key.keyboard.up" => {
                        if first {
                            c.first_person_screen_height_offset += 0.05
                        } else {
                            c.screen_height_offset += 0.05
                        }
                    }
                    "key.keyboard.down" => {
                        if first {
                            c.first_person_screen_height_offset -= 0.05
                        } else {
                            c.screen_height_offset -= 0.05
                        }
                    }
                    "key.keyboard.equal" => {
                        if first {
                            c.first_person_screen_scale += 0.05
                        } else {
                            c.screen_scale += 0.05
                        }
                    }
                    "key.keyboard.minus" => {
                        if first {
                            c.first_person_screen_scale -= 0.05
                        } else {
                            c.screen_scale -= 0.05
                        }
                    }
                    "key.keyboard.page.up" => {
                        if first {
                            c.first_person_screen_distance -= 0.05
                        } else {
                            c.screen_distance -= 0.05
                        }
                    }
                    _ => {
                        if first {
                            c.first_person_screen_distance += 0.05
                        } else {
                            c.screen_distance += 0.05
                        }
                    }
                }
                let value = s.config.value();
                if let Ok(c) = Config::from_value(value) {
                    s.config = c;
                }
                runtime::save_layout(&mut s);
            }
            _ => return false,
        }
        true
    });
    if open_options {
        game.capture(false);
        game.open_options_at(false, horizon_client::options_ui::Page::Mods);
    }
    claimed
}

/// Runs before the ordinary screen sees an event, including book and sign
/// editors which don't use the cancellable screen-key hook.
pub(super) fn input(game: &mut Game<'_>, event: &mut WindowEvent) -> bool {
    runtime::load(game);
    if !game.native_on(ID) {
        return false;
    }
    if settings_input(game, event) {
        return true;
    }
    if let WindowEvent::KeyboardInput { event: key, .. } = event {
        let PhysicalKey::Code(code) = key.physical_key else {
            return false;
        };
        let Some(name) = horizon_client::keybinds::key_name(code) else {
            return false;
        };
        let down = key.state == ElementState::Pressed;
        return key_event(game, name, down, key.repeat, key.text.as_deref());
    }
    if let WindowEvent::Ime(Ime::Commit(text)) = event {
        return STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.editor.dialog.is_some() {
                s.editor.text(text);
                true
            } else {
                false
            }
        });
    }
    let size = game
        .renderer
        .as_ref()
        .map(|r| [r.config.width, r.config.height]);
    let Some(size) = size else {
        return false;
    };
    let batch = HudBatch::new(size);
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            let raw = Vec2::new(
                position.x as f32 / batch.scale,
                position.y as f32 / batch.scale,
            );
            let synthetic = game.pad_synthetic_input();
            STATE.with(|cell| {
                let mut s = cell.borrow_mut();
                if synthetic && !s.editor.active && !s.editor.picker {
                    s.logical_navigation = true;
                    return false;
                }
                if !synthetic {
                    s.logical_navigation = false;
                    s.raw_pixels = Some(Vec2::new(position.x as f32, position.y as f32));
                }
                let screen_raw = s.surface.map_or(raw, |surface| {
                    Vec2::new(
                        position.x as f32 / size[0] as f32 * surface.gui[0],
                        position.y as f32 / size[1] as f32 * surface.gui[1],
                    )
                });
                s.raw = Some(screen_raw);
                let overlay = Vec2::new(
                    position.x as f32 / s.hud_scale,
                    position.y as f32 / s.hud_scale,
                );
                s.editor.cursor = overlay;
                if (s.editor.active && !s.editor.preview) || s.editor.picker {
                    let first = s.first;
                    let Session { editor, config, .. } = &mut *s;
                    editor.drag_to(overlay, config, first);
                    return true;
                }
                if s.identity.is_some() {
                    if let (Some(surface), Some(camera)) = (s.surface, s.camera) {
                        let point = if s.first
                            && s.config.use_crosshair_for_first_person
                            && game.keyboard_mouse
                            && !s.mouse_free
                        {
                            Vec2::from_array(surface.gui) * 0.5
                        } else {
                            screen_raw
                        };
                        let mapped = surface
                            .intersect(
                                camera.position,
                                runtime::ray(camera, point, surface.gui, s.fov),
                            )
                            .unwrap_or([-10000.0, -10000.0]);
                        position.x = f64::from(mapped[0] * batch.scale);
                        position.y = f64::from(mapped[1] * batch.scale);
                    }
                }
                false
            })
        }
        WindowEvent::MouseWheel { delta, .. } => STATE.with(|cell| {
            let mut s = cell.borrow_mut();
            if !s.editor.active && !s.editor.picker {
                return false;
            }
            let amount = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y,
                MouseScrollDelta::PixelDelta(p) => p.y as f32 / 30.0,
            };
            let first = s.first;
            let Session { editor, config, .. } = &mut *s;
            editor.scroll(amount, config, first);
            runtime::save_layout(&mut s);
            true
        }),
        WindowEvent::MouseInput { state, button, .. } => STATE.with(|cell| {
            let mut s = cell.borrow_mut();
            if *state == ElementState::Pressed && s.surface.is_some() {
                s.click = Some(Instant::now());
            }
            if (!s.editor.active || s.editor.preview) && !s.editor.picker {
                return false;
            }
            if *state == ElementState::Released {
                if s.editor.drag.take().is_some() {
                    runtime::save_layout(&mut s);
                }
                return true;
            }
            let cursor = s.editor.cursor;
            if *button == MouseButton::Left {
                if let Some(control) = s
                    .editor
                    .controls
                    .iter()
                    .find(|c| c.contains(cursor))
                    .cloned()
                {
                    if let Some((key, ..)) = control.slider {
                        let snapshot = s.config.clone();
                        s.editor.checkpoint(&snapshot);
                        s.editor.drag = Some(Drag::Slider { key });
                        let first = s.first;
                        let Session { editor, config, .. } = &mut *s;
                        editor.drag_to(cursor, config, first);
                    } else {
                        action(game, &mut s, &control.id);
                    }
                    return true;
                }
                if let Some(axis) = s.editor.hit_axis(cursor) {
                    let snapshot = s.config.clone();
                    s.editor.checkpoint(&snapshot);
                    s.editor.drag = Some(Drag::Axis {
                        axis,
                        origin: cursor,
                        start: Box::new(snapshot),
                        tool: s.editor.tool,
                    });
                    return true;
                }
            } else if *button == MouseButton::Middle {
                let snapshot = s.config.clone();
                s.editor.checkpoint(&snapshot);
                s.editor.drag = Some(Drag::Move { last: cursor });
            } else if *button == MouseButton::Right {
                s.editor.drag = Some(Drag::Orbit { last: cursor });
            }
            true
        }),
        _ => false,
    }
}

pub(super) fn relative(game: &Game<'_>, event: &DeviceEvent) {
    if !game.focused && game.control.is_none() {
        return;
    }
    let DeviceEvent::MouseMotion { delta: (x, y) } = event else {
        return;
    };
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        if !game.native_on(ID)
            || s.identity.is_none()
            || !s.first
            || !s.config.use_crosshair_for_first_person
            || s.mouse_free
            || s.editor.active
            || s.editor.picker
        {
            return;
        }
        let gain = (game.settings.sensitivity as f32 * 0.6 + 0.2).powi(3);
        let c = s.config.clone();
        s.look.x += *x as f32
            * gain
            * if game.settings.file.bool("invertXMouse") {
                -1.0
            } else {
                1.0
            };
        if c.lock_first_person_yaw {
            s.look.x = s.look.x.clamp(-90.0, 90.0);
        }
        s.look.y += *y as f32
            * gain
            * if game.settings.file.bool("invertYMouse") {
                -1.0
            } else {
                1.0
            };
        s.look.y = s
            .look
            .y
            .clamp(-90.0 - s.anchor_pitch, 90.0 - s.anchor_pitch);
    });
}

/// Only scalar settings and the category selector consume the wheel/right click.
/// All other areas retain Horizon's normal list scrolling and key rebinding.
fn settings_input(game: &mut Game<'_>, event: &WindowEvent) -> bool {
    use horizon_client::options_ui::Page;
    let Page::NativeMod(index) = game.ui.options.page() else {
        STATE.with(|cell| cell.borrow_mut().settings_wheel = None);
        return false;
    };
    if game.screen != Screen::Options
        || STATE.with(|cell| {
            let s = cell.borrow();
            s.editor.active || s.editor.picker || s.editor.dialog.is_some()
        })
    {
        STATE.with(|cell| cell.borrow_mut().settings_wheel = None);
        return false;
    }
    let wheel = match event {
        WindowEvent::MouseWheel { delta, .. } => Some(match delta {
            MouseScrollDelta::LineDelta(_, y) => f64::from(*y),
            MouseScrollDelta::PixelDelta(p) => p.y / 30.0,
        }),
        WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Right,
            ..
        } => None,
        _ => return false,
    };
    let size = game.gui_size();
    let button = game
        .ui
        .options
        .spatial_gui_setting_at(&game.settings, size, game.cursor);
    let Some(button) = button else {
        STATE.with(|cell| cell.borrow_mut().settings_wheel = None);
        return false;
    };
    let adjustable = STATE.with(|cell| {
        let s = cell.borrow();
        button == 0
            || button
                .checked_sub(4)
                .and_then(|i| {
                    SETTINGS
                        .iter()
                        .filter(|spec| spec.category == CATEGORIES[s.category % CATEGORIES.len()])
                        .nth(i)
                })
                .is_some_and(|spec| spec.kind != "list")
    });
    if !adjustable {
        STATE.with(|cell| cell.borrow_mut().settings_wheel = None);
        return false;
    }
    let steps = STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        let Some(amount) = wheel else {
            s.settings_wheel = None;
            return -1;
        };
        if !amount.is_finite() || amount == 0.0 {
            return 0;
        }
        // Keep fractional trackpad deltas, but never carry them to another
        // control or across a change in scrolling direction.
        let previous = s
            .settings_wheel
            .filter(|(target, value)| *target == button && value.signum() == amount.signum())
            .map_or(0.0, |(_, value)| value);
        let total = previous + amount;
        let steps = total.trunc() as i32;
        s.settings_wheel = Some((button, total.fract()));
        steps
    });
    if steps != 0 {
        for _ in 0..steps.unsigned_abs() {
            settings_step(game, button, f64::from(steps.signum()));
        }
        game.native_settings_refresh(index);
    }
    true
}

pub(super) fn settings(game: &Game<'_>) -> Vec<String> {
    runtime::load(game);
    STATE.with(|cell| {
        let s = cell.borrow();
        let category = CATEGORIES[s.category % CATEGORIES.len()];
        let mut out = vec![
            format!("Category: {category}"),
            "Open visual editor".into(),
            "Manage profiles".into(),
            "Reload JSON5 configuration and profiles".into(),
        ];
        out.extend(
            SETTINGS
                .iter()
                .filter(|spec| spec.category == category)
                .map(|spec| s.global.label(spec)),
        );
        out
    })
}
pub(super) fn settings_click(game: &mut Game<'_>, button: usize) {
    STATE.with(|cell| cell.borrow_mut().settings_wheel = None);
    settings_step(game, button, 1.0);
}

fn settings_step(game: &mut Game<'_>, button: usize, direction: f64) {
    runtime::load(game);
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        match button {
            0 => {
                let step = if direction < 0.0 {
                    CATEGORIES.len() - 1
                } else {
                    1
                };
                s.category = (s.category + step) % CATEGORIES.len();
            }
            1 => toggle_editor(game, &mut s),
            2 => {
                if game.player.is_some() {
                    game.screen = Screen::Inventory;
                    game.capture(false);
                    s.editor.picker = true;
                    s.editor.selected = s.selected.clone();
                }
            }
            3 => runtime::reload(&mut s),
            _ => {
                let category = CATEGORIES[s.category % CATEGORIES.len()];
                if let Some(spec) = SETTINGS
                    .iter()
                    .filter(|spec| spec.category == category)
                    .nth(button - 4)
                {
                    if spec.kind == "list" {
                        let value = s.global.value();
                        let text = value[spec.key]
                            .as_array()
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str())
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            })
                            .unwrap_or_default();
                        game.screen = Screen::Inventory;
                        game.capture(false);
                        let c = s.config.clone();
                        s.editor.begin(&c);
                        s.editor.dialog = Some(Dialog::List {
                            key: spec.key.into(),
                            text,
                        });
                    } else {
                        s.global.edit(spec.key, direction);
                        s.config = s.global.clone();
                        runtime::save_global(&s);
                    }
                }
            }
        }
    });
}

fn line(game: &Game<'_>, batch: &mut HudBatch, a: Vec2, b: Vec2, color: u32, width: f32) {
    let n = (b - a).length().ceil().max(1.0) as usize;
    for i in 0..=n {
        let p = a.lerp(b, i as f32 / n as f32);
        game.native_fill(
            batch,
            [p.x - width * 0.5, p.y - width * 0.5, width, width],
            color,
        );
    }
}
pub(super) fn paint(game: &Game<'_>, batch: &mut HudBatch, s: &mut Session) {
    if !s.editor.active && !s.editor.picker && s.editor.dialog.is_none() {
        return;
    }
    let c = s.config.clone();
    let profiles = s.profiles.clone();
    s.editor
        .layout(&c, &profiles, [batch.width, batch.height], s.first);
    if s.editor.preview {
        game.native_text(
            batch,
            "Release Space to return to the editor",
            [8.0, 8.0],
            0xffffff,
            true,
        );
        return;
    }
    if s.editor.picker {
        game.native_fill(
            batch,
            [
                (batch.width - 262.0) * 0.5,
                10.0,
                262.0,
                batch.height - 20.0,
            ],
            0xe820_1818,
        );
    } else {
        game.native_fill(batch, [0.0, 0.0, batch.width, 20.0], 0xd000_0000);
        game.native_text(
            batch,
            "Spatial GUI | G move / R rotate / S scale | Space preview | F8 save",
            [5.0, 6.0],
            0xffffff,
            true,
        );
    }
    for control in &s.editor.controls {
        let kind = if let Some((_, lo, hi, value)) = &control.slider {
            horizon_client::widgets::Kind::Slider {
                value: ((value - lo) / (hi - lo)).clamp(0.0, 1.0),
                track: [control.rect[0], control.rect[2]],
            }
        } else {
            horizon_client::widgets::Kind::Button
        };
        horizon_client::widgets::record_as(&control.text, control.rect, true, kind);
        horizon_client::widgets::target_as(control.rect, kind);
        horizon_client::widgets::narratable(|| {
            horizon_client::widgets::Narratable::button(control.text.clone(), control.rect, true)
        });
        let hover = control.contains(s.editor.cursor);
        game.native_fill(
            batch,
            control.rect,
            if hover { 0xee50_5040 } else { 0xdd28_2828 },
        );
        if let Some((_, lo, hi, value)) = &control.slider {
            let t = ((value - lo) / (hi - lo)).clamp(0.0, 1.0);
            game.native_fill(
                batch,
                [
                    control.rect[0] + t * (control.rect[2] - 3.0),
                    control.rect[1] + 14.0,
                    3.0,
                    5.0,
                ],
                0xff60_c8ff,
            );
        }
        game.native_text(
            batch,
            &control.text,
            [control.rect[0] + 3.0, control.rect[1] + 4.0],
            0xffffff,
            true,
        );
    }
    if let Some(dialog) = &s.editor.dialog {
        let title = match dialog {
            Dialog::Rename { .. } => "Rename profile (Ctrl+A replaces text)",
            Dialog::List { key, .. } => key,
        };
        let text = match dialog {
            Dialog::Rename { text, .. } | Dialog::List { text, .. } => text,
        };
        let x = batch.width * 0.5 - 130.0;
        let y = batch.height * 0.5 - 70.0;
        game.native_fill(batch, [x, y, 260.0, 85.0], 0xf020_2020);
        game.native_text(batch, title, [x + 5.0, y + 6.0], 0xffffff, true);
        let capacity = 46;
        let rows: Vec<String> = text
            .lines()
            .flat_map(|row| {
                row.chars()
                    .collect::<Vec<_>>()
                    .chunks(capacity)
                    .map(|part| part.iter().collect::<String>())
                    .collect::<Vec<_>>()
            })
            .collect();
        for (i, row) in rows.iter().take(5).enumerate() {
            game.native_text(
                batch,
                row,
                [x + 5.0, y + 21.0 + i as f32 * 10.0],
                0xffe0a0,
                true,
            );
        }
        let field = [x + 4.0, y + 18.0, 252.0, 55.0];
        horizon_client::widgets::record_as(
            "Profile text",
            field,
            true,
            horizon_client::widgets::Kind::Field,
        );
        horizon_client::widgets::target_as(field, horizon_client::widgets::Kind::Field);
        horizon_client::widgets::narratable(|| {
            horizon_client::widgets::Narratable::edit_box(title, text, field)
        });
    } else if s.editor.active && !s.editor.picker {
        if let Some(center) = s.editor.center {
            for (i, end) in s.editor.axes.iter().enumerate() {
                let Some(end) = end else {
                    continue;
                };
                let color = [0xff50_50ff, 0xff50_ff50, 0xffff_9050][i];
                line(game, batch, center, *end, color, 2.0);
                game.native_fill(batch, [end.x - 4.0, end.y - 4.0, 8.0, 8.0], color);
            }
            game.native_fill(
                batch,
                [center.x - 3.0, center.y - 3.0, 6.0, 6.0],
                0xffff_ffff,
            );
            if s.editor.tool == Tool::Rotate {
                if let (Some(surface), Some((camera, view, _))) = (s.surface, gpu::view()) {
                    for axis in 0..3 {
                        let (a, b) = match axis {
                            0 => (Vec3::Y, Vec3::Z),
                            1 => (Vec3::X, Vec3::Z),
                            _ => (Vec3::X, Vec3::Y),
                        };
                        let mut previous = None;
                        for i in 0..=64 {
                            let angle = i as f32 / 64.0 * std::f32::consts::TAU;
                            let p = surface.center
                                + (surface.rotation * (a * angle.cos() + b * angle.sin()) * 0.35)
                                    .as_dvec3();
                            let point =
                                surface.project(p, camera, view, [batch.width, batch.height]);
                            if let (Some(from), Some(to)) = (previous, point) {
                                line(
                                    game,
                                    batch,
                                    from,
                                    to,
                                    [0xff50_50ff, 0xff50_ff50, 0xffff_9050][axis],
                                    1.0,
                                );
                            }
                            previous = point;
                        }
                    }
                }
            }
        }
        game.native_text(
            batch,
            "Wheel: size | Middle drag: move | Right drag: orbit | Ctrl+Z/Y: undo/redo",
            [5.0, batch.height - 12.0],
            0xffffff,
            true,
        );
    }
    if !s.editor.notice.is_empty() {
        game.native_text(
            batch,
            &s.editor.notice,
            [8.0, batch.height - 24.0],
            0xffd080,
            true,
        );
    }
}

// Native controller and control-socket key paths have no WindowEvent. Queue their
// mutations until the next frame, outside the read-only screen callback borrow.
pub(super) fn screen_key(game: &Game<'_>, name: &str, repeat: bool, canceled: &mut bool) {
    runtime::load(game);
    if !game.native_on(ID) {
        return;
    }
    let bindings = [
        binding(game, "editor", "key.keyboard.f8"),
        binding(game, "profiles", "key.keyboard.grave.accent"),
        binding(game, "config", "key.keyboard.f7"),
        binding(game, "reload", "key.keyboard.f6"),
        binding(game, "mouse_free", "key.keyboard.tab"),
    ];
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        if bindings.iter().any(|b| b == name)
            || s.editor.active
            || s.editor.picker
            || s.editor.dialog.is_some()
        {
            s.pending_keys.push((name.into(), true, repeat));
            *canceled = true;
        }
    });
}
pub(super) fn released(game: &Game<'_>, key: winit::keyboard::KeyCode, pressed: bool) {
    if pressed || !game.native_on(ID) {
        return;
    }
    if horizon_client::keybinds::key_name(key) == Some("key.keyboard.space") {
        STATE.with(|cell| {
            let mut s = cell.borrow_mut();
            if s.editor.active {
                s.pending_keys
                    .push(("key.keyboard.space".into(), false, false));
            }
        });
    }
}
pub(super) fn flush(game: &mut Game<'_>) {
    let pending = STATE.with(|cell| std::mem::take(&mut cell.borrow_mut().pending_keys));
    for (name, down, repeat) in pending {
        key_event(game, &name, down, repeat, None);
    }
}
