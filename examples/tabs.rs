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

impl MyApp {
    fn handle_edit(&mut self, action: TabEditAction) {
        match action {
            TabEditAction::Add => {
                let id = self.next_id.to_string();
                self.panes.push((id.clone(), "New Tab".to_string()));
                self.editable_active_key = id;
                self.next_id += 1;
            }
            TabEditAction::Remove(key) => {
                if let Some(pos) = self.panes.iter().position(|(k, _)| k == &key) {
                    self.panes.remove(pos);
                    if key == self.editable_active_key && !self.panes.is_empty() {
                        let new_pos = if pos > 0 { pos - 1 } else { 0 };
                        self.editable_active_key = self.panes[new_pos].0.clone();
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn show_demo(
    ui: &mut egui::Ui,
    pending_screenshot: &mut Option<egui::Rect>,
    title: &str,
    desc: &str,
    content: impl FnOnce(&mut egui::Ui),
) {
    show_demo_ext(ui, pending_screenshot, title, desc, false, content);
}

fn show_demo_ext(
    ui: &mut egui::Ui,
    pending_screenshot: &mut Option<egui::Rect>,
    title: &str,
    desc: &str,
    no_padding: bool,
    content: impl FnOnce(&mut egui::Ui),
) {
    if let Some(rect) = demo_card(ui, title, desc, no_padding, content) {
        *pending_screenshot = Some(rect);
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            active_key: "1".to_string(),
            editable_active_key: "3".to_string(),
            panes: vec![
                ("1".to_string(), "Tab 1".to_string()),
                ("2".to_string(), "Tab 2".to_string()),
                ("3".to_string(), "Tab 3".to_string()),
            ],
            next_id: 4,
            size: TabSize::Small,
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

                    show_demo_ext(ui, &mut self.pending_screenshot, "Basic", "Default tabs.", true, |ui| {
                        // The active key is managed by the app state. `active_key` method provides two-way binding.
                        Tabs::new("basic")
                            .active_key(&mut self.active_key)
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Icon", "Tabs with icons.", true, |ui| {
                        Tabs::new("icons")
                            .default_active_key("2")
                            .items([
                                TabPane::icon("1", "Tab 1", |ui| ui.label(egui_phosphor::regular::APPLE_LOGO)),
                                TabPane::icon("2", "Tab 2", |ui| ui.label(egui_phosphor::regular::ANDROID_LOGO)),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Custom Label", "Customized tab label.", true, |ui| {
                        Tabs::new("custom_label")
                            .items([
                                TabPane::custom("1", |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui_phosphor::regular::USER);
                                        ui.add_space(4.0);
                                        ui.strong("User Info")
                                    }).response
                                }),
                                TabPane::new("2", "Settings"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Position", "Tab's position: left, right, top or bottom.", true, |ui| {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
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
                                .items([
                                    TabPane::new("1", "Tab 1"),
                                    TabPane::new("2", "Tab 2"),
                                    TabPane::new("3", "Tab 3"),
                                ])
                                .show(ui, |ui, key| {
                                    ui.add_space(8.0);
                                    ui.label(format!("Content of tab {}", key));
                                });
                        });
                    });

                    // Right Column
                    let ui = &mut columns[1];

                    let mut edit_action = None;
                    show_demo_ext(ui, &mut self.pending_screenshot, "Size & Type", "Large size tabs are usually used in page header, and small size tabs are used in Modal. All types support size.", true, |ui| {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            ui.selectable_value(&mut self.size, TabSize::Large, "Large");
                            ui.selectable_value(&mut self.size, TabSize::Medium, "Medium");
                            ui.selectable_value(&mut self.size, TabSize::Small, "Small");
                        });

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(16.0);

                        ui.label("Line (Basic)");
                        Tabs::new("size_line")
                            .size(self.size)
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });

                        ui.add_space(24.0);
                        ui.label("Card");
                        Tabs::new("size_card")
                            .size(self.size)
                            .tab_type(TabType::Card)
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of card tab {}", key));
                            });

                        ui.add_space(24.0);
                        ui.label("Editable Card");
                        let panes: Vec<_> = self.panes.iter().map(|(k, t)| TabPane::new(k, t)).collect();
                        Tabs::new("size_editable")
                            .size(self.size)
                            .tab_type(TabType::EditableCard)
                            .items(panes)
                            .active_key(&mut self.editable_active_key)
                            .on_edit(|action| {
                                edit_action = Some(action);
                            })
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of editable tab {}", key));
                            });
                    });

                    // Handle tab additions and removals
                    if let Some(action) = edit_action {
                        self.handle_edit(action);
                    }

                    show_demo_ext(ui, &mut self.pending_screenshot, "Extra Content", "You can add extra actions to the right or left of Tabs, or even both side of Tabs.", true, |ui| {
                        Tabs::new("extra_demo")
                            .default_active_key("2")
                            .extra_content(TabBarExtraContent::new()
                                .left(|ui| { ui.add(Button::new("Left Extra Action").size(ButtonSize::Small)); })
                                .right(|ui| { ui.add(Button::new("Right Extra Action").size(ButtonSize::Small)); })
                            )
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Centered", "Centered tabs.", true, |ui| {
                        Tabs::new("centered_demo")
                            .centered(true)
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Disabled", "Disabled a tab.", true, |ui| {
                        Tabs::new("disabled_demo")
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Disabled Tab").disabled(true),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Custom Gutter", "The spacing between tabs.", true, |ui| {
                        Tabs::new("gutter_demo")
                            .gutter(64.0)
                            .items([
                                TabPane::new("1", "Tab 1"),
                                TabPane::new("2", "Tab 2"),
                                TabPane::new("3", "Tab 3"),
                            ])
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Sliding", "Tab bar can be scrolled when there is no enough space.", true, |ui| {
                        let panes: Vec<_> = (1..=20).map(|i| TabPane::new(i.to_string(), format!("Tab {}", i))).collect();
                        // Force a narrow width to demonstrate horizontal scrolling
                        ui.set_max_width(300.0);
                        Tabs::new("sliding_demo")
                            .items(panes)
                            .show(ui, |ui, key| {
                                ui.label(format!("Content of tab {}", key));
                            });
                    });

                    show_demo_ext(ui, &mut self.pending_screenshot, "Vertical Scroll", "Vertical tabs with scroll arrows.", true, |ui| {
                        let panes: Vec<_> = (1..=20).map(|i| TabPane::new(i.to_string(), format!("Tab {}", i))).collect();
                        ui.allocate_ui(egui::vec2(ui.available_width(), 200.0), |ui| {
                            Tabs::new("vertical_scroll_demo")
                                .tab_position(TabPosition::Start)
                                .items(panes)
                                .show(ui, |ui, key| {
                                    ui.add_space(8.0);
                                    ui.label(format!("Content of tab {}", key));
                                });
                        });
                    });
                });

                ui.add_space(50.0);
            });
        });
    }
}

