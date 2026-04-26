use egui::{Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2, Widget};

use super::utils::{self, InputStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Large,
    #[default]
    Middle,
    Small,
}

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
    prefix: Option<Box<dyn FnOnce(&mut Ui)>>,
    #[allow(clippy::type_complexity)]
    suffix: Option<Box<dyn FnOnce(&mut Ui)>>,
    #[allow(clippy::type_complexity)]
    addon_before: Option<Box<dyn FnOnce(&mut Ui)>>,
    #[allow(clippy::type_complexity)]
    addon_after: Option<Box<dyn FnOnce(&mut Ui)>>,
    password: bool,
    allow_clear: bool,
    max_length: Option<usize>,
    show_count: bool,
    read_only: bool,
    status: InputStatus,
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
            addon_before: None,
            addon_after: None,
            password: false,
            allow_clear: false,
            max_length: None,
            show_count: false,
            read_only: false,
            status: InputStatus::Normal,
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

    pub fn addon_before(mut self, addon: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.addon_before = Some(Box::new(addon));
        self
    }

    pub fn addon_after(mut self, addon: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.addon_after = Some(Box::new(addon));
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

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn show_count(mut self, show_count: bool) -> Self {
        self.show_count = show_count;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn render_input_core(
    ui: &mut Ui,
    text: &mut String,
    hint_text: Option<String>,
    size: InputSize,
    variant: InputVariant,
    disabled: bool,
    read_only: bool,
    password: bool,
    allow_clear: bool,
    max_length: Option<usize>,
    show_count: bool,
    status: InputStatus,
    prefix: Option<Box<dyn FnOnce(&mut Ui)>>,
    suffix: Option<Box<dyn FnOnce(&mut Ui)>>,
    rounding: CornerRadius,
) -> Response {
    let (padding, height) = utils::get_input_metrics(size);
    let (bg_color, base_stroke, text_color) = utils::get_input_colors(variant, disabled);

    let frame = egui::Frame::default()
        .inner_margin(padding)
        .corner_radius(rounding)
        .fill(bg_color)
        .stroke(base_stroke);

    let frame_resp = frame.show(ui, |ui| {
        // The frame's outer rect = content + 2*inner_margin + 2*stroke.width.
        // Subtracting both keeps the input's rendered height equal to the
        // design metric (32/40/24 for Middle/Large/Small) so it matches the
        // adjacent grouped Button height exactly.
        ui.set_min_height(height - padding.y * 2.0 - base_stroke.width * 2.0);

        ui.horizontal(|ui| {
            if let Some(prefix_fn) = prefix {
                prefix_fn(ui);
            }

            // Render trailing widgets (suffix, count, clear) in right_to_left so the
            // text edit naturally fills only the remaining space and never overflows
            // the input frame's width.
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if let Some(suffix_fn) = suffix {
                        suffix_fn(ui);
                    }

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

                    let show_clear_icon = allow_clear && !text.is_empty() && !disabled;
                    let mut clear_clicked = false;
                    if show_clear_icon {
                        let clear_btn = ui.add(
                            egui::Label::new(
                                egui::RichText::new("\u{2716}")
                                    .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.25)),
                            )
                            .sense(Sense::click()),
                        );
                        if clear_btn.clicked() {
                            clear_clicked = true;
                        }
                    }

                    let mut text_edit = egui::TextEdit::singleline(text)
                        .text_color(text_color)
                        .frame(egui::Frame::NONE)
                        .desired_width(f32::INFINITY)
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

                    let text_resp = ui.add_enabled(!disabled && !read_only, text_edit);

                    if clear_clicked {
                        text.clear();
                        text_resp.request_focus();
                    }

                    text_resp
                },
            )
            .inner
        })
        .inner
    });

    let resp = &frame_resp.response;
    let focused = resp.has_focus();
    let hovered = ui.rect_contains_pointer(resp.rect);

    if disabled && hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::NotAllowed);
    }

    if variant == InputVariant::Underlined {
        let underline_stroke = utils::get_interactive_stroke(
            Stroke::new(1.0, Color32::from_rgb(217, 217, 217)),
            variant,
            disabled,
            focused,
            hovered,
            status,
        );
        ui.painter().line_segment(
            [resp.rect.left_bottom(), resp.rect.right_bottom()],
            underline_stroke,
        );
    } else {
        let interactive_stroke =
            utils::get_interactive_stroke(base_stroke, variant, disabled, focused, hovered, status);
        if interactive_stroke != base_stroke {
            ui.painter()
                .rect_stroke(resp.rect, rounding, interactive_stroke, egui::StrokeKind::Inside);
        }
    }

    frame_resp.response
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
            addon_before,
            addon_after,
            password,
            allow_clear,
            max_length,
            show_count,
            read_only,
            status,
        } = self;

        if let Some(max_len) = max_length
            && text.chars().count() > max_len
        {
            let truncated: String = text.chars().take(max_len).collect();
            *text = truncated;
        }

        let mut rounding = utils::get_input_rounding(variant);
        let has_before = addon_before.is_some();
        let has_after = addon_after.is_some();

        if has_before {
            rounding.nw = 0;
            rounding.sw = 0;
        }
        if has_after {
            rounding.ne = 0;
            rounding.se = 0;
        }

        if has_before || has_after {
            let (addon_padding, addon_height) = utils::get_input_metrics(size);
            let inner_height = addon_height - addon_padding.y * 2.0;
            let resp = ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                if let Some(addon_fn) = addon_before {
                    egui::Frame::default()
                        .inner_margin(addon_padding)
                        .fill(Color32::from_rgb(250, 250, 250))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(217, 217, 217)))
                        .corner_radius(CornerRadius {
                            nw: 6,
                            sw: 6,
                            ne: 0,
                            se: 0,
                        })
                        .show(ui, |ui| {
                            ui.set_min_height(inner_height);
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| addon_fn(ui),
                            );
                        });
                }

                let input_resp = render_input_core(
                    ui, text, hint_text, size, variant, disabled, read_only, password, allow_clear,
                    max_length, show_count, status, prefix, suffix, rounding,
                );

                if let Some(addon_fn) = addon_after {
                    egui::Frame::default()
                        .inner_margin(addon_padding)
                        .fill(Color32::from_rgb(250, 250, 250))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(217, 217, 217)))
                        .corner_radius(CornerRadius {
                            nw: 0,
                            sw: 0,
                            ne: 6,
                            se: 6,
                        })
                        .show(ui, |ui| {
                            ui.set_min_height(inner_height);
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| addon_fn(ui),
                            );
                        });
                }

                input_resp
            });
            resp.inner
        } else {
            render_input_core(
                ui, text, hint_text, size, variant, disabled, read_only, password, allow_clear,
                max_length, show_count, status, prefix, suffix, rounding,
            )
        }
    }
}
