fn spatial_pause(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    if crate::game::spatial_integration::pauses(game) { core::ops::ControlFlow::Break(true) } else { core::ops::ControlFlow::Continue(()) }
}
