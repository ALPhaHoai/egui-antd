/*
 * egui-antd Component Demo Template
 *
 * Use this template to create new component examples that match the library's
 * visual documentation style.
 *
 * To add a new example:
 * 1. Copy this template to `examples/<your_component>.rs`.
 * 2. Update the `MyApp` struct with your component's specific state.
 * 3. Add demo sections in the `update` method using the `demo_card` helper.
 * 4. Run with: `cargo run --example <your_component>`
 */

use eframe::egui;
use egui_antd::*;

fn main() -> eframe::Result {
    // 1280x800 is a safer default for most displays than 1500x900
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ant Design Component Template",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Merge Phosphor icons into existing font definitions
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp {
    basic_text: String,
    sized_large: String,
    sized_middle: String,
    sized_small: String,
    disabled_text: String,
    variant_outlined: String,
    variant_filled: String,
    variant_borderless: String,
    variant_underlined: String,
    number_basic: f64,

    /// Required for the demo_card's screenshot functionality
    pending_screenshot: Option<egui::Rect>,
    /// Persist clipboard to avoid re-opening connection every frame (important for Linux)
    clipboard: Option<arboard::Clipboard>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            basic_text: String::new(),
            sized_large: String::new(),
            sized_middle: String::new(),
            sized_small: String::new(),
            disabled_text: "Disabled text".to_string(),
            variant_outlined: String::new(),
            variant_filled: String::new(),
            variant_borderless: String::new(),
            variant_underlined: String::new(),
            number_basic: 3.14,

            pending_screenshot: None,
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Screenshot Boilerplate: Do not modify unless you need to change capture logic ---
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    if let Some(rect) = self.pending_screenshot.take() {
                        let ppp = ctx.pixels_per_point();
                        // Saturate negative coordinates to 0 for safety
                        let x = (rect.min.x * ppp).round().max(0.0) as usize;
                        let y = (rect.min.y * ppp).round().max(0.0) as usize;
                        let w = (rect.width() * ppp).round().max(0.0) as usize;
                        let h = (rect.height() * ppp).round().max(0.0) as usize;

                        let x = x.min(image.width());
                        let y = y.min(image.height());
                        let w = w.min(image.width() - x);
                        let h = h.min(image.height() - y);

                        if w > 0 && h > 0 {
                            let mut cropped =
                                egui::ColorImage::new([w, h], vec![egui::Color32::BLACK; w * h]);
                            for cy in 0..h {
                                for cx in 0..w {
                                    cropped[(cx, cy)] = image[(x + cx, y + cy)];
                                }
                            }

                            if let Some(clipboard) = &mut self.clipboard {
                                let image_data = arboard::ImageData {
                                    width: w,
                                    height: h,
                                    bytes: std::borrow::Cow::Borrowed(bytemuck::cast_slice(
                                        &cropped.pixels,
                                    )),
                                };
                                let _ = clipboard.set_image(image_data);
                            }
                        }
                    }
                }
            }
        });

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            // Use ConfigProvider to ensure Ant Design theme is applied
            ConfigProvider::new().show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Input");
                    ui.label("A basic widget for getting the user input is a text field.");

                    ui.add_space(20.0);
                    ui.heading("When to use");
                    ui.label("Detailed explanation of when this component should be used in an interface.");

                    ui.add_space(30.0);
                    ui.heading("Code Demo");

                    // 2-Column Demo Grid (Adaptive)
                    let num_columns = if ui.available_width() > 800.0 { 2 } else { 1 };
                    ui.columns(num_columns, |columns| {
                        // --- Left Column ---
                        let ui = &mut columns[0];

                        if let Some(rect) = demo_card(ui, "#1 Basic", "Basic usage example.", |ui| {
                            ui.add(Input::new(&mut self.basic_text).hint_text("Basic usage"));
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        if let Some(rect) = demo_card(ui, "#2 Three Sizes", "Add size to Input. There are three sizes.", |ui| {
                            ui.add(Input::new(&mut self.sized_large).size(InputSize::Large).hint_text("Large size"));
                            ui.add_space(8.0);
                            ui.add(Input::new(&mut self.sized_middle).size(InputSize::Middle).hint_text("Middle size (default)"));
                            ui.add_space(8.0);
                            ui.add(Input::new(&mut self.sized_small).size(InputSize::Small).hint_text("Small size"));
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        if let Some(rect) = demo_card(ui, "#2.5 With Count", "Show character count.", |ui| {
                            ui.add(Input::new(&mut self.basic_text).hint_text("Basic usage").show_count(true).max_length(20));
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        if let Some(rect) = demo_card(ui, "#3 Disabled", "Input can be disabled.", |ui| {
                            ui.add(Input::new(&mut self.disabled_text).disabled(true));
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // --- Right Column ---
                        if columns.len() > 1 {
                            let ui = &mut columns[1];

                            if let Some(rect) = demo_card(ui, "#4 Variants", "There are four variants for Input component.", |ui| {
                                ui.add(Input::new(&mut self.variant_outlined).variant(InputVariant::Outlined).hint_text("Outlined"));
                                ui.add_space(8.0);
                                ui.add(Input::new(&mut self.variant_filled).variant(InputVariant::Filled).hint_text("Filled"));
                                ui.add_space(8.0);
                                ui.add(Input::new(&mut self.variant_borderless).variant(InputVariant::Borderless).hint_text("Borderless"));
                                ui.add_space(8.0);
                                ui.add(Input::new(&mut self.variant_underlined).variant(InputVariant::Underlined).hint_text("Underlined"));
                            }) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            if let Some(rect) = demo_card(ui, "#5 InputNumber Basic", "Numeric-only input box.", |ui| {
                                ui.add(InputNumber::new(&mut self.number_basic).min(0.0).max(10.0).step(0.1));
                            }) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }
                        }
                    });

                    ui.add_space(50.0);
                });
            });
        });
    }
}

