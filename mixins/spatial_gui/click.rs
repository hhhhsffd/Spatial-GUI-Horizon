fn spatial_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    if mod_id == crate::game::spatial_integration::ID { crate::game::spatial_controls::settings_click(game,button); }
}
