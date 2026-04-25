use eframe::egui;

#[derive(Default, Clone)]
pub struct ButtonTheme {
    pub default_bg_disabled: Option<egui::Color32>,
    pub dashed_bg_disabled: Option<egui::Color32>,
}

#[derive(Default, Clone)]
pub struct ComponentsTheme {
    pub button: ButtonTheme,
}

#[derive(Default, Clone)]
pub struct Theme {
    pub components: ComponentsTheme,
}

pub struct ConfigProvider {
    theme: Theme,
}

impl Default for ConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigProvider {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn show<R>(self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
        let id = egui::Id::new("antd_config_provider_theme");

        let previous_theme = ui
            .ctx()
            .data(|d| d.get_temp::<Theme>(id).unwrap_or_default());

        ui.ctx().data_mut(|d| d.insert_temp(id, self.theme.clone()));

        let result = add_contents(ui);

        ui.ctx().data_mut(|d| d.insert_temp(id, previous_theme));

        result
    }
}
