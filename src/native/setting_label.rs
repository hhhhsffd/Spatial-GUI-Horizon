fn spatial_setting_label(setting: &crate::mod_list::NativeSetting) -> core::ops::ControlFlow<Text> {
    if setting.key.starts_with("spatial_gui/button/") { core::ops::ControlFlow::Break(crate::text::Text::literal(setting.label.clone())) } else { core::ops::ControlFlow::Continue(()) }
}
