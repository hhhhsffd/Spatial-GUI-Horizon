// The camera, panel and rendered avatar must use the same partial tick.
use glam::DVec3;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    /// Translate the transition's starting point with the player before easing.
    /// An absolute world-space start makes a running/falling player outrun it.
    pub fn interpolate(self, to: Self, t: f32, movement: DVec3) -> Self {
        let yaw = (to.yaw - self.yaw + 180.0).rem_euclid(360.0) - 180.0;
        Self {
            position: (self.position + movement).lerp(to.position, f64::from(t)),
            yaw: self.yaw + yaw * t,
            pitch: self.pitch + (to.pitch - self.pitch) * t,
        }
    }
}

pub fn player_position(previous: [f64; 3], current: [f64; 3], partial: f64) -> DVec3 {
    DVec3::from_array(previous).lerp(DVec3::from_array(current), partial.clamp(0.0, 1.0))
}
