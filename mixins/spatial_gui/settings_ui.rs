// Hit-test the native page's real layout, so scrolling, GUI scale, clipping,
// and key-binding rows use exactly the same coordinates as normal clicks.
use super::{Act, OptionsUi, Page, Settings, WKind, ui};

impl OptionsUi {
    pub fn spatial_gui_setting_at(
        &mut self,
        settings: &Settings,
        size: (f32, f32),
        cursor: (f32, f32),
    ) -> Option<usize> {
        let Page::NativeMod(index) = self.page() else {
            return None;
        };
        if settings.mods.installed.get(index)?.id != "spatial_gui"
            || self.selected_key.is_some()
            || self.selected_native_key.is_some()
            || self.scrollbar_drag_y.is_some()
            || self.resource_error.is_some()
        {
            return None;
        }
        let (widgets, top, bottom, _) = self.layout(settings, size.0, size.1);
        widgets.into_iter().find_map(|widget| match widget.kind {
            WKind::Button {
                active: true,
                act: Act::NativeButton(owner, button),
                ..
            } if owner == index
                && ui::hovered(cursor, widget.x, widget.y, widget.w, widget.h)
                && !(widget.list && (cursor.1 < top || cursor.1 >= bottom)) =>
            {
                Some(button)
            }
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (OptionsUi, Settings) {
        let mut settings = Settings::default();
        settings.mods.installed.push(crate::mod_list::Installed {
            id: "spatial_gui".into(),
            title: "Spatial GUI".into(),
            description: String::new(),
            problem: None,
            dev: false,
            native: Some(crate::mod_list::NativePage {
                keys: vec![["editor".into(), "Editor".into(), "key.keyboard.f8".into()]],
                buttons: (0..40).map(|i| format!("Setting {i}")).collect(),
            }),
        });
        let mut ui = OptionsUi::default();
        ui.show(Page::NativeMod(0));
        (ui, settings)
    }

    #[test]
    fn setting_hit_uses_scrolled_layout_and_clips_hidden_rows() {
        for size in [(320.0, 180.0), (480.0, 240.0), (640.0, 360.0)] {
            let (mut ui, settings) = fixture();
            for scroll in [0.0, 100.0, 300.0] {
                ui.scroll = scroll;
                let (widgets, top, bottom, _) = ui.layout(&settings, size.0, size.1);
                for widget in widgets {
                    if let WKind::Button {
                        act: Act::NativeButton(_, button),
                        ..
                    } = widget.kind
                    {
                        let cursor = (widget.x + widget.w / 2.0, widget.y + widget.h / 2.0);
                        let expected = if cursor.1 >= top && cursor.1 < bottom {
                            Some(button)
                        } else {
                            None
                        };
                        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), expected);
                    }
                }
            }
        }
    }

    #[test]
    fn setting_hit_preserves_bindings_other_pages_and_scrollbar_drags() {
        let (mut ui, mut settings) = fixture();
        let size = (480.0, 240.0);
        let (widgets, ..) = ui.layout(&settings, size.0, size.1);
        let button = widgets
            .iter()
            .find(|w| {
                matches!(
                    w.kind,
                    WKind::Button {
                        act: Act::NativeButton(0, 0),
                        ..
                    }
                )
            })
            .unwrap();
        let cursor = (button.x + 1.0, button.y + 1.0);
        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), Some(0));
        ui.selected_key = Some("key.jump");
        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), None);
        ui.selected_key = None;
        ui.selected_native_key = Some((0, 0));
        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), None);
        ui.selected_native_key = None;
        ui.scrollbar_drag_y = Some(cursor.1);
        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), None);
        ui.scrollbar_drag_y = None;
        settings.mods.installed[0].id = "other_mod".into();
        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), None);
        settings.mods.installed[0].id = "spatial_gui".into();
        ui.show(Page::Mods);
        assert_eq!(ui.spatial_gui_setting_at(&settings, size, cursor), None);
    }
}
