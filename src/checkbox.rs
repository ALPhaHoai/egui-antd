use egui::{self, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, Vec2, Widget, WidgetInfo, WidgetType};

// AntD 5.0 Design Tokens
const CONTROL_INTERACTIVE_SIZE: f32 = 16.0;
const BORDER_RADIUS_SM: u8 = 2;
const COLOR_PRIMARY: Color32 = Color32::from_rgb(22, 119, 255);
const COLOR_PRIMARY_HOVER: Color32 = Color32::from_rgb(64, 150, 255);
const COLOR_BORDER: Color32 = Color32::from_rgb(217, 217, 217);
const COLOR_BG_CONTAINER: Color32 = Color32::WHITE;
const COLOR_BG_CONTAINER_DISABLED: Color32 = Color32::from_rgb(245, 245, 245);
const COLOR_TEXT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 224);
const COLOR_TEXT_DISABLED: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 64);
const LABEL_GAP: f32 = 8.0;
const CHECKMARK_COLOR: Color32 = Color32::WHITE;

pub struct CheckboxOption {
    pub label: String,
    pub value: String,
    pub disabled: Option<bool>,
}

impl CheckboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }
}

pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: Option<String>,
    disabled: bool,
    indeterminate: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            disabled: false,
            indeterminate: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
}

impl Widget for Checkbox<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Checkbox {
            checked,
            label,
            disabled,
            indeterminate,
        } = self;

        let text_color = if disabled {
            COLOR_TEXT_DISABLED
        } else {
            COLOR_TEXT
        };

        // Compute text size only (drop galley immediately to avoid holding Arc across ctx calls)
        let font_id = FontId::proportional(14.0);
        let text_size = label.as_ref().map(|text| {
            let g = ui.painter().layout_no_wrap(text.clone(), font_id.clone(), text_color);
            g.size()
        });

        let total_width = CONTROL_INTERACTIVE_SIZE
            + text_size.map_or(0.0, |s| LABEL_GAP + s.x);
        let total_height = CONTROL_INTERACTIVE_SIZE
            .max(text_size.map_or(0.0, |s| s.y));

        let desired_size = Vec2::new(total_width, total_height);
        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        if disabled {
            response = response.on_hover_cursor(egui::CursorIcon::NotAllowed);
        } else {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        if response.clicked() && !disabled {
            *checked = !*checked;
            response.mark_changed();
        }

        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::Checkbox,
                !disabled,
                *checked,
                label.as_deref().unwrap_or(""),
            )
        });

        if ui.is_rect_visible(rect) {
            let is_hover = response.hovered() && !disabled;

            let hover_t =
                ui.ctx()
                    .animate_bool_with_time(response.id.with("hover"), is_hover, 0.1);

            // Wave effect timing
            let wave_id = response.id.with("wave");
            let now = ui.input(|i| i.time);
            if response.clicked() && !disabled {
                ui.ctx().data_mut(|d| d.insert_temp(wave_id, now));
            }
            let last_click_time: Option<f64> = ui.ctx().data(|d| d.get_temp(wave_id));
            let wave_t = if let Some(t) = last_click_time {
                let elapsed = now - t;
                let duration = 0.4;
                if elapsed < duration {
                    ui.ctx().request_repaint();
                    (elapsed / duration) as f32
                } else {
                    ui.ctx().data_mut(|d| d.remove_temp::<f64>(wave_id));
                    0.0
                }
            } else {
                0.0
            };

            // Box rect: 16x16, vertically centered, left-aligned
            let box_rect = Rect::from_min_size(
                Pos2::new(
                    rect.min.x,
                    rect.min.y + (rect.height() - CONTROL_INTERACTIVE_SIZE) / 2.0,
                ),
                Vec2::splat(CONTROL_INTERACTIVE_SIZE),
            );

            let rounding = CornerRadius::same(BORDER_RADIUS_SM);
            let is_checked = *checked;

            // Box painting: 6 visual states
            if indeterminate || is_checked {
                let fill = if disabled {
                    COLOR_BG_CONTAINER_DISABLED
                } else {
                    lerp_color(COLOR_PRIMARY, COLOR_PRIMARY_HOVER, hover_t)
                };
                ui.painter().rect_filled(box_rect, rounding, fill);
                if disabled {
                    ui.painter().rect_stroke(
                        box_rect,
                        rounding,
                        Stroke::new(1.0, COLOR_BORDER),
                        egui::StrokeKind::Inside,
                    );
                }
            } else {
                let fill = if disabled {
                    COLOR_BG_CONTAINER_DISABLED
                } else {
                    COLOR_BG_CONTAINER
                };
                let border_color = if disabled {
                    COLOR_BORDER
                } else {
                    lerp_color(COLOR_BORDER, COLOR_PRIMARY, hover_t)
                };
                ui.painter().rect_filled(box_rect, rounding, fill);
                ui.painter().rect_stroke(
                    box_rect,
                    rounding,
                    Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );
            }

            // Checkmark (checked, not indeterminate)
            if is_checked && !indeterminate {
                let mark_color = if disabled {
                    COLOR_TEXT_DISABLED
                } else {
                    CHECKMARK_COLOR
                };
                let p1 = box_rect.min + Vec2::new(3.5, 8.0);
                let p2 = box_rect.min + Vec2::new(6.5, 11.0);
                let p3 = box_rect.min + Vec2::new(13.0, 4.5);
                ui.painter()
                    .line_segment([p1, p2], Stroke::new(2.0, mark_color));
                ui.painter()
                    .line_segment([p2, p3], Stroke::new(2.0, mark_color));
            }

            // Indeterminate dash
            if indeterminate {
                let dash_color = if disabled {
                    COLOR_TEXT_DISABLED
                } else {
                    CHECKMARK_COLOR
                };
                let dash_rect =
                    Rect::from_center_size(box_rect.center(), Vec2::new(8.0, 2.0));
                ui.painter()
                    .rect_filled(dash_rect, CornerRadius::ZERO, dash_color);
            }

            // Wave effect halo
            if wave_t > 0.0 && wave_t < 1.0 {
                let alpha = (1.0 - wave_t) * 0.4;
                let wave_stroke = Stroke::new(2.0, COLOR_PRIMARY.gamma_multiply(alpha));
                let expansion = wave_t * 6.0;
                let wave_rect = box_rect.expand(expansion);
                let mut wave_radius = rounding;
                wave_radius.nw = (wave_radius.nw as f32 + expansion).min(255.0) as u8;
                wave_radius.ne = (wave_radius.ne as f32 + expansion).min(255.0) as u8;
                wave_radius.sw = (wave_radius.sw as f32 + expansion).min(255.0) as u8;
                wave_radius.se = (wave_radius.se as f32 + expansion).min(255.0) as u8;
                ui.painter().rect_stroke(
                    wave_rect,
                    wave_radius,
                    wave_stroke,
                    egui::StrokeKind::Outside,
                );
            }

            // Label: create galley fresh here during paint phase only
            if let Some(text) = &label {
                let galley = ui.painter().layout_no_wrap(
                    text.clone(),
                    font_id,
                    text_color,
                );
                let text_pos = Pos2::new(
                    box_rect.max.x + LABEL_GAP,
                    rect.min.y + (rect.height() - galley.size().y) / 2.0,
                );
                ui.painter().galley(text_pos, galley, text_color);
            }
        }

        response
    }
}

