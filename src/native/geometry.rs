// Shared world-surface geometry for rendering, gizmos and ray-based input.
use glam::{DVec3, Mat4, Quat, Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Surface {
    pub center: DVec3,
    pub rotation: Quat,
    pub width: f32,
    pub height: f32,
    pub arc: f32,
    pub gui: [f32; 2],
}
impl Surface {
    pub fn point(&self, uv: Vec2) -> DVec3 {
        let x = (uv.x - 0.5) * self.width;
        let y = (0.5 - uv.y) * self.height;
        let local = if self.arc.abs() > 0.001 {
            let r = self.width / self.arc;
            let a = (uv.x - 0.5) * self.arc;
            Vec3::new(r * a.sin(), y, r * (1.0 - a.cos()))
        } else {
            Vec3::new(x, y, 0.0)
        };
        self.center + (self.rotation * local).as_dvec3()
    }
    pub fn intersect(&self, eye: DVec3, direction: Vec3) -> Option<[f32; 2]> {
        let inverse = self.rotation.inverse();
        let o = inverse * (eye - self.center).as_vec3();
        let d = inverse * direction;
        let (t, x) = if self.arc.abs() > 0.001 {
            let r = self.width / self.arc;
            let a = d.x * d.x + d.z * d.z;
            let b = 2.0 * (o.x * d.x + (o.z - r) * d.z);
            let c = o.x * o.x + (o.z - r).powi(2) - r * r;
            let disc = b * b - 4.0 * a * c;
            if a < 1e-8 || disc < 0.0 {
                return None;
            }
            let mut result = None;
            for t in [
                (-b - disc.sqrt()) / (2.0 * a),
                (-b + disc.sqrt()) / (2.0 * a),
            ] {
                if t <= 0.0 {
                    continue;
                }
                let p = o + d * t;
                let angle = (p.x / r).atan2(1.0 - p.z / r);
                if angle.abs() <= self.arc.abs() * 0.5 + 0.05 {
                    result = Some((t, angle * r));
                    break;
                }
            }
            result?
        } else {
            if d.z.abs() < 1e-6 {
                return None;
            }
            let t = -o.z / d.z;
            (t, o.x + d.x * t)
        };
        if t <= 0.0 {
            return None;
        }
        let y = o.y + d.y * t;
        let uv = Vec2::new(x / self.width + 0.5, 0.5 - y / self.height);
        Some([uv.x * self.gui[0], uv.y * self.gui[1]])
    }
    pub fn project(
        &self,
        point: DVec3,
        camera: DVec3,
        view_proj: Mat4,
        size: [f32; 2],
    ) -> Option<Vec2> {
        let p = view_proj * (point - camera).as_vec3().extend(1.0);
        if p.w <= 0.001 {
            return None;
        }
        Some(Vec2::new(
            (p.x / p.w + 1.0) * size[0] * 0.5,
            (1.0 - p.y / p.w) * size[1] * 0.5,
        ))
    }
}