fn demo_card(ui: &mut egui::Ui, title: &str, desc: &str, no_padding: bool, content: impl FnOnce(&mut egui::Ui)) -> Option<egui::Rect> {
    let mut screenshot_rect = None;

    let success_id = ui.id().with("screenshot_success").with(title);
    let now = ui.input(|i| i.time);
    let last_click: Option<f64> = ui.ctx().data(|d| d.get_temp(success_id));
    let is_success = last_click.map_or(false, |t| now - t < 2.0);

    if is_success {
        ui.ctx().request_repaint();
    }

    ui.vertical(|ui| {
        let frame = if no_padding {
            egui::Frame::group(ui.style()).inner_margin(0.0)
        } else {
            egui::Frame::group(ui.style())
        };

        let response = frame.show(ui, |ui| {
            ui.set_width(ui.available_width());

            // Demo Content (Top)
            if !no_padding {
                ui.add_space(8.0);
            }
            content(ui);
            if !no_padding {
                ui.add_space(8.0);
            }

            ui.separator();

            // Meta Info (Bottom)
            ui.horizontal(|ui| {
                if no_padding {
                    ui.add_space(8.0);
                }
                ui.strong(title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if no_padding {
                        ui.add_space(8.0);
                    }
                    let (icon, color, tooltip) = if is_success {
                        (egui_phosphor::regular::CHECK, egui::Color32::from_rgb(82, 196, 26), "Copied!")
                    } else {
                        (egui_phosphor::regular::CAMERA, egui::Color32::from_gray(150), "Copy screenshot to clipboard")
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
            ui.horizontal(|ui| {
                if no_padding {
                    ui.add_space(8.0);
                }
                ui.label(egui::RichText::new(desc).size(12.0).color(egui::Color32::from_gray(120)));
            });
            ui.add_space(4.0);
        }).response;

        if screenshot_rect.is_some() {
            screenshot_rect = Some(response.rect);
        }
    });
    ui.add_space(16.0);
    screenshot_rect
}
