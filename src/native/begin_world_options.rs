fn spatial_begin_world_options(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::spatial_integration::begin_screen(game,batch,"world_options");
}
