fn spatial_pad_cursor(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) -> core::ops::ControlFlow<()> {
    if crate::game::spatial_integration::pad_cursor(game,batch) { core::ops::ControlFlow::Break(()) } else { core::ops::ControlFlow::Continue(()) }
}
