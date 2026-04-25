use egui::{Response, Ui, Widget, Layout, Align, Vec2, Key};
use crate::input::{InputSize, InputVariant, Input};

pub struct OTP<'a> {
    text: &'a mut String,
    length: usize,
    size: InputSize,
    variant: InputVariant,
    disabled: bool,
    mask: Option<char>,
    #[allow(clippy::type_complexity)]
    formatter: Option<Box<dyn Fn(String) -> String>>,
}

impl<'a> OTP<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            length: 6,
            size: InputSize::Middle,
            variant: InputVariant::Outlined,
            disabled: false,
            mask: None,
            formatter: None,
        }
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
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

    pub fn mask(mut self, mask: char) -> Self {
        self.mask = Some(mask);
        self
    }

    pub fn formatter(mut self, formatter: impl Fn(String) -> String + 'static) -> Self {
        self.formatter = Some(Box::new(formatter));
        self
    }
}

impl<'a> Widget for OTP<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut chars: Vec<char> = self.text.chars().collect();
        chars.resize(self.length, ' ');

        let id = ui.id().with("otp");
        
        let gap = match self.size {
            InputSize::Large => 16.0,
            InputSize::Middle => 8.0,
            InputSize::Small => 8.0,
        };

        let width = match self.size {
            InputSize::Large => 48.0,
            InputSize::Middle => 40.0,
            InputSize::Small => 32.0,
        };

        let mut next_focus: Option<usize> = None;
        let mut prev_focus: Option<usize> = None;
        let mut responses = Vec::new();
        let mut text_changed = false;

        let response = ui.with_layout(Layout::left_to_right(Align::Center).with_main_wrap(false), |ui| {
            ui.spacing_mut().item_spacing.x = gap;

            for i in 0..self.length {
                let cell_id = id.with(i);
                
                let cell_text = if chars[i] != ' ' {
                    chars[i].to_string()
                } else {
                    String::new()
                };

                let mut cell_string = cell_text.clone();
                let inner_response = ui.scope(|ui| {
                    ui.set_max_width(width);
                    let mut inp = Input::new(&mut cell_string)
                        .size(self.size)
                        .variant(self.variant)
                        .disabled(self.disabled)
                        .max_length(1);
                        
                    if let Some(_m) = self.mask {
                        inp = inp.password(true);
                    }
                    
                    let resp = inp.ui(ui);
                    
                    // Note: This relies on the Input internally giving focus to its TextEdit,
                    // but we intercept key events here to do auto-focus advancing
                    resp
                }).inner;

                if inner_response.has_focus() {
                    ui.input(|i| {
                        if i.key_pressed(Key::ArrowRight) {
                            next_focus = Some(i + 1);
                        } else if i.key_pressed(Key::ArrowLeft) {
                            prev_focus = Some(i.saturating_sub(1));
                        } else if i.key_pressed(Key::Backspace) && cell_string.is_empty() {
                            prev_focus = Some(i.saturating_sub(1));
                        }
                    });
                }
                
                if inner_response.changed() {
                    if cell_string.chars().count() > 1 {
                        cell_string = cell_string.chars().last().unwrap_or(' ').to_string();
                    }
                    
                    if !cell_string.is_empty() {
                        chars[i] = cell_string.chars().next().unwrap_or(' ');
                        next_focus = Some(i + 1);
                    } else {
                        chars[i] = ' ';
                    }
                    text_changed = true;
                } else if cell_string != cell_text {
                    chars[i] = cell_string.chars().next().unwrap_or(' ');
                    text_changed = true;
                }

                // Add an explicit id for focus management
                ui.memory_mut(|mem| {
                    if let Some(nf) = next_focus {
                        if nf == i {
                            mem.request_focus(cell_id);
                        }
                    }
                    if let Some(pf) = prev_focus {
                        if pf == i {
                            mem.request_focus(cell_id);
                        }
                    }
                });

                responses.push(inner_response);
            }
        });
        
        let mut final_response = response.response;
        for r in responses.iter() {
            final_response = final_response.union(r.clone());
        }

        if text_changed {
            let mut result = chars.into_iter().collect::<String>();
            if let Some(formatter) = &self.formatter {
                result = formatter(result);
            }
            *self.text = result;
            final_response.mark_changed();
        }

        final_response
    }
}