pub struct CheckboxGroup<'a> {
    values: &'a mut Vec<String>,
    options: Vec<CheckboxOption>,
    disabled: bool,
}

impl<'a> CheckboxGroup<'a> {
    pub fn new(values: &'a mut Vec<String>) -> Self {
        Self {
            values,
            options: Vec::new(),
            disabled: false,
        }
    }

    pub fn options(mut self, options: Vec<CheckboxOption>) -> Self {
        self.options = options;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) {
        let CheckboxGroup {
            values,
            options,
            disabled,
        } = self;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for option in &options {
                let mut is_checked = values.contains(&option.value);
                let was_checked = is_checked;
                let option_disabled = disabled || option.disabled.unwrap_or(false);
                ui.add(
                    Checkbox::new(&mut is_checked)
                        .label(option.label.as_str())
                        .disabled(option_disabled),
                );
                if is_checked != was_checked {
                    if is_checked {
                        values.push(option.value.clone());
                    } else {
                        values.retain(|v| v != &option.value);
                    }
                }
            }
        });
    }
}

fn lerp_color(c1: Color32, c2: Color32, t: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        egui::lerp((c1.r() as f32)..=(c2.r() as f32), t) as u8,
        egui::lerp((c1.g() as f32)..=(c2.g() as f32), t) as u8,
        egui::lerp((c1.b() as f32)..=(c2.b() as f32), t) as u8,
        egui::lerp((c1.a() as f32)..=(c2.a() as f32), t) as u8,
    )
}
