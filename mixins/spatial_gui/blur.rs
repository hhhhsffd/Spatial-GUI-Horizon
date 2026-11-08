fn spatial_blur(game: &crate::game::Game<'_>, value: Option<f32>) -> Option<f32> {
    crate::game::spatial_integration::blur(game,value)
}
