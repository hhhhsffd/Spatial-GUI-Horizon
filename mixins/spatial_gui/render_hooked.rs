fn spatial_render_hooked() -> core::ops::ControlFlow<bool> {
    if crate::hud::spatial_gui::active() { core::ops::ControlFlow::Break(true) } else { core::ops::ControlFlow::Continue(()) }
}
