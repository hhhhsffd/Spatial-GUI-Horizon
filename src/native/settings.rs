fn spatial_settings(value: Result<()>, game: &mut crate::game::Game<'_>) -> Result<()> {
    crate::game::spatial_compat::refresh(game); value
}
