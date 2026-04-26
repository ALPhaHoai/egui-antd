use egui::{Color32, CornerRadius, Stroke, Vec2};

use crate::input::{InputSize, InputVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputStatus {
    #[default]
    Normal,
    Error,
    Warning,
}

pub fn get_input_metrics(size: InputSize) -> (Vec2, f32) {
    match size {
        InputSize::Large => (Vec2::new(11.0, 7.0), 40.0),
        InputSize::Middle => (Vec2::new(11.0, 4.0), 32.0),
        InputSize::Small => (Vec2::new(7.0, 0.0), 24.0),
    }
}

pub fn get_input_colors(variant: InputVariant, disabled: bool) -> (Color32, Stroke, Color32) {
    let bg = if disabled || variant == InputVariant::Filled {
        Color32::from_rgb(245, 245, 245)
    } else if variant == InputVariant::Borderless {
        Color32::TRANSPARENT
    } else {
        Color32::WHITE
    };

    let stroke = if variant == InputVariant::Outlined {
        Stroke::new(1.0, Color32::from_rgb(217, 217, 217))
    } else {
        Stroke::NONE
    };

    let text_color = if disabled {
        Color32::from_rgb(0, 0, 0).linear_multiply(0.25)
    } else {
        Color32::from_rgb(0, 0, 0).linear_multiply(0.88)
    };

    (bg, stroke, text_color)
}

pub fn get_input_rounding(variant: InputVariant) -> CornerRadius {
    if variant == InputVariant::Underlined || variant == InputVariant::Borderless {
        CornerRadius::ZERO
    } else {
        CornerRadius::same(6)
    }
}

pub fn get_status_color(status: InputStatus) -> Option<Color32> {
    match status {
        InputStatus::Normal => None,
        InputStatus::Error => Some(Color32::from_rgb(255, 77, 79)),
        InputStatus::Warning => Some(Color32::from_rgb(250, 173, 20)),
    }
}

pub fn get_interactive_stroke(
    base_stroke: Stroke,
    variant: InputVariant,
    disabled: bool,
    focused: bool,
    hovered: bool,
    status: InputStatus,
) -> Stroke {
    if disabled || variant == InputVariant::Borderless {
        return base_stroke;
    }

    if let Some(status_color) = get_status_color(status) {
        return if focused {
            Stroke::new(2.0, status_color)
        } else {
            Stroke::new(1.0, status_color)
        };
    }

    if focused {
        Stroke::new(2.0, Color32::from_rgb(22, 119, 255))
    } else if hovered {
        Stroke::new(1.0, Color32::from_rgb(22, 119, 255))
    } else {
        base_stroke
    }
}
