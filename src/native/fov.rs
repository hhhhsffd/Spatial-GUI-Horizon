fn spatial_fov(game: &crate::game::Game<'_>, degrees: &mut f32) {
    crate::game::spatial_integration::fov(game,degrees);
}
