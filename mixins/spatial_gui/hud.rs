fn spatial_hud(game: &crate::game::Game<'_>, layer: &str, canceled: &mut bool) {
    crate::game::spatial_integration::hide_hud(game,layer,canceled);
}
