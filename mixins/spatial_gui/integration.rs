// Complete native lifecycle, configuration, camera and world-screen extraction.
use super::{
    Game, Screen,
    perspective::CameraType,
    spatial_config::{Config, Profiles},
    spatial_editor::Editor,
};
use glam::{DVec3, Quat, Vec2, Vec3};
use horizon_client::{
    first_person::Hand,
    hud::{
        HudBatch,
        spatial_gui::{self as gpu, Presentation, Surface},
    },
};
use std::{cell::RefCell, fs, path::PathBuf, sync::atomic::Ordering, time::Instant};
pub(super) const ID: &str = "spatial_gui";
pub(super) const VERSION: &str = "0.2.1";
#[derive(Clone, Copy, Debug)]
pub(super) struct Camera {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}
impl Camera {
    fn interpolate(self, to: Self, t: f32) -> Self {
        Self {
            position: self.position.lerp(to.position, f64::from(t)),
            yaw: self.yaw + wrap(to.yaw - self.yaw) * t,
            pitch: self.pitch + (to.pitch - self.pitch) * t,
        }
    }
}
#[derive(Clone, Debug)]
pub(super) struct Identity {
    pub key: String,
    pub id: String,
    pub alias: String,
    pub category: String,
}
pub(super) struct Session {
    pub loaded: bool,
    pub pending_keys: Vec<(String, bool, bool)>,
    pub global: Config,
    pub config: Config,
    pub profiles: Profiles,
    pub identity: Option<Identity>,
    pub selected: Option<String>,
    pub editor: Editor,
    pub opened: Option<Instant>,
    pub closing: Option<Instant>,
    pub start: Option<Camera>,
    pub camera: Option<Camera>,
    pub close_start: Option<Camera>,
    pub anchor_yaw: f32,
    pub anchor_pitch: f32,
    pub look: Vec2,
    pub parallax: Vec2,
    pub raw: Option<Vec2>,
    pub raw_pixels: Option<Vec2>,
    pub mouse_free: bool,
    pub first: bool,
    pub fallback: bool,
    pub saved_camera: Option<CameraType>,
    pub saved_gui: Option<u32>,
    pub surface: Option<Surface>,
    pub presentation: Option<Presentation>,
    pub frame: u64,
    pub fov: f32,
    pub updated: Instant,
    pub click: Option<Instant>,
    pub end: Option<u32>,
    pub capture_start: Option<u32>,
    pub category: usize,
    pub file: PathBuf,
    pub profiles_file: PathBuf,
    pub reload_notice: String,
    pub captured_by_mod: bool,
    pub recipe_delay: u8,
    pub logical_navigation: bool,
    pub camera_passthrough: bool,
    pub in_screen: bool,
    pub hud_scale: f32,
    pub screen_scale: f32,
    pub hud_gui: [f32; 2],
}
impl Default for Session {
    fn default() -> Self {
        Self {
            loaded: false,
            pending_keys: Vec::new(),
            global: Config::default(),
            config: Config::default(),
            profiles: Profiles::default(),
            identity: None,
            selected: None,
            editor: Editor::default(),
            opened: None,
            closing: None,
            start: None,
            camera: None,
            close_start: None,
            anchor_yaw: 0.0,
            anchor_pitch: 0.0,
            look: Vec2::ZERO,
            parallax: Vec2::ZERO,
            raw: None,
            raw_pixels: None,
            mouse_free: false,
            first: false,
            fallback: false,
            saved_camera: None,
            saved_gui: None,
            surface: None,
            presentation: None,
            frame: 0,
            fov: 70.0,
            updated: Instant::now(),
            click: None,
            end: None,
            capture_start: None,
            category: 0,
            file: PathBuf::new(),
            profiles_file: PathBuf::new(),
            reload_notice: String::new(),
            captured_by_mod: false,
            recipe_delay: 0,
            logical_navigation: false,
            camera_passthrough: false,
            in_screen: false,
            hud_scale: 1.0,
            screen_scale: 1.0,
            hud_gui: [320.0, 240.0],
        }
    }
}
thread_local! {pub(super) static STATE:RefCell<Session>=RefCell::new(Session::default());}
pub(super) fn wrap(v: f32) -> f32 {
    (v + 180.0).rem_euclid(360.0) - 180.0
}
pub(super) fn load(_game: &Game<'_>) {
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        if s.loaded {
            return;
        }
        s.loaded = true;
        let dir = Game::native_config_dir().unwrap_or_else(|_| PathBuf::from("config"));
        s.file = dir.join("spatial-gui.json5");
        s.profiles_file = dir.join("spatial-gui/profiles.json5");
        reload(&mut s);
        if !s.file.exists() {
            save_global(&s);
        }
        if !s.profiles_file.exists() {
            save_profiles(&s);
        }
    });
}
pub(super) fn reload(s: &mut Session) {
    if let Ok(text) = fs::read_to_string(&s.file) {
        match Config::parse(&text) {
            Ok(c) => s.global = c,
            Err(e) => {
                log::warn!("Spatial GUI config: {e}");
                s.reload_notice = e;
            }
        }
    }
    if let Ok(text) = fs::read_to_string(&s.profiles_file) {
        match Profiles::parse(&text) {
            Ok(p) => s.profiles = p,
            Err(e) => {
                log::warn!("Spatial GUI profiles: {e}");
                s.reload_notice = e;
            }
        }
    }
    s.config = s.global.clone();
    s.reload_notice = "Configuration and profiles reloaded".into();
}
fn write(path: &std::path::Path, value: &impl serde::Serialize) {
    let Some(parent) = path.parent() else {
        return;
    };
    if let Err(e) = fs::create_dir_all(parent).and_then(|_| {
        let text = serde_json::to_string_pretty(value).map_err(std::io::Error::other)?;
        let temp = path.with_extension("json5.tmp");
        fs::write(&temp, text + "\n")?;
        fs::rename(temp, path)
    }) {
        log::warn!("Spatial GUI could not save {}: {e}", path.display());
    }
}
pub(super) fn save_global(s: &Session) {
    write(&s.file, &s.global);
}
pub(super) fn save_profiles(s: &Session) {
    write(&s.profiles_file, &s.profiles);
}
pub(super) fn save_layout(s: &mut Session) {
    if let Some(name) = s.selected.clone() {
        s.profiles.update(&name, &s.config);
        if s.profiles.linked.as_deref() == Some(&name) {
            s.global = s.config.clone();
            save_global(s);
        }
        save_profiles(s);
    } else {
        s.global = s.config.clone();
        save_global(s);
    }
}
pub(super) fn identity(game: &Game<'_>) -> Option<Identity> {
    if game.player.is_none() || game.title.is_some() || game.remote_loading() {
        return None;
    }
    let mut id = game.screen.id().to_owned();
    let mut alias = String::new();
    let menu = game
        .remote_menu_type()
        .filter(|m| !m.is_empty())
        .unwrap_or("")
        .to_owned();
    if let Some(book) = &game.book {
        id = "book".into();
        alias = format!(
            "net.minecraft.client.gui.screens.inventory.{}",
            match book {
                super::books::BookScreen::Edit { signing: true, .. } => "BookSignScreen",
                super::books::BookScreen::Edit { .. } => "BookEditScreen",
                super::books::BookScreen::View {
                    lectern: Some(_), ..
                } => "LecternScreen",
                _ => "BookViewScreen",
            }
        );
    } else if game.sign_edit.is_some() {
        id = "sign".into();
        alias = "net.minecraft.client.gui.screens.inventory.SignEditScreen".into();
    } else if game.code_of_conduct.is_some() {
        id = "code_of_conduct".into();
    } else if game.link_prompt.is_some() {
        id = "link_prompt".into();
    } else if game.command_prompt.is_some() {
        id = "command_prompt".into();
    } else if game.dialog_open() {
        id = "dialog".into();
    } else if game.server_packs.prompt.is_some() {
        id = "pack_confirm".into();
    } else if id == "playing" {
        return None;
    }
    let category = if id == "paused" {
        "#pause"
    } else if (id == "inventory" && menu.is_empty()) || id == "creative" {
        "#inventory"
    } else if !menu.is_empty() {
        "#containers"
    } else {
        "#other"
    };
    if alias.is_empty() {
        let class = match menu.as_str() {
            "minecraft:crafting" => "CraftingScreen",
            "minecraft:furnace" => "FurnaceScreen",
            "minecraft:smoker" => "SmokerScreen",
            "minecraft:blast_furnace" => "BlastFurnaceScreen",
            "minecraft:anvil" => "AnvilScreen",
            "minecraft:enchantment" => "EnchantmentScreen",
            "minecraft:beacon" => "BeaconScreen",
            "minecraft:brewing_stand" => "BrewingStandScreen",
            "minecraft:merchant" => "MerchantScreen",
            "minecraft:shulker_box" => "ShulkerBoxScreen",
            "minecraft:hopper" => "HopperScreen",
            "minecraft:generic_3x3" => "DispenserScreen",
            "minecraft:grindstone" => "GrindstoneScreen",
            "minecraft:smithing" => "SmithingScreen",
            "minecraft:cartography_table" => "CartographyTableScreen",
            "minecraft:loom" => "LoomScreen",
            "minecraft:stonecutter" => "StonecutterScreen",
            m if m.starts_with("minecraft:generic_9x") => "ContainerScreen",
            _ => match id.as_str() {
                "inventory" => "InventoryScreen",
                "creative" => "CreativeModeInventoryScreen",
                "paused" => "PauseScreen",
                "options" => "OptionsScreen",
                _ => "",
            },
        };
        alias = if class.is_empty() {
            id.clone()
        } else if id == "paused" || id == "options" {
            format!("net.minecraft.client.gui.screens.{class}")
        } else {
            format!("net.minecraft.client.gui.screens.inventory.{class}")
        };
    }
    let key = if let Some(owner) = game.native_mod_screen() {
        format!("mod:{owner}")
    } else if menu.is_empty() {
        id.clone()
    } else {
        format!("{id}:{menu}")
    };
    Some(Identity {
        key,
        id,
        alias,
        category: category.into(),
    })
}
fn allowed(c: &Config, id: &Identity) -> bool {
    let contains = |list: &[String]| {
        list.iter().any(|v| {
            v == &id.key
                || v == &id.id
                || v == &id.alias
                || id.key.strip_prefix("inventory:") == Some(v.as_str())
        })
    };
    c.enabled
        && !contains(&c.disabled_screens)
        && match id.category.as_str() {
            "#inventory" => c.inventory,
            "#containers" => c.containers,
            "#pause" => c.pause_screen,
            _ => c.all_screens || contains(&c.enabled_screens),
        }
}
fn eye(game: &Game<'_>) -> Option<DVec3> {
    let p = game.player.as_ref()?;
    Some(DVec3::new(
        p.position.x,
        p.position.y + f64::from(p.dimensions.eye_height),
        p.position.z,
    ))
}
fn forward(yaw: f32, pitch: f32) -> Vec3 {
    let (sy, cy) = yaw.to_radians().sin_cos();
    let (sp, cp) = pitch.to_radians().sin_cos();
    Vec3::new(-sy * cp, -sp, cy * cp)
}
fn natural_camera(game: &Game<'_>) -> Option<Camera> {
    let p = game.native_camera();
    Some(Camera {
        position: p
            .as_ref()
            .map_or(eye(game)?, |p| DVec3::from_array(p.position)),
        yaw: p.as_ref().map_or(game.yaw, |p| p.yaw),
        pitch: p.as_ref().map_or(game.pitch, |p| p.pitch),
    })
}
pub(super) fn frame(game: &mut Game<'_>) {
    load(game);
    let observed = identity(game);
    let natural = natural_camera(game);
    let native_on = game.native_on(ID);
    let now = Instant::now();
    let mut capture = None;
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        let dt = now.duration_since(s.updated).as_secs_f32().min(0.1);
        s.updated = now;
        s.frame = s.frame.wrapping_add(1);
        s.end = None;
        s.capture_start = None;
        let mut observed = observed;
        if let Some(id) = &observed {
            if !s.global.seen_screens.contains(&id.alias) {
                s.global.seen_screens.push(id.alias.clone());
                save_global(&s);
            }
            let (effective, name) = s
                .profiles
                .resolve(&id.key, &id.alias, &id.category, &s.global);
            if !s.editor.active && !s.editor.picker {
                s.config = effective;
                s.selected = name;
            }
            if !native_on || !allowed(&s.config, id) {
                observed = None;
            }
        }
        if observed.is_none() && s.identity.is_some() {
            if s.config.keep_first_person_camera_angle && s.first {
                if let Some(camera) = s.camera {
                    game.yaw = camera.yaw;
                    game.pitch = camera.pitch.clamp(-90.0, 90.0);
                }
            }
            if s.config.enable_close_animation && s.presentation.is_some() {
                s.closing = Some(now);
                s.close_start = s.camera;
            }
            s.identity = None;
            s.editor.end();
            s.mouse_free = false;
            if s.captured_by_mod {
                capture = Some(game.screen == Screen::Playing && !game.screen_over_playing());
                s.captured_by_mod = false;
            }
        }
        if let Some(id) = observed {
            let opening = s.identity.is_none();
            let changed = s
                .identity
                .as_ref()
                .is_none_or(|previous| previous.key != id.key);
            if opening {
                let resuming = s.closing.is_some();
                s.opened = Some(now);
                s.closing = None;
                s.start = natural;
                if !resuming {
                    s.anchor_yaw = game.yaw;
                    s.anchor_pitch = game.pitch;
                    s.look = Vec2::ZERO;
                    s.saved_camera = Some(game.camera_type);
                    s.saved_gui = Some(horizon_client::hud::GUI_SCALE.load(Ordering::Relaxed));
                }
                s.fov = game
                    .native_camera()
                    .map_or(game.settings.fov as f32, |c| c.fov_degrees);
                if s.raw.is_none() {
                    s.raw = Some(Vec2::new(game.cursor.0, game.cursor.1));
                }
            }
            if changed {
                let (c, name) = s
                    .profiles
                    .resolve(&id.key, &id.alias, &id.category, &s.global);
                if opening && s.editor.active && s.editor.undo.is_empty() {
                    s.editor.initial = Some(c.clone());
                }
                s.config = c;
                s.selected = name;
                s.editor.selected = s.selected.clone();
            }
            s.identity = Some(id);
        }
        if s.identity.is_none() && s.closing.is_none() {
            reset(game, &mut s);
            return;
        }
        if game.player.is_none() || !native_on {
            reset(game, &mut s);
            return;
        }
        let c = s.config.clone();
        let original = s.saved_camera.unwrap_or(game.camera_type);
        s.first = if c.auto_detect_camera_mode {
            original.first_person()
        } else {
            c.view_mode == "FIRST_PERSON"
        };
        if (s.first && c.disable_first_person) || (!s.first && c.disable_third_person) {
            reset(game, &mut s);
            return;
        }
        let Some(eye) = eye(game) else {
            reset(game, &mut s);
            return;
        };
        let elapsed = s
            .opened
            .map_or(0.0, |t| now.duration_since(t).as_secs_f32() * 1000.0);
        let transition = (elapsed / c.transition_duration_ms.max(1) as f32).clamp(0.0, 1.0);
        let transition = c.transition_skip_percentage as f32 * 0.01
            + (1.0 - c.transition_skip_percentage as f32 * 0.01)
                * (1.0 - (1.0 - transition).powi(3));
        let close = s
            .closing
            .map_or(0.0, |t| now.duration_since(t).as_secs_f32() * 1000.0);
        let target_fov = if c.override_fov {
            c.target_fov as f32
        } else {
            game.settings.fov as f32
        };
        s.fov += (target_fov - s.fov) * (1.0 - (-dt * c.fov_lerp_speed.max(1) as f32).exp());
        let crosshair = s.first
            && c.use_crosshair_for_first_person
            && game.keyboard_mouse
            && !s.logical_navigation
            && !s.mouse_free
            && !s.editor.active
            && !s.editor.picker;
        let want_capture = crosshair && !game.pad_synthetic_input() && game.focused;
        if s.identity.is_some()
            && (want_capture != s.captured_by_mod || (want_capture && !game.captured))
        {
            capture = Some(want_capture);
            s.captured_by_mod = want_capture;
        }
        let raw = s
            .raw
            .unwrap_or_else(|| Vec2::new(game.cursor.0, game.cursor.1));
        let (w, h) = game.gui_size();
        let cursor_delta = raw - Vec2::new(w, h) * 0.5;
        let normal = Vec2::new(cursor_delta.x / (w * 0.5), cursor_delta.y / (h * 0.5))
            .clamp(Vec2::splat(-1.0), Vec2::ONE);
        let target_parallax = if s.editor.active && !s.editor.preview {
            Vec2::ZERO
        } else if s.first {
            if crosshair {
                s.look
            } else if c.disable_first_person_parallax {
                Vec2::ZERO
            } else {
                Vec2::new(
                    normal.x * 90.0 * c.first_person_mouse_sensitivity_yaw,
                    normal.y * 180.0 * c.first_person_mouse_sensitivity_pitch,
                )
            }
        } else if c.disable_third_person_parallax {
            Vec2::ZERO
        } else {
            Vec2::new(
                normal.x * 90.0 * c.third_person_mouse_sensitivity_yaw,
                normal.y * 180.0 * c.third_person_mouse_sensitivity_pitch,
            )
        };
        s.parallax = if crosshair {
            target_parallax
        } else {
            s.parallax.lerp(target_parallax, 1.0 - (-dt / 0.05).exp())
        };
        let parallax = s.parallax;
        s.fallback = false;
        let mut camera = if s.first {
            Camera {
                position: eye,
                yaw: s.anchor_yaw + parallax.x,
                pitch: (s.anchor_pitch.clamp(
                    -c.first_person_pitch_clamp as f32,
                    c.first_person_pitch_clamp as f32,
                ) + parallax.y)
                    .clamp(-90.0, 90.0),
            }
        } else {
            let yaw = s.anchor_yaw.to_radians();
            let mirror = if c.mirror_third_person { -1.0 } else { 1.0 };
            let ratio = if c.auto_scale_by_fov {
                (s.fov / c.auto_scale_baseline_fov.max(1.0)).max(0.1)
            } else {
                1.0
            };
            let distance = (c.camera_distance
                * (1.0 + (1.0 / ratio - 1.0) * c.auto_scale_distance_multiplier))
                .clamp(-4.0, 4.0);
            let side = (c.camera_side_offset
                * (1.0 + (1.0 / ratio - 1.0) * c.auto_scale_side_offset_multiplier))
                .clamp(-4.0, 4.0)
                * mirror;
            let base = game.player.as_ref().unwrap().position;
            let flying = game.player.as_ref().is_some_and(|p| p.fall_flying);
            let side = side
                + if flying {
                    c.elytra_flying_side_offset
                } else {
                    0.0
                };
            let height = c.camera_height_offset
                + if flying {
                    c.elytra_flying_height_offset
                } else {
                    0.0
                };
            let target = DVec3::new(
                base.x + f64::from(yaw.sin() * distance + yaw.cos() * side),
                base.y + f64::from(height),
                base.z + f64::from(-yaw.cos() * distance + yaw.sin() * side),
            );
            let (position, blocked) = collision(game, eye, target);
            s.fallback = blocked && c.fallback_to_first_person_on_block_collision;
            if s.fallback {
                Camera {
                    position: eye,
                    yaw: s.anchor_yaw + parallax.x,
                    pitch: s.anchor_pitch + parallax.y,
                }
            } else {
                Camera {
                    position,
                    yaw: s.anchor_yaw + parallax.x,
                    pitch: if c.lerp_camera_pitch {
                        s.anchor_pitch
                            + (c.camera_target_pitch - s.anchor_pitch) * transition
                            + parallax.y
                    } else {
                        c.camera_target_pitch + parallax.y
                    },
                }
            }
        };
        if s.fallback {
            s.first = true;
        }
        if s.editor.active {
            let yaw = Quat::from_rotation_y(-s.editor.orbit.x.to_radians());
            let right = Vec3::new(
                -s.anchor_yaw.to_radians().cos(),
                0.0,
                -s.anchor_yaw.to_radians().sin(),
            );
            let pitch = Quat::from_axis_angle(right, -s.editor.orbit.y.to_radians());
            camera.position = eye + (yaw * pitch * (camera.position - eye).as_vec3()).as_dvec3();
            camera.yaw += s.editor.orbit.x;
            camera.pitch = (camera.pitch + s.editor.orbit.y).clamp(-89.0, 89.0);
        }
        camera = s.start.unwrap_or(camera).interpolate(camera, transition);
        if s.closing.is_some() {
            let progress = (close / c.transition_duration_ms.max(1) as f32).clamp(0.0, 1.0);
            camera = s.close_start.unwrap_or(camera).interpolate(
                Camera {
                    position: eye,
                    yaw: game.yaw,
                    pitch: game.pitch,
                },
                1.0 - (1.0 - progress).powi(3),
            );
            if close
                >= c.transition_duration_ms
                    .max(c.fade_duration_ms)
                    .max(c.open_animation_duration_ms) as f32
            {
                reset(game, &mut s);
                return;
            }
        }
        s.camera = Some(camera);
        game.camera_type = if s.first {
            CameraType::FirstPerson
        } else {
            CameraType::ThirdPersonBack
        };
        if c.override_gui_scale {
            horizon_client::hud::GUI_SCALE.store(c.gui_scale.clamp(1, 6) as u32, Ordering::Relaxed);
        } else if let Some(original) = s.saved_gui {
            horizon_client::hud::GUI_SCALE.store(original, Ordering::Relaxed);
        }
        let (w, h) = game.gui_size();
        if let (Some(pixels), Some(renderer)) = (s.raw_pixels, &game.renderer) {
            s.raw = Some(Vec2::new(
                pixels.x / renderer.config.width as f32 * w,
                pixels.y / renderer.config.height as f32 * h,
            ));
        }
        let first = s.first;
        let (distance, mut side, mut height, yaw_offset, pitch_offset, base_scale) = if first {
            (
                c.first_person_screen_distance,
                c.first_person_screen_side_offset,
                c.first_person_screen_height_offset,
                c.first_person_screen_yaw_offset,
                c.first_person_screen_pitch_offset,
                c.first_person_screen_scale,
            )
        } else {
            (
                c.screen_distance,
                c.screen_side_offset,
                c.screen_height_offset,
                c.screen_yaw_offset,
                c.screen_pitch_offset,
                c.screen_scale,
            )
        };
        if !first && game.player.as_ref().is_some_and(|p| p.fall_flying) {
            side += c.elytra_flying_side_offset;
            height += c.elytra_flying_height_offset;
        }
        if !first && c.mirror_third_person {
            side = -side;
        }
        let yaw = s.anchor_yaw.to_radians();
        let pitch = if first {
            s.anchor_pitch.clamp(
                -c.first_person_pitch_clamp as f32,
                c.first_person_pitch_clamp as f32,
            )
        } else {
            0.0
        };
        let offset = forward(s.anchor_yaw, pitch) * distance
            + Vec3::new(yaw.cos() * side, height, yaw.sin() * side);
        let screen_yaw = if !first && c.mirror_third_person {
            -yaw_offset
        } else {
            yaw_offset
        };
        let rotation = Quat::from_rotation_y((screen_yaw - s.anchor_yaw).to_radians())
            * Quat::from_rotation_x((pitch_offset - pitch).to_radians());
        let ratio = if c.auto_scale_by_fov {
            (s.fov / c.auto_scale_baseline_fov.max(1.0)).powf(c.auto_scale_screen_power)
        } else {
            1.0
        };
        let fov_scale = if first {
            ratio
        } else {
            1.0 + (ratio - 1.0) * c.auto_scale_third_person_screen_multiplier
        };
        let progress = (elapsed / c.open_animation_duration_ms.max(1) as f32).clamp(0.0, 1.0);
        let mut scale = base_scale * fov_scale;
        if c.enable_scale_animation {
            scale *= c.animation_start_scale_percent as f32 * 0.01
                + (1.0 - c.animation_start_scale_percent as f32 * 0.01) * c.easing(progress);
        }
        if game.recipe_book_shift(w) != 0.0 || game.book_covers_menu(w) {
            s.recipe_delay = 2;
        } else {
            s.recipe_delay = s.recipe_delay.saturating_sub(1);
        }
        if s.recipe_delay > 0 {
            scale /= c.recipe_book_shrink_factor.max(1.0);
        }
        if game.panel.shown(|id| game.native_on(id)).is_some() {
            scale *= c.recipe_viewer_scale_multiplier;
        }
        if s.closing.is_some() && c.enable_scale_animation {
            scale *= 1.0
                - (1.0 - c.animation_start_scale_percent as f32 * 0.01)
                    * c.easing(
                        (close / c.open_animation_duration_ms.max(1) as f32).clamp(0.0, 1.0),
                    );
        }
        s.surface = Some(Surface {
            center: eye + offset.as_dvec3(),
            rotation,
            width: scale * w / h,
            height: scale,
            arc: if first && c.curved_screen_enabled {
                (c.curved_screen_arc_degrees as f32).to_radians()
            } else {
                0.0
            },
            gui: [w, h],
        });
        if s.identity.is_some() && !s.logical_navigation {
            let mouse = if crosshair {
                Vec2::new(w, h) * 0.5
            } else {
                s.raw.unwrap_or(Vec2::new(w, h) * 0.5)
            };
            let mapped = s
                .surface
                .unwrap()
                .intersect(camera.position, ray(camera, mouse, [w, h], s.fov))
                .unwrap_or([-10000.0, -10000.0]);
            game.cursor = (mapped[0], mapped[1]);
        }
        if !s.first {
            let flying = game
                .player
                .as_ref()
                .is_some_and(|player| player.fall_flying);
            if let Some(model) = game.player_model.as_mut() {
                let [body, head, pitch] = super::spatial_avatar::pose(
                    &c,
                    s.anchor_yaw,
                    normal.to_array(),
                    flying.then_some(model.head_pitch),
                );
                model.yaw = body;
                model.previous_yaw = body;
                model.body_yaw = body;
                model.previous_body_yaw = body;
                model.head_yaw = head;
                model.previous_head_yaw = model.head_yaw;
                model.head_pitch = pitch;
                model.previous_head_pitch = model.head_pitch;
                if c.arm_swing {
                    let t = s
                        .click
                        .map_or(1.0, |at| at.elapsed().as_secs_f32() / 0.3)
                        .clamp(0.0, 1.0);
                    model.anim.attack_anim = t;
                    model.anim.attack_anim_o = t;
                }
            }
        }
    });
    if let Some(capture) = capture {
        if capture != game.captured {
            game.capture(capture);
        }
    }
}
fn reset(game: &mut Game<'_>, s: &mut Session) {
    if s.captured_by_mod && game.captured {
        game.capture(game.screen == Screen::Playing && !game.screen_over_playing());
    }
    if let Some(camera) = s.saved_camera.take() {
        game.camera_type = camera;
    }
    if let Some(scale) = s.saved_gui.take() {
        horizon_client::hud::GUI_SCALE.store(scale, Ordering::Relaxed);
    }
    if s.surface.take().is_some() {
        if let Some(raw) = s.raw {
            game.cursor = (raw.x, raw.y);
        }
    }
    s.identity = None;
    s.opened = None;
    s.closing = None;
    s.camera = None;
    s.presentation = None;
    s.editor.end();
    s.captured_by_mod = false;
    gpu::publish(None);
}
pub(super) fn ray(camera: Camera, point: Vec2, size: [f32; 2], fov: f32) -> Vec3 {
    let forward = forward(camera.yaw, camera.pitch);
    let yaw = camera.yaw.to_radians();
    let right = Vec3::new(-yaw.cos(), 0.0, -yaw.sin());
    let up = right.cross(forward).normalize_or_zero();
    let x = point.x / size[0] * 2.0 - 1.0;
    let y = 1.0 - point.y / size[1] * 2.0;
    let tan = (fov.to_radians() * 0.5).tan();
    (forward + right * x * tan * size[0] / size[1] + up * y * tan).normalize_or_zero()
}
pub(super) fn camera(game: &mut Game<'_>, partial: f32) -> Option<(DVec3, f32, f32)> {
    let desired = STATE.with(|cell| {
        let s = cell.borrow();
        if s.camera_passthrough {
            return None;
        }
        Some((
            s.camera?,
            s.closing,
            s.close_start,
            s.saved_camera,
            s.config.transition_duration_ms,
        ))
    })?;
    let (mut c, closing, start, original, duration) = desired;
    if let Some(closed) = closing {
        STATE.with(|s| s.borrow_mut().camera_passthrough = true);
        let current = game.camera_type;
        if let Some(original) = original {
            game.camera_type = original;
        }
        let normal = game.camera_view(partial);
        game.camera_type = current;
        STATE.with(|s| s.borrow_mut().camera_passthrough = false);
        if let Some((position, yaw, pitch)) = normal {
            let t =
                (closed.elapsed().as_secs_f32() * 1000.0 / duration.max(1) as f32).clamp(0.0, 1.0);
            c = start.unwrap_or(c).interpolate(
                Camera {
                    position,
                    yaw,
                    pitch,
                },
                1.0 - (1.0 - t).powi(3),
            );
        }
    }
    game.view.position = c.position.to_array();
    game.view.set_rotation(c.yaw, c.pitch);
    Some((c.position, c.yaw, c.pitch))
}
pub(super) fn orientation() -> Option<Quat> {
    STATE.with(|s| {
        s.borrow().camera.map(|c| {
            Quat::from_euler(
                glam::EulerRot::YXZ,
                std::f32::consts::PI - c.yaw.to_radians(),
                -c.pitch.to_radians(),
                0.0,
            )
        })
    })
}
pub(super) fn pauses(game: &Game<'_>) -> bool {
    STATE.with(|s| {
        let s = s.borrow();
        s.editor.active
            && s.config.editor_pauses_game
            && game.has_singleplayer_server()
            && !game.is_published()
    })
}
fn collision(game: &Game<'_>, eye: DVec3, target: DVec3) -> (DVec3, bool) {
    let delta = target - eye;
    let length = delta.length();
    if length < 0.001 {
        return (target, false);
    }
    let direction = delta / length;
    let mut nearest = length;
    for corner in 0..8 {
        let offset = DVec3::new(
            if corner & 1 == 0 { -0.2 } else { 0.2 },
            if corner & 2 == 0 { -0.2 } else { 0.2 },
            if corner & 4 == 0 { -0.2 } else { 0.2 },
        );
        let from = eye + offset;
        let to = target + offset;
        let lo = from.min(to).floor().as_ivec3();
        let hi = from.max(to).floor().as_ivec3();
        for x in lo.x..=hi.x {
            for y in lo.y..=hi.y {
                for z in lo.z..=hi.z {
                    let name = game.native_block([x, y, z]);
                    if name.ends_with("glass")
                        || name.ends_with("glass_pane")
                        || name.ends_with("_bars")
                        || name.ends_with("copper_grate")
                        || name == "minecraft:powder_snow"
                    {
                        continue;
                    }
                    for shape in game.native_block_collision([x, y, z]) {
                        let min = DVec3::new(
                            shape[0] + f64::from(x),
                            shape[1] + f64::from(y),
                            shape[2] + f64::from(z),
                        );
                        let max = DVec3::new(
                            shape[3] + f64::from(x),
                            shape[4] + f64::from(y),
                            shape[5] + f64::from(z),
                        );
                        let mut entry = 0.0_f64;
                        let mut exit = length;
                        for axis in 0..3 {
                            if direction[axis].abs() < 1e-9 {
                                if from[axis] < min[axis] || from[axis] > max[axis] {
                                    exit = -1.0;
                                    break;
                                }
                            } else {
                                let a = (min[axis] - from[axis]) / direction[axis];
                                let b = (max[axis] - from[axis]) / direction[axis];
                                entry = entry.max(a.min(b));
                                exit = exit.min(a.max(b));
                            }
                        }
                        if exit >= entry && exit >= 0.0 {
                            nearest = nearest.min((entry - 0.05).max(0.0));
                        }
                    }
                }
            }
        }
    }
    (eye + direction * nearest, nearest + 0.01 < length)
}
pub(super) fn begin_hud(game: &Game<'_>) {
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        s.in_screen = false;
        if s.surface.is_none() {
            return;
        }
        let Some(renderer) = &game.renderer else {
            return;
        };
        horizon_client::hud::GUI_SCALE.store(
            s.saved_gui.unwrap_or(game.settings.gui_scale),
            Ordering::Relaxed,
        );
        let hud = HudBatch::new([renderer.config.width, renderer.config.height]);
        s.hud_scale = hud.scale;
        s.hud_gui = [hud.width, hud.height];
    });
}
pub(super) fn begin_screen(game: &Game<'_>, batch: &mut HudBatch, id: &str) {
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        if s.in_screen
            || s.closing.is_some()
            || s.identity.as_ref().is_none_or(|screen| screen.id != id)
        {
            return;
        }
        let Some(renderer) = &game.renderer else {
            return;
        };
        let requested = if s.config.override_gui_scale {
            s.config.gui_scale.clamp(1, 6) as u32
        } else {
            s.saved_gui.unwrap_or(game.settings.gui_scale)
        };
        horizon_client::hud::GUI_SCALE.store(requested, Ordering::Relaxed);
        let screen = HudBatch::new([renderer.config.width, renderer.config.height]);
        let ratio = batch.scale / screen.scale;
        for v in &mut batch.vertices {
            v.pos[0] *= ratio;
            v.pos[1] *= ratio;
        }
        batch.scale = screen.scale;
        batch.width = screen.width;
        batch.height = screen.height;
        s.screen_scale = screen.scale;
        s.capture_start = Some(batch.vertices.len() as u32);
        s.in_screen = true;
    });
}
pub(super) fn screen_drawn(batch: &mut HudBatch) {
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        if !s.in_screen {
            return;
        }
        let ratio = batch.scale / s.hud_scale;
        for v in &mut batch.vertices {
            v.pos[0] *= ratio;
            v.pos[1] *= ratio;
        }
        batch.scale = s.hud_scale;
        batch.width = s.hud_gui[0];
        batch.height = s.hud_gui[1];
        s.in_screen = false;
        s.end = Some(batch.vertices.len() as u32);
    });
}
pub(super) fn capture(game: &Game<'_>, batch: &mut HudBatch) {
    screen_drawn(batch);
    STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        let (Some(surface), Some(_)) = (s.surface, s.camera) else {
            return;
        };
        let c = s.config.clone();
        let closing = s.closing.is_some();
        let start = s
            .capture_start
            .unwrap_or(batch.blur_at)
            .min(batch.vertices.len() as u32);
        let end = s
            .end
            .unwrap_or(batch.vertices.len() as u32)
            .min(batch.vertices.len() as u32);
        if !closing {
            for vertices in batch.vertices[start as usize..end as usize].chunks_exact_mut(6) {
                let fullscreen = vertices.iter().all(|v| {
                    (v.pos[0].abs() < 0.1 || (v.pos[0] - batch.width).abs() < 0.1)
                        && (v.pos[1].abs() < 0.1 || (v.pos[1] - batch.height).abs() < 0.1)
                });
                if fullscreen {
                    for v in vertices {
                        v.color = 0;
                    }
                }
            }
            if c.hide_hud || c.hide_hotbar {
                for vertex in &mut batch.vertices[..start as usize] {
                    if c.hide_hud || vertex.pos[1] >= batch.height - 24.0 {
                        vertex.color = 0;
                    }
                }
            }
        }
        let elapsed = s
            .opened
            .map_or(0.0, |at| at.elapsed().as_secs_f32() * 1000.0);
        let close = s
            .closing
            .map_or(0.0, |at| at.elapsed().as_secs_f32() * 1000.0);
        let fade = if c.enable_fade_animation {
            if closing {
                1.0 - (close / c.fade_duration_ms.max(1) as f32).clamp(0.0, 1.0)
            } else {
                (elapsed / c.fade_duration_ms.max(1) as f32).clamp(0.0, 1.0)
            }
        } else {
            1.0
        };
        let tint = if c.enable_world_lighting_tint {
            let p = surface.center.floor().as_ivec3();
            let light = game.world.light(p.x, p.y, p.z);
            let index = ((light >> 4) as usize * 16 + (light & 15) as usize) * 4;
            if let Some(renderer) = &game.renderer {
                std::array::from_fn(|i| {
                    1.0 + (f32::from(renderer.lightmap_pixels[index + i]) / 255.0 - 1.0)
                        * (c.lighting_tint_strength / 100.0)
                })
            } else {
                [1.0; 3]
            }
        } else {
            [1.0; 3]
        };
        let player_id = game.player.as_ref().map_or(0, |p| p.entity_id as u32);
        let mut presentation = Presentation {
            frame: s.frame,
            start,
            end,
            surface,
            opacity: c.screen_alpha as f32 / 255.0 * fade,
            blur: if c.blur_world_background {
                c.world_blur_strength as f32
            } else {
                0.0
            },
            tint,
            render_scale: c.render_scale_percent as f32 / 100.0,
            linear: c.use_linear_filtering,
            anisotropic: c.use_anisotropic_filtering,
            shadow_layers: if c.drop_shadow_enabled {
                c.drop_shadow_layers as u32
            } else {
                0
            },
            shadow_alpha: c.drop_shadow_alpha as f32 / 255.0 / c.drop_shadow_layers.max(1) as f32,
            shadow_scale: c.drop_shadow_scale_multiplier,
            shadow_offset: [
                c.drop_shadow_horizontal_offset,
                c.drop_shadow_vertical_offset,
                c.drop_shadow_depth_offset,
            ],
            player_front: c.render_player_in_front,
            over_player_alpha: c.screen_over_player_alpha as f32 / 255.0,
            player_id,
            closing,
        };
        if closing {
            if let Some(previous) = &s.presentation {
                presentation.start = previous.start;
                presentation.end = previous.end;
            } else {
                return;
            }
        }
        s.presentation = Some(presentation.clone());
        gpu::publish(Some(presentation));
        if s.first
            && c.use_crosshair_for_first_person
            && !s.mouse_free
            && !s.editor.active
            && !s.editor.picker
        {
            game.native_fill(
                batch,
                [batch.width * 0.5 - 3.0, batch.height * 0.5 - 0.5, 6.0, 1.0],
                0xffff_ffff,
            );
            game.native_fill(
                batch,
                [batch.width * 0.5 - 0.5, batch.height * 0.5 - 3.0, 1.0, 6.0],
                0xffff_ffff,
            );
        }
        if let Some((camera, view, _)) = gpu::view() {
            let yaw = s.anchor_yaw.to_radians();
            let pitch = if s.first {
                s.anchor_pitch.clamp(
                    -c.first_person_pitch_clamp as f32,
                    c.first_person_pitch_clamp as f32,
                )
            } else {
                0.0
            };
            let anchor_yaw = s.anchor_yaw;
            s.editor.update_gizmo(
                &surface,
                camera,
                view,
                [batch.width, batch.height],
                [
                    Vec3::new(yaw.cos(), 0.0, yaw.sin()),
                    Vec3::Y,
                    forward(anchor_yaw, pitch),
                ],
            );
        }
        if !s.global.profile_picker_tip_seen
            && !s.editor.active
            && !s.editor.picker
            && elapsed < 6000.0
        {
            game.native_text(
                batch,
                "Press ` to choose a Spatial GUI profile",
                [8.0, batch.height - 12.0],
                0xeeeeee,
                true,
            );
        }
        super::spatial_controls::paint(game, batch, &mut s);
        let requested = if s.editor.active && !s.editor.preview || s.editor.picker {
            s.saved_gui.unwrap_or(game.settings.gui_scale)
        } else if c.override_gui_scale {
            c.gui_scale.clamp(1, 6) as u32
        } else {
            s.saved_gui.unwrap_or(game.settings.gui_scale)
        };
        horizon_client::hud::GUI_SCALE.store(requested, Ordering::Relaxed);
    });
}
pub(super) fn fov(_game: &Game<'_>, degrees: &mut f32) {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.camera.is_some() {
            if s.config.override_fov {
                let close = s.closing.map_or(1.0, |at| {
                    1.0 - (at.elapsed().as_secs_f32() * 1000.0
                        / s.config.transition_duration_ms.max(1) as f32)
                        .clamp(0.0, 1.0)
                });
                *degrees += (s.fov - *degrees) * close;
            } else {
                s.fov = *degrees;
            }
        }
    });
}
pub(super) fn blur(_game: &Game<'_>, vanilla: Option<f32>) -> Option<f32> {
    STATE.with(|s| {
        if s.borrow().surface.is_some() {
            None
        } else {
            vanilla
        }
    })
}
pub(super) fn hide_hand(
    _game: &Game<'_>,
    stack: Option<&horizon_client::items::ItemStack>,
    canceled: &mut bool,
) {
    STATE.with(|s| {
        let s = s.borrow();
        if s.surface.is_some()
            && s.first
            && (s.config.hide_hands_in_first_person
                || (s.config.hide_shield_in_first_person
                    && stack.is_some_and(|stack| &*stack.item == "minecraft:shield")))
        {
            *canceled = true;
        }
    });
}
pub(super) fn hand_pose(_game: &Game<'_>, hand: Hand, z: &mut f32, pitch: &mut f32) {
    STATE.with(|s| {
        let s = s.borrow();
        if s.surface.is_none() || !s.first {
            return;
        }
        let c = &s.config;
        let t = s
            .click
            .map_or(1.0, |at| {
                at.elapsed().as_secs_f32() * 1000.0 / c.hand_poke_duration_ms.max(1) as f32
            })
            .clamp(0.0, 1.0);
        let pulse = (t * std::f32::consts::PI).sin();
        if c.hand_poke_animation {
            *z -= pulse * 0.18 * c.hand_poke_strength;
        }
        if c.arm_swing {
            *pitch -= pulse * 20.0 * if hand == Hand::Main { 1.0 } else { -1.0 };
        }
    });
}
pub(super) fn hide_hud(_game: &Game<'_>, layer: &str, canceled: &mut bool) {
    STATE.with(|s| {
        let s = s.borrow();
        if s.surface.is_some()
            && (s.config.hide_hud || (s.config.hide_hotbar && layer == "minecraft:hotbar"))
        {
            *canceled = true;
        }
    });
}
pub(super) fn pad_cursor(game: &Game<'_>, batch: &mut HudBatch) -> bool {
    STATE.with(|cell| {
        let s = cell.borrow();
        if s.surface.is_none()
            || (!s.logical_navigation && game.keyboard_mouse)
            || s.editor.active
            || s.editor.picker
            || game.pad.menu.osk.is_some()
        {
            return false;
        }
        if let (Some(surface), Some((camera, view, _))) = (s.surface, gpu::view()) {
            let uv = Vec2::new(
                game.cursor.0 / surface.gui[0],
                game.cursor.1 / surface.gui[1],
            );
            if let Some(p) =
                surface.project(surface.point(uv), camera, view, [batch.width, batch.height])
            {
                game.native_fill(batch, [p.x - 4.0, p.y - 1.0, 8.0, 2.0], 0xff00_0000);
                game.native_fill(batch, [p.x - 1.0, p.y - 4.0, 2.0, 8.0], 0xffff_ffff);
            }
        }
        true
    })
}
