use eframe::egui;
use egui_antd::{Tabs, TabPane, TabType, TabPosition, TabSize, TabBarExtraContent, TabEditAction, Button, ButtonType, ButtonSize};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Ant Design Tabs in egui",
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

struct MyApp {
    active_key: String,
    editable_active_key: String,
    panes: Vec<(String, String)>,
    next_id: usize,
    size: TabSize,
    position: TabPosition,
    pending_screenshot: Option<egui::Rect>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            active_key: "1".to_string(),
            editable_active_key: "1".to_string(),
            panes: vec![
                ("1".to_string(), "Tab 1".to_string()),
                ("2".to_string(), "Tab 2".to_string()),
                ("3".to_string(), "Tab 3".to_string()),
            ],
            next_id: 4,
            size: TabSize::Medium,
            position: TabPosition::Top,
            pending_screenshot: None,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for event in ctx.input(|i| i.events.clone()) {
            if let egui::Event::Screenshot { image, .. } = event {
                if let Some(rect) = self.pending_screenshot.take() {
                    let ppp = ctx.pixels_per_point();
                    let x = (rect.min.x * ppp).round() as usize;
                    let y = (rect.min.y * ppp).round() as usize;
                    let w = (rect.width() * ppp).round() as usize;
                    let h = (rect.height() * ppp).round() as usize;

                    let x = x.min(image.width());
                    let y = y.min(image.height());
                    let w = w.min(image.width() - x);
                    let h = h.min(image.height() - y);

                    if w > 0 && h > 0 {
                        let mut cropped = egui::ColorImage::new([w, h], vec![egui::Color32::BLACK; w * h]);
                        for cy in 0..h {
                            for cx in 0..w {
                                cropped[(cx, cy)] = image[(x + cx, y + cy)];
                            }
                        }

                        let pixels: Vec<u8> = cropped.pixels
                            .iter()
                            .flat_map(|color| [color.r(), color.g(), color.b(), color.a()])
                            .collect();

                        let image_data = arboard::ImageData {
                            width: w,
                            height: h,
                            bytes: pixels.into(),
                        };

                        if let Ok(mut clipboard) = arboard::Clipboard::new() {
                            let _ = clipboard.set_image(image_data);
                        }
                    }
                }
            }
        }

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Tabs");
                ui.label("Tabs make it easy to switch between different views.");

                ui.add_space(20.0);
                ui.heading("When to use");
                ui.label("Ant Design Tabs can be used to organize and allow navigation between groups of content that are at the same level of hierarchy.");

                ui.add_space(30.0);
                ui.heading("Code Demo");

