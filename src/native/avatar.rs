// Avatar yaw uses the player's world heading. Head yaw in Horizon is absolute;
// upstream's avatarBaseYawOffset is relative to the posed body, not part of it.
use super::spatial_config::Config;

pub fn pose(
    config: &Config,
    anchor_yaw: f32,
    cursor: [f32; 2],
    flying_pitch: Option<f32>,
) -> [f32; 3] {
    let side = if config.mirror_third_person {
        -1.0
    } else {
        1.0
    };
    let head_direction = if config.mirror_head_movement {
        -1.0
    } else {
        1.0
    };
    let body = anchor_yaw + side * config.avatar_body_rotation_offset;
    let head = body
        + side * config.avatar_base_yaw_offset
        + cursor[0].clamp(-1.0, 1.0) * head_direction * config.avatar_max_yaw_offset as f32;
    let pitch = flying_pitch.unwrap_or_else(|| {
        (cursor[1].clamp(-1.0, 1.0) * config.avatar_max_pitch_offset as f32).clamp(-60.0, 60.0)
    });
    [body, head, pitch]
}
