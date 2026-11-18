fn spatial_key_release(game: &crate::game::Game<'_>, key: &str) {
    crate::game::spatial_controls::released(game,key,false);
}
