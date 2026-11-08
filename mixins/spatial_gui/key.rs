fn spatial_key(game: &crate::game::Game<'_>, key: &str, repeat: bool, canceled: &mut bool) {
    crate::game::spatial_controls::screen_key(game,key,repeat,canceled);
}
