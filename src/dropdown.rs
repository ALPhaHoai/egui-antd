use crate::button::Button;
use egui::{Id, Response, Ui};

pub struct Dropdown<'a> {
    id: Id,
    button: Button<'a>,
}

impl<'a> Dropdown<'a> {
    pub fn new(id_source: impl std::hash::Hash, button: Button<'a>) -> Self {
        Self {
            id: Id::new(id_source),
            button,
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        menu_contents: impl FnOnce(&mut Ui) -> R,
    ) -> (Response, Option<R>) {
        let response = ui.add(self.button);

        let mut inner_res = None;
        egui::Popup::menu(&response).id(self.id).show(|ui| {
            ui.set_min_width(120.0);

            // Ant Design 5.0 menu styling
            egui::Frame::NONE
                .inner_margin(egui::Margin::same(4))
                .corner_radius(egui::CornerRadius::same(8))
                .fill(ui.visuals().window_fill())
                .show(ui, |ui| {
                    inner_res = Some(menu_contents(ui));
                });
        });

        (response, inner_res)
    }
}

pub fn menu_item(ui: &mut Ui, text: impl Into<egui::WidgetText>) -> Response {
    let text: egui::WidgetText = text.into();
    let text_size = 14.0;
    let font_id = egui::FontId::proportional(text_size);
    let padding = egui::vec2(12.0, 5.0);

    let color_text = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 224);
    let color_bg_hover = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 10); // ~rgba(0,0,0,0.04)

    let galley = ui
        .painter()
        .layout_no_wrap(text.text().to_string(), font_id, color_text);
    let desired_size = galley.size() + 2.0 * padding;

    let (rect, response) = ui.allocate_at_least(
        egui::vec2(ui.available_width(), desired_size.y),
        egui::Sense::click(),
    );

    if ui.is_rect_visible(rect) {
        let is_hover = response.hovered();
        let bg_fill = if is_hover {
            color_bg_hover
        } else {
            egui::Color32::TRANSPARENT
        };

        if is_hover {
            ui.painter()
                .rect_filled(rect, egui::CornerRadius::same(4), bg_fill);
        }

        let text_pos = rect.left_center() + egui::vec2(padding.x, -galley.size().y / 2.0);
        ui.painter().galley(text_pos, galley, color_text);
    }

    if response.clicked() {
        // No direct close_any_popup in current egui version's Memory
        // But clicking an item inside a popup usually closes it if CloseOnClick is set on the popup itself
    }

    response
}
