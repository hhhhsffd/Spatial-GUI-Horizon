fn spatial_orientation() -> core::ops::ControlFlow<glam::Quat> {
    if let Some(rotation) = crate::game::spatial_integration::orientation() { core::ops::ControlFlow::Break(rotation) } else { core::ops::ControlFlow::Continue(()) }
}
