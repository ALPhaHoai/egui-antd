use egui::{Align, Color32, Layout, Response, Sense, TextEdit, Ui, Vec2, Widget};

use super::input::{InputSize, InputVariant};
use super::utils::{
    InputStatus, get_input_colors, get_input_metrics, get_input_rounding, get_interactive_stroke,
};

pub struct TextArea<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    size: InputSize,
    variant: InputVariant,
    disabled: bool,
    read_only: bool,
    allow_clear: bool,
    max_length: Option<usize>,
    show_count: bool,
    auto_size: Option<(usize, Option<usize>)>,
    status: InputStatus,
}

impl<'a> TextArea<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            size: InputSize::Middle,
            variant: InputVariant::Outlined,
            disabled: false,
            read_only: false,
            allow_clear: false,
            max_length: None,
            show_count: false,
            auto_size: None,
            status: InputStatus::default(),
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

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn show_count(mut self, show_count: bool) -> Self {
        self.show_count = show_count;
        self
    }

    pub fn auto_size(mut self, min_rows: usize, max_rows: Option<usize>) -> Self {
        self.auto_size = Some((min_rows, max_rows));
        self
    }

    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }
}

impl<'a> Widget for TextArea<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let TextArea {
            text,
            hint_text,
            size,
            variant,
            disabled,
            read_only,
            allow_clear,
            max_length,
            show_count,
            auto_size,
            status,
        } = self;

        if let Some(max_len) = max_length
            && text.chars().count() > max_len
        {
            let truncated: String = text.chars().take(max_len).collect();
            *text = truncated;
        }

        let (padding, _min_height) = get_input_metrics(size);
        let (bg_color, base_stroke, text_color) = get_input_colors(variant, disabled);
        let rounding = get_input_rounding(variant);

        let is_hovered = ui.rect_contains_pointer(egui::Rect::from_min_max(
            ui.next_widget_position(),
            ui.next_widget_position() + Vec2::new(ui.available_width(), 80.0),
        ));
        let interact_id = ui.next_auto_id();
        let is_focused = ui.memory(|mem| mem.has_focus(interact_id));

        let stroke =
            get_interactive_stroke(base_stroke, variant, disabled, is_focused, is_hovered, status);

        let frame = egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(padding.x as i8, padding.y as i8))
            .corner_radius(rounding)
            .fill(bg_color)
            .stroke(stroke);

        let frame_resp = frame
            .show(ui, |ui| {
                let mut text_edit = TextEdit::multiline(text)
                    .text_color(text_color)
                    .frame(egui::Frame::NONE)
                    .desired_width(ui.available_width());

                if disabled || read_only {
                    text_edit = text_edit.interactive(false);
                }

                if let Some(hint) = hint_text {
                    text_edit = text_edit.hint_text(hint);
                }

                if let Some((min_rows, _max_rows)) = auto_size {
                    text_edit = text_edit.desired_rows(min_rows);
                }

                let text_output = text_edit.show(ui);
                let response = text_output.response;

                let has_footer =
                    (allow_clear && !text.is_empty() && !disabled && !read_only) || show_count;
                if has_footer {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if show_count {
                            let current_count = text.chars().count();
                            let count_text = if let Some(max_len) = max_length {
                                format!("{}/{}", current_count, max_len)
                            } else {
                                format!("{}", current_count)
                            };
                            ui.label(
                                egui::RichText::new(count_text)
                                    .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                            );
                        }

                        if allow_clear && !text.is_empty() && !disabled && !read_only {
                            let clear_btn = ui.add(
                                egui::Label::new(
                                    egui::RichText::new("✖").color(
                                        Color32::from_rgb(0, 0, 0).linear_multiply(0.25),
                                    ),
                                )
                                .sense(Sense::click()),
                            );
                            if clear_btn.clicked() {
                                text.clear();
                            }
                        }
                    });
                }

                response
            });
        
        // Return the response for the frame, so interaction works properly 
        frame_resp.response
    }
}
