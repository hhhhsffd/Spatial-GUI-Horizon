fn spatial_relative(app: &crate::game::App, event: &winit::event::DeviceEvent) {
    if let crate::game::App::Ready(game) = app { crate::game::spatial_controls::relative(&game.host,event); }
}
