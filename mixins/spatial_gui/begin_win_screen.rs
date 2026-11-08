fn spatial_begin_win_screen(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::spatial_integration::begin_screen(game,batch,"win");
}
