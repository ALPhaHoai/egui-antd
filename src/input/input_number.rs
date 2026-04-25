use egui::{Response, Ui, Widget};

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
}

// Minimal implementation to get it compiling
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
}

impl<'a, T> Widget for InputNumber<'a, T>
where
    T: egui::emath::Numeric + std::fmt::Display + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let InputNumber {
            value,
            min,
            max,
            step,
            size,
            variant: _,
            disabled,
            controls: _,
        } = self;

        // Simple implementation: use a DragValue instead of a custom text box with spinner
        // Real implementation needs to look like an Input component with up/down arrows

        let height = match size {
            InputSize::Large => 40.0,
            InputSize::Middle => 32.0,
            InputSize::Small => 24.0,
        };

        ui.horizontal(|ui| {
            ui.set_min_height(height);

            let mut drag_value = egui::DragValue::new(value).speed(step.to_f64());

            if let (Some(min), Some(max)) = (min, max) {
                drag_value = drag_value.range(min.to_f64()..=max.to_f64());
            } else if let Some(min) = min {
                drag_value = drag_value.range(min.to_f64()..=f64::INFINITY);
            } else if let Some(max) = max {
                drag_value = drag_value.range(f64::NEG_INFINITY..=max.to_f64());
            }

            ui.add_enabled(!disabled, drag_value)
        })
        .inner
    }
}
