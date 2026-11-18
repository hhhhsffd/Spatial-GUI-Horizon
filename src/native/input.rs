fn spatial_input(game: &mut crate::game::Game<'_>, event: &mut winit::event::WindowEvent) -> core::ops::ControlFlow<()> {
    if crate::game::spatial_controls::input(game, event) { core::ops::ControlFlow::Break(()) } else { core::ops::ControlFlow::Continue(()) }
}
