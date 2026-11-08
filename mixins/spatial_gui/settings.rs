fn spatial_settings(game: &crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id == crate::game::spatial_integration::ID { buttons.extend(crate::game::spatial_controls::settings(game)); }
}
