use egui::{NumExt, Response, Ui, Widget, WidgetInfo, WidgetType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonType {
    #[default]
    Default,
    Primary,
    Dashed,
    Link,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Large,
    #[default]
    Middle,
    Small,
}

pub struct Button {
    text: String,
    button_type: ButtonType,
    size: ButtonSize,
    danger: bool,
    disabled: bool,
    loading: bool,
    block: bool,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            button_type: ButtonType::Default,
            size: ButtonSize::Middle,
            danger: false,
            disabled: false,
            loading: false,
            block: false,
        }
    }

    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let Button {
            text,
            button_type,
            size,
            danger,
            disabled,
            loading,
            block,
        } = self;

        // Ant Design Colors (approximate)
        let primary_color = egui::Color32::from_rgb(22, 119, 255);
        let danger_color = egui::Color32::from_rgb(255, 77, 79);
        let border_color = egui::Color32::from_rgb(217, 217, 217);
        let text_color = ui.visuals().widgets.inactive.text_color();

        let button_padding = match size {
            ButtonSize::Large => egui::vec2(15.0, 7.0),
            ButtonSize::Middle => egui::vec2(15.0, 4.0),
            ButtonSize::Small => egui::vec2(7.0, 0.0),
        };

        let text_size = match size {
            ButtonSize::Large => 16.0,
            ButtonSize::Middle => 14.0,
            ButtonSize::Small => 12.0,
        };

        let font_id = egui::FontId::proportional(text_size);
        let wrap_width = ui.available_width();
        let text_job = egui::WidgetText::from(&text).into_galley(ui, None, wrap_width, font_id);

        let mut desired_size = text_job.size() + 2.0 * button_padding;
        if block {
            desired_size.x = ui.available_width();
        }

        let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        fn lighten(color: egui::Color32, amount: f32) -> egui::Color32 {
            let mut hsva = egui::ecolor::Hsva::from(color);
            hsva.v = (hsva.v + amount).at_most(1.0);
            hsva.into()
        }

        if ui.is_rect_visible(rect) {
            let _visuals = ui.style().interact(&response);

            let (bg_fill, stroke_color, text_color) = match (button_type, danger, disabled) {
                (_, _, true) => (
                    egui::Color32::from_rgb(245, 245, 245),
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(217, 217, 217)),
                    egui::Color32::from_rgba_premultiplied(0, 0, 0, 64),
                ),
                (ButtonType::Primary, false, false) => {
                    let fill = if response.hovered() { lighten(primary_color, 0.1) } else { primary_color };
                    (fill, egui::Stroke::NONE, egui::Color32::WHITE)
                }
                (ButtonType::Primary, true, false) => {
                    let fill = if response.hovered() { lighten(danger_color, 0.1) } else { danger_color };
                    (fill, egui::Stroke::NONE, egui::Color32::WHITE)
                }
                (ButtonType::Default, false, false) => {
                    let stroke = if response.hovered() {
                        egui::Stroke::new(1.0, primary_color)
                    } else {
                        egui::Stroke::new(1.0, border_color)
                    };
                    let text = if response.hovered() { primary_color } else { text_color };
                    (egui::Color32::WHITE, stroke, text)
                }
                (ButtonType::Default, true, false) => {
                    let stroke = if response.hovered() {
                        egui::Stroke::new(1.0, lighten(danger_color, 0.1))
                    } else {
                        egui::Stroke::new(1.0, danger_color)
                    };
                    let text = if response.hovered() { lighten(danger_color, 0.1) } else { danger_color };
                    (egui::Color32::WHITE, stroke, text)
                }
                (ButtonType::Dashed, false, false) => {
                    // Dashed is hard in egui without custom painting, simplified here
                    let stroke = if response.hovered() {
                        egui::Stroke::new(1.0, primary_color)
                    } else {
                        egui::Stroke::new(1.0, border_color)
                    };
                    (egui::Color32::TRANSPARENT, stroke, text_color)
                }
                (ButtonType::Link, false, false) => {
                    let text = if response.hovered() { lighten(primary_color, 0.1) } else { primary_color };
                    (egui::Color32::TRANSPARENT, egui::Stroke::NONE, text)
                }
                (ButtonType::Text, false, false) => {
                    let bg = if response.hovered() { egui::Color32::from_gray(240) } else { egui::Color32::TRANSPARENT };
                    (bg, egui::Stroke::NONE, text_color)
                }
                _ => (egui::Color32::WHITE, egui::Stroke::new(1.0, border_color), text_color),
            };

            ui.painter().rect(
                rect,
                2.0, // Border radius
                bg_fill,
                stroke_color,
                egui::StrokeKind::Inside,
            );

            let text_pos = rect.center() - text_job.size() / 2.0;
            ui.painter().galley(text_pos, text_job, text_color);

            if loading {
                 // Simple loading indicator (spinner) would go here
            }
        }

        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), &text));
        response
    }
}
