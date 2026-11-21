fn spatial_begin_options(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::spatial_compat::refresh(game); crate::game::spatial_integration::begin_screen(game,batch,"options");
}
