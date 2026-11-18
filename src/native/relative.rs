fn spatial_relative(game: &crate::game::Game<'_>, event: &winit::event::DeviceEvent) {
    crate::game::spatial_controls::relative(game,event);
}
