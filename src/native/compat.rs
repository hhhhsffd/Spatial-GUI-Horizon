// Preserve Spatial GUI's actions, category picker and click-to-step settings
// on Horizon's typed native settings pages.
use super::{Game, spatial_controls, spatial_integration as runtime};
use horizon_client::mod_list::{
    Installed, KeyMapping, NativePage, NativeSetting, NativeSettingValue,
};

impl Game<'_> {
    pub(super) fn native_on(&self, id: &str) -> bool {
        if id == runtime::ID {
            !self.settings.mods.disabled.contains(id)
        } else {
            self.settings
                .mods
                .installed
                .iter()
                .position(|row| row.id == id)
                .is_some_and(|i| self.settings.mods.is_on(i))
        }
    }
    pub(super) fn native_mod_key(&self, owner: &str, name: &str, default: &str) -> String {
        self.settings
            .mods
            .keys
            .get(owner)
            .and_then(|keys| keys.get(name))
            .map_or_else(|| default.into(), Clone::clone)
    }
}

pub(super) fn refresh(game: &mut Game<'_>) {
    let clicked: Vec<_> = game
        .settings
        .mods
        .installed
        .iter()
        .find(|row| row.id == runtime::ID)
        .and_then(|row| row.native.as_ref())
        .map(|page| {
            page.settings
                .iter()
                .enumerate()
                .filter_map(|(i, setting)| {
                    matches!(setting.value, NativeSettingValue::Choice { index: 1, .. })
                        .then_some(i)
                })
                .collect()
        })
        .unwrap_or_default();
    for button in clicked {
        spatial_controls::settings_click(game, button);
    }
    let settings = spatial_controls::settings(game)
        .into_iter()
        .enumerate()
        .map(|(i, label)| NativeSetting {
            key: format!("spatial_gui/button/{i}"),
            label,
            tooltip: None,
            // The two sentinel choices ensure every left activation emits an event.
            // The label hook draws the original caption; our config owns its value.
            value: NativeSettingValue::Choice {
                options: vec![String::new(), String::new()],
                index: 0,
            },
        })
        .collect();
    let keys = [
        ("editor", "Toggle visual editor", "key.keyboard.f8"),
        ("profiles", "Choose a profile", "key.keyboard.grave.accent"),
        ("config", "Open configuration", "key.keyboard.f7"),
        ("reload", "Reload configuration", "key.keyboard.f6"),
        ("mouse_free", "Toggle free cursor", "key.keyboard.tab"),
    ];
    let mappings = keys
        .into_iter()
        .map(|(id, label, key)| KeyMapping {
            id: id.into(),
            label: label.into(),
            key: key.into(),
            modifiers: 0,
            context: Default::default(),
            category: "Spatial GUI".into(),
        })
        .collect();
    let page = NativePage { mappings, settings };

    if let Some(row) = game
        .settings
        .mods
        .installed
        .iter_mut()
        .find(|row| row.id == runtime::ID)
    {
        if row.native.as_ref() != Some(&page) {
            row.native = Some(page);
            game.ui.options.layout_changed();
        }
    } else {
        game.ui.options.layout_changed();
        game.settings.mods.installed.push(Installed {
            id: runtime::ID.into(), title: format!("Spatial GUI {}", runtime::VERSION),
            description: "World-space menus, camera transitions, curved input and visual editor. Rust port of tastytrash/Spatial-GUI.".into(),
            problem: None, dev: false, native: Some(page),
        });
    }
}
