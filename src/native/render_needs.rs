fn spatial_render_needs(needs: RenderNeeds) -> RenderNeeds {
    let mut needs = needs; crate::hud::spatial_gui::needs(&mut needs); needs
}