/// A standard documentation card with a title, description, and screenshot button.
fn demo_card(
    ui: &mut egui::Ui,
    title: &str,
    desc: &str,
    content: impl FnOnce(&mut egui::Ui),
) -> Option<egui::Rect> {
    let mut screenshot_rect = None;

    // Feedback state
    let success_id = ui.id().with("screenshot_success").with(title);
    let now = ui.input(|i| i.time);
    let last_click: Option<f64> = ui.ctx().data(|d| d.get_temp(success_id));
    let is_success = last_click.map_or(false, |t| now - t < 2.0);

    if is_success {
        ui.ctx().request_repaint();
    }

    ui.vertical(|ui| {
        let response = ui
            .group(|ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.strong(title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let icon = if is_success {
                            egui_phosphor::regular::CHECK
                        } else {
                            egui_phosphor::regular::CAMERA
                        };

                        let color = if is_success {
                            egui::Color32::from_rgb(82, 196, 26) // Ant Design Success Green
                        } else {
                            egui::Color32::from_gray(150)
                        };

                        let tooltip = if is_success {
                            "Copied!"
                        } else {
                            "Copy screenshot to clipboard"
                        };

                        let btn = Button::new("")
                            .button_type(ButtonType::Text)
                            .size(ButtonSize::Small)
                            .icon(egui::RichText::new(icon).size(16.0).color(color));

                        if ui.add(btn).on_hover_text(tooltip).clicked() {
                            ui.ctx().data_mut(|d| d.insert_temp(success_id, now));
                            screenshot_rect = Some(egui::Rect::NOTHING); // Flag for capture
                        }
                    });
                });
                ui.add_space(4.0); // marginXXS
                ui.label(
                    egui::RichText::new(desc)
                        .size(12.0)
                        .color(egui::Color32::from_gray(115)),
                ); // colorTextDescription
                ui.add_space(12.0); // paddingSM
                content(ui);
            })
            .response;

        // If the button inside was clicked, we want the rect of the entire group
        if screenshot_rect.is_some() {
            screenshot_rect = Some(response.rect);
        }
    });
    ui.add_space(16.0);
    screenshot_rect
}
