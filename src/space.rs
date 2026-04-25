use crate::button::ButtonPosition;
use egui::{Align, Layout, Ui, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpaceSize {
    Small,
    #[default]
    Middle,
    Large,
    Custom(f32),
}

impl SpaceSize {
    pub fn value(&self) -> f32 {
        match self {
            SpaceSize::Small => 8.0,
            SpaceSize::Middle => 16.0,
            SpaceSize::Large => 24.0,
            SpaceSize::Custom(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaceAlign {
    Start,
    End,
    Center,
    #[default]
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaceDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Space component for consistent spacing between components.
/// Mimics Ant Design's Space component.
pub struct Space {
    direction: SpaceDirection,
    size: SpaceSize,
    wrap: bool,
    align: Option<SpaceAlign>,
}

impl Default for Space {
    fn default() -> Self {
        Self::new()
    }
}

impl Space {
    pub fn new() -> Self {
        Self {
            direction: SpaceDirection::Horizontal,
            size: SpaceSize::Small,
            wrap: false,
            align: None,
        }
    }

    pub fn direction(mut self, direction: SpaceDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = SpaceDirection::Vertical;
        self
    }

    pub fn size(mut self, size: SpaceSize) -> Self {
        self.size = size;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn align(mut self, align: SpaceAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let spacing = self.size.value();

        let layout = match self.direction {
            SpaceDirection::Horizontal => {
                let mut l =
                    Layout::left_to_right(match self.align.unwrap_or(SpaceAlign::Baseline) {
                        SpaceAlign::Start => Align::Min,
                        SpaceAlign::End => Align::Max,
                        SpaceAlign::Center => Align::Center,
                        SpaceAlign::Baseline => Align::Center,
                    });
                if self.wrap {
                    l = l.with_main_wrap(true);
                }
                l
            }
            SpaceDirection::Vertical => {
                Layout::top_down(match self.align.unwrap_or(SpaceAlign::Start) {
                    SpaceAlign::Start => Align::Min,
                    SpaceAlign::End => Align::Max,
                    SpaceAlign::Center => Align::Center,
                    SpaceAlign::Baseline => Align::Min,
                })
            }
        };

        ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(spacing);
            add_contents(ui)
        })
        .inner
    }
}

/// Compact component for grouping components tightly.
/// Mimics Ant Design's Space.Compact component.
pub struct SpaceCompact<'a> {
    ui: &'a mut Ui,
    direction: SpaceDirection,
    block: bool,
}

impl<'a> SpaceCompact<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        Self {
            ui,
            direction: SpaceDirection::Horizontal,
            block: false,
        }
    }

    pub fn direction(mut self, direction: SpaceDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = SpaceDirection::Vertical;
        self
    }

    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    pub fn show<R>(self, add_contents: impl FnOnce(&mut SpaceCompactWriter<'a>) -> R) -> R {
        let mut writer = SpaceCompactWriter {
            ui: self.ui,
            direction: self.direction,
            block: self.block,
            items: Vec::new(),
        };
        let res = add_contents(&mut writer);
        writer.render();
        res
    }
}

pub struct SpaceCompactWriter<'a> {
    ui: &'a mut Ui,
    direction: SpaceDirection,
    block: bool,
    #[allow(clippy::type_complexity)]
    items: Vec<Box<dyn FnOnce(&mut Ui, ButtonPosition) + 'a>> /* FIXME: clippy::type_complexity */,
}

impl<'a> SpaceCompactWriter<'a> {
    /// Add a button to the compact group.
    pub fn add_button(&mut self, mut button: crate::button::Button<'a>) {
        if self.block {
            button = button.block(true);
        }
        self.items.push(Box::new(move |ui, pos| {
            ui.add(button.set_position(pos));
        }));
    }

    /// Add an arbitrary widget to the compact group.
    pub fn add_widget(&mut self, widget: impl egui::Widget + 'a) {
        self.items.push(Box::new(move |ui, _| {
            ui.add(widget);
        }));
    }

    fn render(self) {
        let layout = match self.direction {
            SpaceDirection::Horizontal => Layout::left_to_right(Align::Min),
            SpaceDirection::Vertical => Layout::top_down(Align::Min),
        };

        self.ui
            .allocate_ui_with_layout(self.ui.available_size(), layout, |ui| {
                // Overlap borders by 1px to avoid double borders
                ui.spacing_mut().item_spacing = match self.direction {
                    SpaceDirection::Horizontal => Vec2::new(-1.0, 0.0),
                    SpaceDirection::Vertical => Vec2::new(0.0, -1.0),
                };

                let count = self.items.len();
                for (i, item) in self.items.into_iter().enumerate() {
                    let position = if count <= 1 {
                        ButtonPosition::None
                    } else if i == 0 {
                        ButtonPosition::First
                    } else if i == count - 1 {
                        ButtonPosition::Last
                    } else {
                        ButtonPosition::Middle
                    };
                    item(ui, position);
                }
            });
    }
}
