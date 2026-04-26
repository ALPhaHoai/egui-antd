use eframe::egui;
use egui_antd::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ant Design Checkbox Demo",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(MyApp::default()))
        }),
    )
}

const ALL_OPTIONS: [&str; 3] = ["Apple", "Pear", "Orange"];

struct MyApp {
    basic_checked: bool,
    check_all_items: Vec<String>,
    group_values: Vec<String>,
    pending_screenshot: Option<egui::Rect>,
    clipboard: Option<arboard::Clipboard>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            basic_checked: false,
            check_all_items: vec!["Apple".to_string(), "Orange".to_string()],
            group_values: vec!["Apple".to_string(), "Pear".to_string()],
            pending_screenshot: None,
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    if let Some(rect) = self.pending_screenshot.take() {
                        let ppp = ctx.pixels_per_point();
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
            ConfigProvider::new().show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Checkbox");
                    ui.label("Collect user's choices.");

                    ui.add_space(20.0);
                    ui.heading("When to use");
                    ui.label("Used for selecting multiple values from several options.");

                    ui.add_space(30.0);
                    ui.heading("Code Demo");

                    let num_columns = if ui.available_width() > 800.0 { 2 } else { 1 };
                    ui.columns(num_columns, |columns| {
                        // --- Left Column ---
                        let ui = &mut columns[0];

                        if let Some(rect) = demo_card(ui, "Basic", "Basic usage of checkbox.", |ui| {
                            ui.add(Checkbox::new(&mut self.basic_checked).label("Checkbox"));
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        if let Some(rect) = demo_card(ui, "Disabled", "Disabled checkbox.", |ui| {
                            ui.vertical(|ui| {
                                let mut unchecked = false;
                                ui.add(Checkbox::new(&mut unchecked).disabled(true));
                                ui.add_space(4.0);
                                let mut indet = false;
                                ui.add(Checkbox::new(&mut indet).indeterminate(true).disabled(true));
                                ui.add_space(4.0);
                                let mut checked = true;
                                ui.add(Checkbox::new(&mut checked).disabled(true));
                            });
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        if let Some(rect) = demo_card(ui, "Check All", "The indeterminate property can help you to achieve a 'check all' effect.", |ui| {
                            let all_checked = self.check_all_items.len() == ALL_OPTIONS.len();
                            let indeterminate = !self.check_all_items.is_empty() && !all_checked;
                            let mut check_all_state = all_checked;

                            if ui.add(
                                Checkbox::new(&mut check_all_state)
                                    .label("Check all")
                                    .indeterminate(indeterminate),
                            ).changed() {
                                if check_all_state {
                                    self.check_all_items = ALL_OPTIONS.iter().map(|s| s.to_string()).collect();
                                } else {
                                    self.check_all_items.clear();
                                }
                            }

                            ui.separator();

                            CheckboxGroup::new(&mut self.check_all_items)
                                .options(ALL_OPTIONS.iter().map(|s| CheckboxOption::new(*s, *s)).collect())
                                .show(ui);
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // --- Right Column ---
                        if columns.len() > 1 {
                            let ui = &mut columns[1];

                            if let Some(rect) = demo_card(ui, "Checkbox Group", "Generate a group of checkboxes from an array.", |ui| {
                                CheckboxGroup::new(&mut self.group_values)
                                    .options(ALL_OPTIONS.iter().map(|s| CheckboxOption::new(*s, *s)).collect())
                                    .show(ui);
                            }) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            if let Some(rect) = demo_card(ui, "Disabled Group", "Disable all checkboxes in a group.", |ui| {
                                let mut disabled_values = vec!["Apple".to_string()];
                                CheckboxGroup::new(&mut disabled_values)
                                    .options(ALL_OPTIONS.iter().map(|s| CheckboxOption::new(*s, *s)).collect())
                                    .disabled(true)
                                    .show(ui);
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

fn demo_card(
    ui: &mut egui::Ui,
    title: &str,
    desc: &str,
    content: impl FnOnce(&mut egui::Ui),
) -> Option<egui::Rect> {
    let mut screenshot_rect = None;

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
                            egui::Color32::from_rgb(82, 196, 26)
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
                            screenshot_rect = Some(egui::Rect::NOTHING);
                        }
                    });
                });
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(desc)
                        .size(12.0)
                        .color(egui::Color32::from_gray(115)),
                );
                ui.add_space(12.0);
                content(ui);
            })
            .response;

        if screenshot_rect.is_some() {
            screenshot_rect = Some(response.rect);
        }
    });
    ui.add_space(16.0);
    screenshot_rect
}
