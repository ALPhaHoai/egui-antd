use egui::{Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Size options for the input component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Large,
    #[default]
    Middle,
    Small,
}

/// Visual variants for the input component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputVariant {
    #[default]
    Outlined,
    Borderless,
    Filled,
    Underlined,
}

pub struct Input<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    size: InputSize,
    variant: InputVariant,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    prefix: Option<Box<dyn FnOnce(&mut Ui)>>, /* FIXME: clippy::type_complexity */
    #[allow(clippy::type_complexity)]
    suffix: Option<Box<dyn FnOnce(&mut Ui)>>, /* FIXME: clippy::type_complexity */
    password: bool,
    allow_clear: bool,
}

impl<'a> Input<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            size: InputSize::Middle,
            variant: InputVariant::Outlined,
            disabled: false,
            prefix: None,
            suffix: None,
            password: false,
            allow_clear: false,
        }
    }

    pub fn hint_text(mut self, text: impl Into<String>) -> Self {
        self.hint_text = Some(text.into());
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

    pub fn prefix(mut self, prefix: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.prefix = Some(Box::new(prefix));
        self
    }

    pub fn suffix(mut self, suffix: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.suffix = Some(Box::new(suffix));
        self
    }

    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    pub fn allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }
}

impl<'a> Widget for Input<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Input {
            text,
            hint_text,
            size,
            variant,
            disabled,
            prefix,
            suffix,
            password,
            allow_clear,
        } = self;

        let padding = match size {
            InputSize::Large => Vec2::new(11.0, 7.0),
            InputSize::Middle => Vec2::new(11.0, 4.0),
            InputSize::Small => Vec2::new(7.0, 0.0),
        };

        let height = match size {
            InputSize::Large => 40.0,
            InputSize::Middle => 32.0,
            InputSize::Small => 24.0,
        };

        let bg_color = if disabled || variant == InputVariant::Filled {
            Color32::from_rgb(245, 245, 245)
        } else if variant == InputVariant::Borderless {
            Color32::TRANSPARENT
        } else {
            Color32::WHITE
        };

        let stroke = if variant == InputVariant::Outlined || variant == InputVariant::Underlined {
            Stroke::new(1.0, Color32::from_rgb(217, 217, 217))
        } else {
            Stroke::NONE
        };

        let rounding = if variant == InputVariant::Underlined || variant == InputVariant::Borderless
        {
            CornerRadius::ZERO
        } else {
            CornerRadius::same(6)
        };

        let text_color = if disabled {
            Color32::from_rgb(0, 0, 0).linear_multiply(0.25)
        } else {
            Color32::from_rgb(0, 0, 0).linear_multiply(0.88)
        };

        let frame = egui::Frame::default()
            .inner_margin(padding)
            .corner_radius(rounding)
            .fill(bg_color)
            .stroke(stroke);

        // We'll calculate hover and focus states inside the ui block

        let response = frame.show(ui, |ui| {
            ui.set_min_height(height - padding.y * 2.0);

            ui.horizontal(|ui| {
                if let Some(prefix_fn) = prefix {
                    prefix_fn(ui);
                }

                let mut text_edit = egui::TextEdit::singleline(text)
                    .text_color(text_color)
                    .min_size(Vec2::new(0.0, height - padding.y * 2.0));

                if let Some(hint) = hint_text {
                    text_edit = text_edit.hint_text(
                        egui::WidgetText::from(hint)
                            .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.25)),
                    );
                }

                if password {
                    text_edit = text_edit.password(true);
                }

                let text_resp = ui.add_enabled(!disabled, text_edit);

                if allow_clear && !text.is_empty() && !disabled {
                    // Add clear icon
                    let clear_icon = "✖"; // Placeholder for actual phosphor icon
                    let clear_btn = ui.add(
                        egui::Label::new(
                            egui::RichText::new(clear_icon)
                                .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.25)),
                        )
                        .sense(Sense::click()),
                    );

                    if clear_btn.clicked() {
                        text.clear();
                        text_resp.request_focus();
                    }
                }

                if let Some(suffix_fn) = suffix {
                    suffix_fn(ui);
                }

                text_resp
            })
            .inner
        });

        // Add focus/hover styling logic
        // This is a simplified approach, need proper interaction tracking

        // Return the response of the whole frame, not just the text edit
        response.response
    }
}
