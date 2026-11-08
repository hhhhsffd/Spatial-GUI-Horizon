fn spatial_draw(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::spatial_integration::capture(game,batch);
}
