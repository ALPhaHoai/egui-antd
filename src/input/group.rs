use egui::{Response, Ui};

use crate::input::InputSize;

pub struct InputGroup {
    size: InputSize,
    compact: bool,
}

impl InputGroup {
    pub fn new() -> Self {
        Self {
            size: InputSize::Middle,
            compact: false,
        }
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn show(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> Response {
        let spacing = if self.compact { 0.0 } else { 8.0 };

        let resp = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = spacing;
            add_contents(ui);
        });

        resp.response
    }
}

impl Default for InputGroup {
    fn default() -> Self {
        Self::new()
    }
}
