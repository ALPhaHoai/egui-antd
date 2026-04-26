use egui::{Color32, Response, Sense, Ui, Vec2, Widget};

use super::utils::{self, InputStatus};
use crate::input::{InputSize, InputVariant};

pub struct InputNumber<'a, T> {
    value: &'a mut T,
    min: Option<T>,
    max: Option<T>,
    step: T,
    size: InputSize,
    variant: InputVariant,
    disabled: bool,
    controls: bool,
    status: InputStatus,
}

impl<'a, T> InputNumber<'a, T>
where
    T: egui::emath::Numeric + std::fmt::Display + std::str::FromStr,
{
    pub fn new(value: &'a mut T) -> Self {
        let step = T::from_f64(1.0);
        Self {
            value,
            min: None,
            max: None,
            step,
            size: InputSize::Middle,
            variant: InputVariant::Outlined,
            disabled: false,
            controls: true,
            status: InputStatus::Normal,
        }
    }

    pub fn min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: T) -> Self {
        self.step = step;
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

    pub fn controls(mut self, controls: bool) -> Self {
        self.controls = controls;
        self
    }

    pub fn status(mut self, status: InputStatus) -> Self {
        self.status = status;
        self
    }

    fn clamp_val(val: f64, min: &Option<T>, max: &Option<T>) -> f64 {
        let mut v = val;
        if let Some(lo) = min {
            v = v.max(lo.to_f64());
        }
        if let Some(hi) = max {
            v = v.min(hi.to_f64());
        }
        v
    }
}

impl<'a, T> Widget for InputNumber<'a, T>
where
    T: egui::emath::Numeric + std::fmt::Display + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.next_auto_id();

        let step_f64 = self.step.to_f64();
        let min = self.min;
        let max = self.max;
        let size = self.size;
        let variant = self.variant;
        let disabled = self.disabled;
        let controls = self.controls;
        let status = self.status;
        let value = self.value;

        let (padding, height) = utils::get_input_metrics(size);
        let (bg_color, base_stroke, text_color) = utils::get_input_colors(variant, disabled);
        let rounding = utils::get_input_rounding(variant);

        let current_text = value.to_string();
        let mut text_buf: String = ui
            .data_mut(|d| d.get_temp::<String>(id))
            .unwrap_or_else(|| current_text.clone());

        if let Ok(buf_val) = text_buf.parse::<T>()
            && buf_val.to_f64() != value.to_f64()
        {
            text_buf = current_text;
        }

        let frame = egui::Frame::default()
            .inner_margin(padding)
            .corner_radius(rounding)
            .fill(bg_color)
            .stroke(base_stroke);

        let frame_resp = frame.show(ui, |ui| {
            ui.set_min_height(height - padding.y * 2.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                let text_edit = egui::TextEdit::singleline(&mut text_buf)
                    .text_color(text_color)
                    .frame(egui::Frame::NONE)
                    .desired_width(f32::INFINITY)
                    .min_size(Vec2::new(0.0, height - padding.y * 2.0));

                let text_resp = ui.add_enabled(!disabled, text_edit);

                if text_resp.lost_focus() {
                    if let Ok(parsed) = text_buf.parse::<T>() {
                        let clamped = Self::clamp_val(parsed.to_f64(), &min, &max);
                        *value = T::from_f64(clamped);
                    }
                    text_buf = value.to_string();
                } else if text_resp.changed()
                    && let Ok(parsed) = text_buf.parse::<T>()
                {
                    let clamped = Self::clamp_val(parsed.to_f64(), &min, &max);
                    *value = T::from_f64(clamped);
                }

                if controls && !disabled {
                    let btn_height = (height - padding.y * 2.0) / 2.0;

                    // Vertical separator
                    let (rect, _) =
                        ui.allocate_at_least(Vec2::new(1.0, height - padding.y * 2.0), Sense::hover());
                    ui.painter()
                        .line_segment([rect.left_top(), rect.left_bottom()], base_stroke);

                    ui.vertical(|ui| {
                        ui.set_min_height(height - padding.y * 2.0);
                        ui.spacing_mut().item_spacing.y = 0.0;

                        let up_btn = ui.add_sized(
                            Vec2::new(20.0, btn_height),
                            egui::Label::new(
                                egui::RichText::new("\u{25b2}")
                                    .size(8.0)
                                    .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                            )
                            .sense(Sense::click()),
                        );
                        if up_btn.clicked() {
                            let new_val = Self::clamp_val(value.to_f64() + step_f64, &min, &max);
                            *value = T::from_f64(new_val);
                            text_buf = value.to_string();
                        }

                        // Horizontal separator between buttons
                        let (rect, _) = ui.allocate_at_least(Vec2::new(20.0, 1.0), Sense::hover());
                        ui.painter()
                            .line_segment([rect.left_top(), rect.right_top()], base_stroke);

                        let down_btn = ui.add_sized(
                            Vec2::new(20.0, btn_height),
                            egui::Label::new(
                                egui::RichText::new("\u{25bc}")
                                    .size(8.0)
                                    .color(Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                            )
                            .sense(Sense::click()),
                        );
                        if down_btn.clicked() {
                            let new_val = Self::clamp_val(value.to_f64() - step_f64, &min, &max);
                            *value = T::from_f64(new_val);
                            text_buf = value.to_string();
                        }
                    });
                }

                if text_resp.has_focus() {
                    let up_pressed = ui.input(|r| r.key_pressed(egui::Key::ArrowUp));
                    let down_pressed = ui.input(|r| r.key_pressed(egui::Key::ArrowDown));

                    if up_pressed {
                        let new_val =
                            Self::clamp_val(value.to_f64() + step_f64, &min, &max);
                        *value = T::from_f64(new_val);
                        text_buf = value.to_string();
                    } else if down_pressed {
                        let new_val =
                            Self::clamp_val(value.to_f64() - step_f64, &min, &max);
                        *value = T::from_f64(new_val);
                        text_buf = value.to_string();
                    }
                }

                text_resp
            })
            .inner
        });

        ui.data_mut(|d| d.insert_temp(id, text_buf));

        let resp = &frame_resp.response;
        let focused = resp.has_focus();
        let hovered = ui.rect_contains_pointer(resp.rect);
        let interactive_stroke =
            utils::get_interactive_stroke(base_stroke, variant, disabled, focused, hovered, status);
        if interactive_stroke != base_stroke {
            ui.painter()
                .rect_stroke(resp.rect, rounding, interactive_stroke, egui::StrokeKind::Inside);
        }

        frame_resp.response
    }
}
