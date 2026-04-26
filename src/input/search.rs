use egui::{Color32, Response, Sense, Ui, Widget};

use crate::button::{Button, ButtonPosition, ButtonSize, ButtonType};
use crate::input::input::render_input_core;
use crate::input::utils::{self, InputStatus};
use crate::input::{Input, InputSize, InputVariant};

pub struct Search<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    size: InputSize,
    disabled: bool,
    allow_clear: bool,
    enter_button: bool,
    enter_button_text: Option<String>,
    status: InputStatus,
    #[allow(clippy::type_complexity)]
    suffix: Option<Box<dyn FnOnce(&mut Ui)>>,
}

impl<'a> Search<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            size: InputSize::Middle,
            disabled: false,
            allow_clear: false,
            enter_button: false,
            enter_button_text: None,
            status: InputStatus::Normal,
            suffix: None,
        }
    }

    pub fn hint_text(mut self, hint: impl Into<String>) -> Self {
        self.hint_text = Some(hint.into());
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }

    pub fn enter_button(mut self, enter_button: bool) -> Self {
        self.enter_button = enter_button;
        self
    }

    pub fn enter_button_text(mut self, text: impl Into<String>) -> Self {
        self.enter_button_text = Some(text.into());
        self.enter_button = true;
        self
    }

    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }

    pub fn suffix(mut self, suffix: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.suffix = Some(Box::new(suffix));
        self
    }
}

impl<'a> Widget for Search<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Search {
            text,
            hint_text,
            size,
            disabled,
            allow_clear,
            enter_button,
            enter_button_text,
            status,
            suffix,
        } = self;

        let has_enter_button = enter_button || enter_button_text.is_some();
        let search_icon = egui_phosphor::regular::MAGNIFYING_GLASS;

        if has_enter_button {
            let resp = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                let btn_size = match size {
                    InputSize::Large => ButtonSize::Large,
                    InputSize::Middle => ButtonSize::Middle,
                    InputSize::Small => ButtonSize::Small,
                };

                let btn = if let Some(btn_text) = enter_button_text {
                    Button::new(btn_text)
                        .button_type(ButtonType::Primary)
                        .size(btn_size)
                        .set_position(ButtonPosition::Last)
                } else {
                    Button::new("")
                        .button_type(ButtonType::Primary)
                        .size(btn_size)
                        .set_position(ButtonPosition::Last)
                        .icon(egui::RichText::new(search_icon))
                };

                // Place the trailing button first so it occupies the right edge,
                // leaving the input to fill only the remaining width on the left.
                ui.add_enabled(!disabled, btn);

                let mut rounding = utils::get_input_rounding(InputVariant::Outlined);
                rounding.ne = 0;
                rounding.se = 0;

                render_input_core(
                    ui,
                    text,
                    hint_text,
                    size,
                    InputVariant::Outlined,
                    disabled,
                    false,
                    false,
                    allow_clear,
                    None,
                    false,
                    status,
                    None,
                    suffix,
                    rounding,
                )
            });
            resp.inner
        } else {
            let search_disabled = disabled;
            let icon_color = if search_disabled {
                Color32::from_rgb(0, 0, 0).linear_multiply(0.25)
            } else {
                Color32::from_rgb(0, 0, 0).linear_multiply(0.45)
            };

            let search_suffix: Box<dyn FnOnce(&mut Ui)> = if let Some(user_suffix) = suffix {
                Box::new(move |ui: &mut Ui| {
                    user_suffix(ui);
                    ui.add(
                        egui::Label::new(egui::RichText::new(search_icon).color(icon_color))
                            .sense(Sense::click()),
                    );
                })
            } else {
                Box::new(move |ui: &mut Ui| {
                    ui.add(
                        egui::Label::new(egui::RichText::new(search_icon).color(icon_color))
                            .sense(Sense::click()),
                    );
                })
            };

            let mut input = Input::new(text)
                .size(size)
                .variant(InputVariant::Outlined)
                .disabled(disabled)
                .allow_clear(allow_clear)
                .status(status)
                .suffix(search_suffix);

            if let Some(hint) = hint_text {
                input = input.hint_text(hint);
            }

            input.ui(ui)
        }
    }
}
