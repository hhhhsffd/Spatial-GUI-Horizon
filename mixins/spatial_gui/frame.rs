fn spatial_frame(game: &mut crate::game::Game<'_>) {
    crate::game::spatial_controls::flush(game); crate::game::spatial_integration::frame(game);
}
