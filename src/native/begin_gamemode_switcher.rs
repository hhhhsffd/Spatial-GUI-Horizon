fn spatial_begin_gamemode_switcher(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::spatial_integration::begin_screen(game,batch,"game_mode_switcher");
}
