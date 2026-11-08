// Mouse-operated world-space editor and profile picker. All editing is local;
// the inventory continues to own its items and ordinary screen actions.
use super::spatial_config::{Config, Profiles};
use super::spatial_surface::Surface;
use glam::{DVec3, Mat2, Mat4, Vec2, Vec3};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Translate,
    Rotate,
    Scale,
}
#[derive(Clone, Debug)]
pub enum Drag {
    Slider {
        key: String,
    },
    Axis {
        axis: usize,
        origin: Vec2,
        start: Box<Config>,
        tool: Tool,
    },
    Move {
        last: Vec2,
    },
    Orbit {
        last: Vec2,
    },
}
#[derive(Clone, Debug)]
pub enum Dialog {
    Rename { old: String, text: String },
    List { key: String, text: String },
}
#[derive(Clone, Debug)]
pub struct Control {
    pub id: String,
    pub text: String,
    pub rect: [f32; 4],
    pub slider: Option<(String, f32, f32, f32)>,
}
impl Control {
    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.rect[0]
            && p.y >= self.rect[1]
            && p.x < self.rect[0] + self.rect[2]
            && p.y < self.rect[1] + self.rect[3]
    }
}
#[derive(Clone, Debug)]
pub struct Editor {
    pub active: bool,
    pub picker: bool,
    pub preview: bool,
    pub tool: Tool,
    pub drag: Option<Drag>,
    pub undo: Vec<Config>,
    pub redo: Vec<Config>,
    pub initial: Option<Config>,
    pub dialog: Option<Dialog>,
    pub scope: usize,
    pub scroll: usize,
    pub cursor: Vec2,
    pub orbit: Vec2,
    pub selected: Option<String>,
    pub controls: Vec<Control>,
    pub axes: [Option<Vec2>; 3],
    pub rings: [Vec<Vec2>; 3],
    pub center: Option<Vec2>,
    pub move_axes: [Vec2; 2],
    pub notice: String,
    pub last_scroll: Option<Instant>,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            active: false,
            picker: false,
            preview: false,
            tool: Tool::Translate,
            drag: None,
            undo: Vec::new(),
            redo: Vec::new(),
            initial: None,
            dialog: None,
            scope: 0,
            scroll: 0,
            cursor: Vec2::ZERO,
            orbit: Vec2::ZERO,
            selected: None,
            controls: Vec::new(),
            axes: [None; 3],
            rings: std::array::from_fn(|_| Vec::new()),
            center: None,
            move_axes: [Vec2::new(-100.0, 0.0), Vec2::new(0.0, -100.0)],
            notice: String::new(),
            last_scroll: None,
        }
    }
}
impl Editor {
    pub fn begin(&mut self, c: &Config) {
        self.active = true;
        self.preview = false;
        self.initial = Some(c.clone());
        self.undo.clear();
        self.redo.clear();
        self.drag = None;
        self.orbit = Vec2::ZERO;
    }
    pub fn checkpoint(&mut self, c: &Config) {
        self.last_scroll = None;
        if self.undo.last() != Some(c) {
            self.undo.push(c.clone());
            if self.undo.len() > 100 {
                self.undo.remove(0);
            }
            self.redo.clear();
        }
    }
    pub fn undo(&mut self, c: &mut Config) {
        if let Some(previous) = self.undo.pop() {
            self.redo.push(c.clone());
            *c = previous;
        }
    }
    pub fn redo(&mut self, c: &mut Config) {
        if let Some(next) = self.redo.pop() {
            self.undo.push(c.clone());
            *c = next;
        }
    }
    pub fn end(&mut self) {
        self.active = false;
        self.picker = false;
        self.preview = false;
        self.dialog = None;
        self.drag = None;
    }
    fn button(&mut self, id: &str, text: impl Into<String>, x: f32, y: f32, w: f32) {
        self.controls.push(Control {
            id: id.into(),
            text: text.into(),
            rect: [x, y, w, 16.0],
            slider: None,
        });
    }
    #[allow(clippy::too_many_arguments)]
    fn slider(&mut self, key: &str, text: &str, x: f32, y: f32, w: f32, lo: f32, hi: f32, v: f32) {
        self.controls.push(Control {
            id: format!("slider:{key}"),
            text: format!("{text}: {v:.2}"),
            rect: [x, y, w, 20.0],
            slider: Some((key.into(), lo, hi, v)),
        });
    }
    pub fn layout(&mut self, c: &Config, profiles: &Profiles, gui: [f32; 2], first: bool) {
        self.controls.clear();
        let [w, h] = gui;
        if self.picker {
            let x = (w - 250.0) * 0.5;
            let y = 20.0;
            let capacity = ((h - 92.0) / 18.0).max(1.0) as usize;
            self.button(
                "scope",
                format!(
                    "Assign to: {}",
                    ["this screen", "screen category", "all screens"][self.scope]
                ),
                x,
                y,
                250.0,
            );
            self.button(
                "unassign",
                "Use inherited/default layout",
                x,
                y + 18.0,
                250.0,
            );
            for (i, name) in profiles
                .profiles
                .keys()
                .skip(self.scroll)
                .take(capacity)
                .enumerate()
            {
                self.button(
                    &format!("profile:{name}"),
                    format!(
                        "{}{}",
                        if self.selected.as_deref() == Some(name) {
                            "> "
                        } else {
                            ""
                        },
                        name
                    ),
                    x,
                    y + 38.0 + i as f32 * 18.0,
                    250.0,
                );
            }
            for (i, (id, label)) in [
                ("create", "New"),
                ("rename", "Rename"),
                ("delete", "Delete"),
                ("default", "Default"),
                ("picker_close", "Done"),
            ]
            .iter()
            .enumerate()
            {
                self.button(id, *label, x + i as f32 * 50.0, h - 26.0, 48.0);
            }
        } else if self.active && !self.preview {
            let left = [
                ("undo", "Undo (Ctrl+Z)"),
                ("redo", "Redo (Ctrl+Y)"),
                (
                    "view",
                    if first {
                        "Third person (V)"
                    } else {
                        "First person (V)"
                    },
                ),
                ("gui", "GUI scale"),
                ("curve", "Curved screen"),
                ("profiles", "Profiles (`)"),
                ("translate", "Translate (G)"),
                ("rotate", "Rotate (R)"),
                ("scale", "Scale (S)"),
            ];
            for (i, (id, label)) in left.iter().enumerate() {
                self.button(id, *label, 5.0, 24.0 + i as f32 * 18.0, 104.0);
            }
            self.button("undo_all", "Undo all", w - 109.0, 24.0, 104.0);
            self.button("default", "Set default", w - 109.0, 42.0, 104.0);
            self.button("done", "Save and close", w - 109.0, 60.0, 104.0);
            let (yaw, pitch, scale, distance) = if first {
                (
                    c.first_person_screen_yaw_offset,
                    c.first_person_screen_pitch_offset,
                    c.first_person_screen_scale,
                    c.first_person_screen_distance,
                )
            } else {
                (
                    c.screen_yaw_offset,
                    c.screen_pitch_offset,
                    c.screen_scale,
                    c.screen_distance,
                )
            };
            let prefix = if first { "firstPersonScreen" } else { "screen" };
            self.slider(
                &format!("{prefix}YawOffset"),
                "Yaw",
                w - 109.0,
                84.0,
                104.0,
                0.0,
                360.0,
                yaw,
            );
            self.slider(
                &format!("{prefix}PitchOffset"),
                "Pitch",
                w - 109.0,
                107.0,
                104.0,
                -90.0,
                90.0,
                pitch,
            );
            self.slider(
                &format!("{prefix}Scale"),
                "Scale",
                w - 109.0,
                130.0,
                104.0,
                0.1,
                8.0,
                scale,
            );
            self.slider(
                &format!("{prefix}Distance"),
                "Distance",
                w - 109.0,
                153.0,
                104.0,
                0.1,
                6.0,
                distance,
            );
            if first && c.curved_screen_enabled {
                self.slider(
                    "curvedScreenArcDegrees",
                    "Arc",
                    w - 109.0,
                    176.0,
                    104.0,
                    5.0,
                    150.0,
                    c.curved_screen_arc_degrees as f32,
                );
            }
        }
        if self.dialog.is_some() {
            self.controls.clear();
            self.button("dialog_ok", "Save", w * 0.5 - 102.0, h * 0.5 + 18.0, 100.0);
            self.button(
                "dialog_cancel",
                "Cancel",
                w * 0.5 + 2.0,
                h * 0.5 + 18.0,
                100.0,
            );
        }
    }
    pub fn update_gizmo(
        &mut self,
        surface: &Surface,
        camera: DVec3,
        view: Mat4,
        size: [f32; 2],
        axes: [Vec3; 3],
    ) {
        self.center = surface.project(surface.center, camera, view, size);
        self.axes = axes.map(|axis| {
            surface.project(
                surface.center + (axis * 0.45).as_dvec3(),
                camera,
                view,
                size,
            )
        });
        if let Some(center) = self.center {
            for (i, endpoint) in self.axes.iter().take(2).enumerate() {
                if let Some(endpoint) = endpoint {
                    self.move_axes[i] = (*endpoint - center) / 0.45;
                }
            }
        }
        self.rings = std::array::from_fn(|axis| {
            let (a, b) = match axis {
                0 => (Vec3::Y, Vec3::Z),
                1 => (Vec3::X, Vec3::Z),
                _ => (Vec3::X, Vec3::Y),
            };
            (0..=64)
                .filter_map(|i| {
                    let angle = i as f32 / 64.0 * std::f32::consts::TAU;
                    surface.project(
                        surface.center
                            + (surface.rotation * (a * angle.cos() + b * angle.sin()) * 0.35)
                                .as_dvec3(),
                        camera,
                        view,
                        size,
                    )
                })
                .collect()
        });
    }
    pub fn hit_axis(&self, p: Vec2) -> Option<usize> {
        if self.tool == Tool::Rotate {
            return self
                .rings
                .iter()
                .enumerate()
                .flat_map(|(axis, ring)| ring.iter().map(move |point| (axis, point.distance(p))))
                .filter(|(_, d)| *d < 7.0)
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(axis, _)| axis);
        }
        let center = self.center?;
        self.axes
            .iter()
            .enumerate()
            .filter_map(|(i, end)| {
                let end = (*end)?;
                let line = end - center;
                let t = ((p - center).dot(line) / line.length_squared().max(1.0)).clamp(0.0, 1.0);
                let d = (p - (center + line * t)).length();
                (d < 7.0).then_some((i, d))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|v| v.0)
    }
    pub fn drag_to(&mut self, p: Vec2, c: &mut Config, first: bool) {
        let Some(drag) = self.drag.clone() else {
            return;
        };
        let (side, height, distance, scale, yaw, pitch) = if first {
            (
                "firstPersonScreenSideOffset",
                "firstPersonScreenHeightOffset",
                "firstPersonScreenDistance",
                "firstPersonScreenScale",
                "firstPersonScreenYawOffset",
                "firstPersonScreenPitchOffset",
            )
        } else {
            (
                "screenSideOffset",
                "screenHeightOffset",
                "screenDistance",
                "screenScale",
                "screenYawOffset",
                "screenPitchOffset",
            )
        };
        let mut value = c.value();
        match drag {
            Drag::Slider { key, .. } => {
                if let Some(control) = self
                    .controls
                    .iter()
                    .find(|control| control.id == format!("slider:{key}"))
                    && let Some((_, lo, hi, _)) = &control.slider
                {
                    let t = ((p.x - control.rect[0]) / control.rect[2]).clamp(0.0, 1.0);
                    let n = lo + (hi - lo) * t;
                    value[&key] = if key == "curvedScreenArcDegrees" {
                        serde_json::Value::from(n.round() as i64)
                    } else {
                        n.into()
                    };
                }
            }
            Drag::Axis {
                axis,
                origin,
                start,
                tool,
            } => {
                let delta = p - origin;
                let base = start.value();
                match tool {
                    Tool::Translate => {
                        let key = [side, height, distance][axis];
                        let projected = self.axes[axis]
                            .zip(self.center)
                            .map(|(end, center)| (end - center) / 0.45)
                            .unwrap_or(Vec2::X);
                        value[key] = (base[key].as_f64().unwrap_or(0.0)
                            + f64::from(
                                delta.dot(projected) / projected.length_squared().max(1.0),
                            ))
                        .into();
                    }
                    Tool::Rotate => {
                        let key = if axis == 0 { pitch } else { yaw };
                        value[key] = (base[key].as_f64().unwrap_or(0.0)
                            + f64::from(delta.x - delta.y))
                        .into();
                    }
                    Tool::Scale => {
                        value[scale] = (base[scale].as_f64().unwrap_or(1.0)
                            * f64::from(((delta.x - delta.y) * 0.01).exp()))
                        .into();
                    }
                }
            }
            Drag::Move { last } => {
                let delta = p - last;
                let projection = Mat2::from_cols(self.move_axes[0], self.move_axes[1]);
                let delta = if projection.determinant().abs() > 0.001 {
                    projection.inverse() * delta
                } else {
                    Vec2::ZERO
                };
                value[side] = (value[side].as_f64().unwrap_or(0.0) + f64::from(delta.x)).into();
                value[height] = (value[height].as_f64().unwrap_or(0.0) + f64::from(delta.y)).into();
                self.drag = Some(Drag::Move { last: p });
            }
            Drag::Orbit { last } => {
                let delta = p - last;
                self.orbit += delta * 0.5;
                self.orbit.y = self.orbit.y.clamp(-80.0, 80.0);
                self.drag = Some(Drag::Orbit { last: p });
            }
        }
        if let Ok(updated) = Config::from_value(value) {
            *c = updated;
        }
    }
    pub fn scroll(&mut self, amount: f32, c: &mut Config, first: bool) {
        if self.picker {
            self.scroll = (self.scroll as i32 - amount.round() as i32).max(0) as usize;
            return;
        }
        if self
            .last_scroll
            .is_none_or(|at| at.elapsed().as_millis() > 400)
        {
            self.checkpoint(c);
        }
        self.last_scroll = Some(Instant::now());
        let mut value = c.value();
        let key = if first {
            "firstPersonScreenScale"
        } else {
            "screenScale"
        };
        value[key] = (value[key].as_f64().unwrap_or(1.0) * f64::from((amount * 0.08).exp())).into();
        if let Ok(next) = Config::from_value(value) {
            *c = next;
        }
    }
    pub fn text(&mut self, text: &str) {
        if let Some(Dialog::Rename { text: input, .. } | Dialog::List { text: input, .. }) =
            &mut self.dialog
        {
            for ch in text.chars().filter(|c| !c.is_control()) {
                if input.len() < 8192 {
                    input.push(ch);
                }
            }
        }
    }
    pub fn backspace(&mut self) {
        if let Some(Dialog::Rename { text, .. } | Dialog::List { text, .. }) = &mut self.dialog {
            text.pop();
        }
    }
}
