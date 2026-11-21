fn spatial_camera(game: &mut crate::game::Game<'_>, partial: f32) -> core::ops::ControlFlow<Option<(glam::DVec3,f32,f32)>> {
    if let Some(camera) = crate::game::spatial_integration::camera(game,partial) { core::ops::ControlFlow::Break(Some(camera)) } else { core::ops::ControlFlow::Continue(()) }
}
