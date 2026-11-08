fn spatial_render(renderer: &crate::hud::HudRenderer, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView, depth: &wgpu::TextureView, batch: &crate::hud::HudBatch, size: [u32;2], vertices: &std::ops::Range<u32>) -> core::ops::ControlFlow<()> {
    if crate::hud::spatial_gui::draw(renderer,encoder,target,depth,batch,size,vertices.clone()) { core::ops::ControlFlow::Break(()) } else { core::ops::ControlFlow::Continue(()) }
}
