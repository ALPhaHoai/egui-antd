use egui::{Color32, Response, Sense, Ui, Widget};

use crate::input::{Input, InputSize, InputVariant};

pub struct Password<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    size: InputSize,
    variant: InputVariant,
    disabled: bool,
    visibility_toggle: bool,
    allow_clear: bool,
}

impl<'a> Password<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            size: InputSize::Middle,
            variant: InputVariant::Outlined,
            disabled: false,
            visibility_toggle: true,
            allow_clear: false,
        }
    }

    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.hint_text = Some(hint_text.into());
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn visibility_toggle(mut self, visibility_toggle: bool) -> Self {
        self.visibility_toggle = visibility_toggle;
        self
    }

    pub fn allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }
}

impl<'a> Widget for Password<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.next_auto_id();
        let mut visible = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

        let mut input = Input::new(self.text)
            .size(self.size)
            .variant(self.variant)
            .disabled(self.disabled)
            .password(!visible)
            .allow_clear(self.allow_clear);

        if let Some(hint_text) = self.hint_text {
            input = input.hint_text(hint_text);
        }

        if self.visibility_toggle {
            input = input.suffix(Box::new(move |ui: &mut Ui| {
                let icon = if visible { "⊙" } else { "⊘" };
                let response = ui
                    .add(
                        egui::Label::new(
                            egui::RichText::new(icon)
                                .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                        )
                        .sense(Sense::click()),
                    );
                if response.clicked() {
                    visible = !visible;
                    ui.data_mut(|d| d.insert_temp(id, visible));
                }
            }));
        }

        input.ui(ui)
    }
}
