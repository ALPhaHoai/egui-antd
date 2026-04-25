use eframe::egui;
use egui_antd::{
    Button, ButtonGroup, ButtonShape, ButtonSize, ButtonTheme, ButtonType, ComponentsTheme,
    ConfigProvider, Dropdown, Space, SpaceCompact, Theme, menu_item,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Ant Design Button in egui",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Setup Chinese font (Runtime load to avoid embedding large file)
            let font_path = "C:\\Windows\\Fonts\\msyh.ttc";
            if let Ok(font_data) = std::fs::read(font_path) {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "msyh".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "msyh".to_owned());
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .push("msyh".to_owned());

                // Add Phosphor icons
                egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

                cc.egui_ctx.set_fonts(fonts);
            }

            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp {
    size: ButtonSize,
    loadings: [bool; 4],
    pending_screenshot: Option<egui::Rect>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            size: ButtonSize::Middle,
            loadings: [false; 4],
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
                    // Crop the screenshot to the card area
                    let x = (rect.min.x * ppp).round() as usize;
                    let y = (rect.min.y * ppp).round() as usize;
                    let w = (rect.width() * ppp).round() as usize;
                    let h = (rect.height() * ppp).round() as usize;

                    // Ensure we don't go out of bounds
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

                        // Copy to clipboard
                        let pixels: Vec<u8> = cropped
                            .pixels
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
                ui.heading("Button");
                ui.label("Buttons are used to trigger an immediate operation.");

                ui.add_space(20.0);
                ui.heading("When to use");
                ui.label("A button means an operation (or a group of operations). Clicking it will trigger corresponding business logic.");

                ui.add_space(30.0);
                ui.heading("Code Demo");

                ui.columns(2, |columns| {
                    // Left Column
                    let ui = &mut columns[0];

                    if let Some(rect) = demo_card(ui, "#1 Button Type", "There are five types of buttons: primary, default, dashed, text and link. A primary button is used for the main action.", |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            ui.add(Button::new("Primary Button").button_type(ButtonType::Primary));
                            ui.add(Button::new("Default Button"));
                            ui.add(Button::new("Dashed Button").button_type(ButtonType::Dashed));
                            ui.add(Button::new("Text Button").button_type(ButtonType::Text));
                            ui.add(Button::new("Link Button").button_type(ButtonType::Link));
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#2 Icon", "You can add an icon using the icon property.", |ui| {
                        let search_icon = || egui::Image::new(egui::include_image!("../node_modules/@ant-design/icons-svg/inline-svg/outlined/search.svg"));

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 12.0;
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                ui.add(Button::new("").button_type(ButtonType::Primary).shape(ButtonShape::Circle).image(search_icon())).on_hover_text("search");
                                ui.add(Button::new("A").button_type(ButtonType::Primary).shape(ButtonShape::Circle));
                                ui.add(Button::new("Search").button_type(ButtonType::Primary).image(search_icon()));
                                ui.add(Button::new("").shape(ButtonShape::Circle).image(search_icon())).on_hover_text("search");
                                ui.add(Button::new("Search").image(search_icon()));
                            });
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                ui.add(Button::new("").shape(ButtonShape::Circle).image(search_icon())).on_hover_text("search");
                                ui.add(Button::new("Search").image(search_icon()));
                                ui.add(Button::new("").button_type(ButtonType::Dashed).shape(ButtonShape::Circle).image(search_icon())).on_hover_text("search");
                                ui.add(Button::new("Search").button_type(ButtonType::Dashed).image(search_icon()));
                                ui.add(Button::new("").image(search_icon()).href("https://www.google.com"));
                            });
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#3 Size", "Ant Design supports three sizes of buttons: small, medium and large. If a large or small button is desired, set the size property to either large or small respectively. Omit the size property for a button with the default medium size.", |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.size, ButtonSize::Large, "Large");
                            ui.selectable_value(&mut self.size, ButtonSize::Middle, "Medium");
                            ui.selectable_value(&mut self.size, ButtonSize::Small, "Small");
                        });

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(16.0);

                        let download_icon = || egui::Image::new(egui::include_image!("../node_modules/@ant-design/icons-svg/inline-svg/outlined/download.svg"));

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 12.0;
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                ui.add(Button::new("Primary").button_type(ButtonType::Primary).size(self.size));
                                ui.add(Button::new("Default").size(self.size));
                                ui.add(Button::new("Dashed").button_type(ButtonType::Dashed).size(self.size));
                            });

                            ui.add(Button::new("Link").button_type(ButtonType::Link).size(self.size));

                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                ui.add(Button::new("").button_type(ButtonType::Primary).size(self.size).image(download_icon()));
                                ui.add(Button::new("").button_type(ButtonType::Primary).size(self.size).shape(ButtonShape::Circle).image(download_icon()));
                                ui.add(Button::new("").button_type(ButtonType::Primary).size(self.size).shape(ButtonShape::Round).image(download_icon()));
                                ui.add(Button::new("Download").button_type(ButtonType::Primary).size(self.size).shape(ButtonShape::Round).image(download_icon()));
                                ui.add(Button::new("Download").button_type(ButtonType::Primary).size(self.size).image(download_icon()));
                            });
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#4 Disabled State", "Add disabled property to make a button unavailable.", |ui| {
                        ui.horizontal(|ui| {
                            ui.add(Button::new("Primary").button_type(ButtonType::Primary));
                            ui.add(Button::new("Primary(disabled)").button_type(ButtonType::Primary).disabled(true));
                        });
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add(Button::new("Default"));
                            ui.add(Button::new("Default(disabled)").disabled(true));
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "Custom disabled backgroundColor", "Customize the background color with disable (applicable to type default and dashed).", |ui| {
                        let theme = Theme {
                            components: ComponentsTheme {
                                button: ButtonTheme {
                                    default_bg_disabled: Some(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 25)), // 0.1 alpha
                                    dashed_bg_disabled: Some(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 102)), // 0.4 alpha
                                },
                            },
                        };

                        ConfigProvider::new().theme(theme).show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                ui.add(Button::new("Primary Button").button_type(ButtonType::Primary).disabled(true));
                                ui.add(Button::new("Default Button").disabled(true));
                                ui.add(Button::new("Dashed Button").button_type(ButtonType::Dashed).disabled(true));
                            });
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    // Right Column
                    let ui = &mut columns[1];

                    if let Some(rect) = demo_card(ui, "#5 Loading State", "Add loading property to set a button to loading state.", |ui| {
                        let poweroff_icon = || egui::Image::new(egui::include_image!("../node_modules/@ant-design/icons-svg/inline-svg/outlined/poweroff.svg"));
                        let sync_icon = || egui::Image::new(egui::include_image!("../node_modules/@ant-design/icons-svg/inline-svg/outlined/sync.svg"));

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 12.0;

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                ui.add(Button::new("Loading").button_type(ButtonType::Primary).loading(true));
                                ui.add(Button::new("Loading").button_type(ButtonType::Primary).size(ButtonSize::Small).loading(true));
                                ui.add(Button::new("").button_type(ButtonType::Primary).shape(ButtonShape::Circle).loading(true));
                                ui.add(Button::new("Loading Icon").button_type(ButtonType::Primary).loading_icon(sync_icon()).loading(true));
                            });

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                if ui.add(Button::new("Icon Start").button_type(ButtonType::Primary).loading(self.loadings[0])).clicked() {
                                    self.loadings[0] = true;
                                }
                                if ui.add(Button::new("Icon End").button_type(ButtonType::Primary).icon_placement(egui_antd::IconPlacement::End).loading(self.loadings[2])).clicked() {
                                    self.loadings[2] = true;
                                }
                                if ui.add(Button::new("Icon Replace").button_type(ButtonType::Primary).image(poweroff_icon()).loading(self.loadings[1])).clicked() {
                                    self.loadings[1] = true;
                                }
                                if ui.add(Button::new("").button_type(ButtonType::Primary).image(poweroff_icon()).loading(self.loadings[3])).clicked() {
                                    self.loadings[3] = true;
                                }
                                if ui.add(Button::new("Loading Icon").button_type(ButtonType::Primary).image(poweroff_icon()).loading_icon(sync_icon()).loading(self.loadings[3])).clicked() {
                                    self.loadings[3] = true;
                                }
                            });
                        });

                        // Reset loading states after some time (simulated)
                        for i in 0..4 {
                            if self.loadings[i] {
                                let id = ui.id().with("loading_timer").with(i);
                                let now = ui.input(|i| i.time);
                                let start_time: f64 = ui.ctx().data(|d| d.get_temp(id).unwrap_or(now));

                                if now - start_time > 3.0 {
                                    self.loadings[i] = false;
                                    ui.ctx().data_mut(|d| d.remove_temp::<f64>(id));
                                } else {
                                    ui.ctx().data_mut(|d| d.insert_temp(id, start_time));
                                    ui.ctx().request_repaint();
                                }
                            }
                        }
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#6 Ghost Button", "The ghost button is transparent and used on colored backgrounds.", |ui| {
                        egui::Frame::NONE
                            .fill(egui::Color32::from_rgb(190, 200, 200))
                            .inner_margin(8.0)
                            .corner_radius(4)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.add(Button::new("Primary").button_type(ButtonType::Primary).ghost(true));
                                    ui.add(Button::new("Default").ghost(true));
                                    ui.add(Button::new("Dashed").button_type(ButtonType::Dashed).ghost(true));
                                    ui.add(Button::new("Danger").button_type(ButtonType::Primary).danger(true).ghost(true));
                                });
                            });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#7 Danger Button", "Danger buttons have several types: primary, default, dashed, text and link.", |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.add(Button::new("Primary").button_type(ButtonType::Primary).danger(true));
                            ui.add(Button::new("Default").danger(true));
                            ui.add(Button::new("Dashed").button_type(ButtonType::Dashed).danger(true));
                            ui.add(Button::new("Text").button_type(ButtonType::Text).danger(true));
                            ui.add(Button::new("Link").button_type(ButtonType::Link).danger(true));
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#8 Multiple Buttons", "If you need several buttons, we recommend that you use 1 primary button + n secondary buttons. If there are more than three operations, you can group some of them into a Dropdown.", |ui| {
                        let ellipsis_icon = || egui::Image::new(egui::include_image!("../node_modules/@ant-design/icons-svg/inline-svg/outlined/ellipsis.svg"));
                        Space::new().vertical().show(ui, |ui| {
                            ui.add(Button::new("primary").button_type(ButtonType::Primary));
                            ui.add(Button::new("secondary"));

                            SpaceCompact::new(ui).show(|group| {
                                group.add_button(Button::new("Actions"));
                                group.add_button(Button::new("").image(ellipsis_icon()));
                            });

                            // Actual Dropdown demo is in #13, but for #8 we want the visual group
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#9 Button Group", "Multiple buttons can be grouped together.", |ui| {
                        ui.horizontal(|ui| {
                            ButtonGroup::new().show(ui, |group| {
                                group.add(Button::new("Cancel"));
                                group.add(Button::new("OK").button_type(ButtonType::Primary));
                            });

                            ui.add_space(20.0);

                            ButtonGroup::new().size(ButtonSize::Small).show(ui, |group| {
                                group.add(Button::new("L"));
                                group.add(Button::new("M"));
                                group.add(Button::new("R"));
                            });
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#10 Gradient Button", "Enhanced visual effect with gradient background.", |ui| {
                        ui.horizontal(|ui| {
                            ui.add(Button::new("Gradient Button").button_type(ButtonType::Gradient));
                            ui.add(Button::new("Round Gradient").button_type(ButtonType::Gradient).shape(ButtonShape::Round));
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#11 Block Button", "The block property will make the button fit its parent width.", |ui| {
                        ui.add(Button::new("Block Button").button_type(ButtonType::Primary).block(true));
                        ui.add_space(8.0);
                        ui.add(Button::new("Block Button").block(true));
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#13 Dropdown", "A dropdown menu for more operations.", |ui| {
                        ui.horizontal(|ui| {
                            Dropdown::new("demo_dropdown", Button::new("Hover me").button_type(ButtonType::Primary))
                                .show(ui, |ui| {
                                    menu_item(ui, "1st menu item");
                                    menu_item(ui, "2nd menu item");
                                    menu_item(ui, "3rd menu item");
                                });
                        });
                    }) {
                        self.pending_screenshot = Some(rect);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    }

                    if let Some(rect) = demo_card(ui, "#12 Link Jump", "The href property supports jumping to a specific address.", |ui| {
                        ui.add(Button::new("Ant Design").button_type(ButtonType::Link).href("https://ant.design"));
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

                // Demo Content (Top)
                ui.add_space(8.0);
                content(ui);
                ui.add_space(8.0);

                ui.separator();

                // Meta Info (Bottom)
                ui.horizontal(|ui| {
                    ui.strong(title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (icon, color, tooltip) = if is_success {
                            (
                                egui_phosphor::regular::CHECK,
                                egui::Color32::from_rgb(82, 196, 26),
                                "Copied!",
                            )
                        } else {
                            (
                                egui_phosphor::regular::CAMERA,
                                egui::Color32::from_gray(150),
                                "Copy screenshot to clipboard",
                            )
                        };

                        let btn = Button::new("")
                            .button_type(ButtonType::Text)
                            .size(ButtonSize::Small)
                            .icon(egui::RichText::new(icon).size(16.0).color(color));

                        if ui.add(btn).on_hover_text(tooltip).clicked() {
                            ui.ctx().data_mut(|d| d.insert_temp(success_id, now));
                            screenshot_rect = Some(egui::Rect::NOTHING); // Flag for capture
                        }

                        // Add a "Code" icon to match Ant Design's style
                        let code_btn = Button::new("")
                            .button_type(ButtonType::Text)
                            .size(ButtonSize::Small)
                            .icon(
                                egui::RichText::new(egui_phosphor::regular::CODE)
                                    .size(16.0)
                                    .color(egui::Color32::from_gray(150)),
                            );
                        ui.add(code_btn)
                            .on_hover_text("View code (Not implemented)");
                    });
                });
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(desc)
                        .size(12.0)
                        .color(egui::Color32::from_gray(120)),
                );
                ui.add_space(4.0);
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
