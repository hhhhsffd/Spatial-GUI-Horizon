fn spatial_blur() -> core::ops::ControlFlow<Option<f32>> {
    if crate::game::spatial_integration::suppress_blur() { core::ops::ControlFlow::Break(None) } else { core::ops::ControlFlow::Continue(()) }
}