                ui.columns(2, |columns| {
                    // Left Column
                    let ui = &mut columns[0];

                    if let Some(rect) = demo_card(ui, "#1 Basic", "Default tabs.", |ui| {
                        let mut tabs = Tabs::new("basic")
                            .items(vec![
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ]);

                        if !self.active_key.is_empty() {
                            tabs = tabs.active_key(&self.active_key);
                        }

                        let mut new_key = None;
                        tabs.show(ui, |ui, key| {
                            if key == self.active_key {
                                ui.label(format!("Content of Tab Pane {}", key));
                            } else {
                                new_key = Some(key.to_string());
                            }
                        });

                        // Internal interaction updates:
                        // The Tabs component uses ui.interact which changes internal state.
                        // In a real app we'd use a callback or the component would expose state.
                        // For now we'll just demonstrate the visual appearance.
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#2 Icon", "Tabs with icons.", |ui| {
                        Tabs::new("icons")
                            .items(vec![
                                TabPane::icon("1", "Apple", |ui| ui.label(egui_phosphor::regular::APPLE_LOGO)),
                                TabPane::icon("2", "Android", |ui| ui.label(egui_phosphor::regular::ANDROID_LOGO)),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of Tab Pane {}", key));
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#3 Size", "Large size tabs are usually used in page header, and small size tabs are used in Modal.", |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.size, TabSize::Large, "Large");
                            ui.selectable_value(&mut self.size, TabSize::Medium, "Medium");
                            ui.selectable_value(&mut self.size, TabSize::Small, "Small");
                        });

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(16.0);

                        Tabs::new("size_demo")
                            .size(self.size)
                            .items(vec![
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of Tab Pane {}", key));
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#4 Position", "Tab's position: left, right, top or bottom.", |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.position, TabPosition::Top, "Top");
                            ui.selectable_value(&mut self.position, TabPosition::Bottom, "Bottom");
                            ui.selectable_value(&mut self.position, TabPosition::Start, "Left");
                            ui.selectable_value(&mut self.position, TabPosition::End, "Right");
                        });

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(16.0);

                        let height = if matches!(self.position, TabPosition::Start | TabPosition::End) { 200.0 } else { 100.0 };

                        ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                            Tabs::new("position_demo")
                                .tab_position(self.position)
                                .items(vec![
                                    TabPane::new("1", "Tab 1"),
                                    TabPane::new("2", "Tab 2"),
                                    TabPane::new("3", "Tab 3"),
                                ])
                                .show(ui, |ui, key| {
                                    ui.label(format!("Content of Tab Pane {}", key));
                                });
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    // Right Column
                    let ui = &mut columns[1];

                    if let Some(rect) = demo_card(ui, "#5 Card Type", "Tab with card style.", |ui| {
                        Tabs::new("card_demo")
                            .tab_type(TabType::Card)
                            .items(vec![
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of Tab Pane {}", key));
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#6 Editable Card", "Only card type Tabs support adding and closable. + is only for Card type.", |ui| {
                        let mut edit_action = None;

                        let panes = self.panes.iter().map(|(k, t)| TabPane::new(k, t)).collect();

                        let mut tabs = Tabs::new("editable_demo")
                            .tab_type(TabType::EditableCard)
                            .items(panes)
                            .on_edit(|action| {
                                edit_action = Some(action);
                            });

                        if !self.panes.iter().any(|(k, _)| k == &self.editable_active_key) {
                            if !self.panes.is_empty() {
                                self.editable_active_key = self.panes[0].0.clone();
                            }
                        }

                        tabs = tabs.active_key(&self.editable_active_key);

                        tabs.show(ui, |ui, key| {
                            ui.label(format!("Content of Tab Pane {}", key));
                        });

                        if let Some(action) = edit_action {
                            match action {
                                TabEditAction::Add => {
                                    let id = self.next_id.to_string();
                                    self.panes.push((id.clone(), format!("New Tab {}", self.next_id)));
                                    self.editable_active_key = id;
                                    self.next_id += 1;
                                }
                                TabEditAction::Remove(key) => {
                                    if let Some(pos) = self.panes.iter().position(|(k, _)| k == &key) {
                                        self.panes.remove(pos);
                                        if key == self.editable_active_key {
                                            if !self.panes.is_empty() {
                                                self.editable_active_key = self.panes[pos.min(self.panes.len() - 1)].0.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#7 Extra Content", "You can add extra actions to the right or left of Tabs.", |ui| {
                        Tabs::new("extra_demo")
                            .extra_content(TabBarExtraContent::new()
                                .left(|ui| { ui.add(Button::new("Left Extra").size(ButtonSize::Small)); })
                                .right(|ui| { ui.add(Button::new("Right Extra").size(ButtonSize::Small)); })
                            )
                            .items(vec![
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of Tab Pane {}", key));
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#8 Centered", "Centered tabs.", |ui| {
                        Tabs::new("centered_demo")
                            .centered(true)
                            .items(vec![
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of Tab Pane {}", key));
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#9 Disabled", "Disabled a tab.", |ui| {
                        Tabs::new("disabled_demo")
                            .items(vec![
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Disabled Tab").disabled(true),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of Tab Pane {}", key));
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }
                });

                ui.add_space(50.0);
            });
        });
    }
}

fn demo_card(ui: &mut egui::Ui, title: &str, desc: &str, content: impl FnOnce(&mut egui::Ui)) -> Option<egui::Rect> {
    let mut screenshot_rect = None;

    let success_id = ui.id().with("screenshot_success").with(title);
    let now = ui.input(|i| i.time);
    let last_click: Option<f64> = ui.ctx().data(|d| d.get_temp(success_id));
    let is_success = last_click.map_or(false, |t| now - t < 2.0);

    if is_success {
        ui.ctx().request_repaint();
    }

    ui.vertical(|ui| {
        let response = ui.group(|ui| {
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
            ui.label(egui::RichText::new(desc).size(12.0).color(egui::Color32::from_gray(120)));
            ui.add_space(12.0);
            content(ui);
        }).response;

        if screenshot_rect.is_some() {
            screenshot_rect = Some(response.rect);
        }
    });
    ui.add_space(16.0);
    screenshot_rect
}
