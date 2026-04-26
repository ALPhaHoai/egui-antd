/*
 * egui-antd Input Component Demo
 *
 * Mirrors the official Ant Design Input documentation examples.
 * See: https://ant.design/components/input
 */

use eframe::egui;
use egui_antd::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ant Design Component Template",
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
    // Basic
    basic_text: String,
    // Three sizes
    size_large: String,
    size_middle: String,
    size_small: String,
    // Variants
    variant_outlined: String,
    variant_filled: String,
    variant_borderless: String,
    variant_underlined: String,
    variant_search: String,
    // Search box
    search_basic: String,
    search_clear: String,
    search_addon: String,
    search_enter: String,
    search_large: String,
    search_suffix: String,
    // TextArea
    textarea_basic: String,
    textarea_max: String,
    // Autosizing
    autosize_basic: String,
    autosize_minmax: String,
    autosize_controlled: String,
    // OTP
    otp_formatter: String,
    otp_disabled: String,
    otp_length: String,
    otp_variant: String,
    otp_mask: String,
    otp_sep_node: String,
    otp_sep_fn: String,
    // prefix and suffix
    ps_username: String,
    ps_money: String,
    ps_money_disabled: String,
    ps_password_suffix: String,
    // Password box
    password_basic: String,
    password_disabled: String,
    // Clear
    clear_input: String,
    clear_textarea: String,
    // Character counting
    count_input: String,
    count_textarea: String,
    // Status
    status_error: String,
    status_warning: String,
    status_error_prefix: String,
    status_warning_prefix: String,

    /// Required for the demo_card's screenshot functionality
    pending_screenshot: Option<egui::Rect>,
    /// Persist clipboard to avoid re-opening connection every frame (important for Linux)
    clipboard: Option<arboard::Clipboard>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            basic_text: String::new(),
            size_large: String::new(),
            size_middle: String::new(),
            size_small: String::new(),
            variant_outlined: String::new(),
            variant_filled: String::new(),
            variant_borderless: String::new(),
            variant_underlined: String::new(),
            variant_search: String::new(),
            search_basic: String::new(),
            search_clear: String::new(),
            search_addon: String::new(),
            search_enter: String::new(),
            search_large: String::new(),
            search_suffix: String::new(),
            textarea_basic: String::new(),
            textarea_max: String::new(),
            autosize_basic: String::new(),
            autosize_minmax: String::new(),
            autosize_controlled: String::new(),
            otp_formatter: String::new(),
            otp_disabled: String::new(),
            otp_length: String::new(),
            otp_variant: String::new(),
            otp_mask: String::new(),
            otp_sep_node: String::new(),
            otp_sep_fn: String::new(),
            ps_username: String::new(),
            ps_money: String::new(),
            ps_money_disabled: "disabled".to_string(),
            ps_password_suffix: String::new(),
            password_basic: String::new(),
            password_disabled: "disabled password".to_string(),
            clear_input: String::new(),
            clear_textarea: String::new(),
            count_input: String::new(),
            count_textarea: String::new(),
            status_error: String::new(),
            status_warning: String::new(),
            status_error_prefix: String::new(),
            status_warning_prefix: String::new(),

            pending_screenshot: None,
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Screenshot Boilerplate: Do not modify unless you need to change capture logic ---
        let ppp = ctx.pixels_per_point();
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    if let Some(rect) = self.pending_screenshot.take() {
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
                    ui.heading("Input");
                    ui.label("A basic widget for getting the user input is a text field.");

                    ui.add_space(20.0);
                    ui.heading("When To Use");
                    ui.label("• A user input in a form field is needed.");
                    ui.label("• A search input is required.");

                    ui.add_space(30.0);
                    ui.heading("Examples");

                    let avail = ui.available_width();
                    let two_cols = avail > 800.0;
                    let gap = 40.0;
                    let col_width = if two_cols { ((avail - gap) / 2.0).max(0.0) } else { avail };

                    ui.horizontal_top(|outer_ui| {
                        outer_ui.allocate_ui_with_layout(
                            egui::vec2(col_width, 0.0),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.set_max_width(col_width);

                        // 1. Basic usage
                        if let Some(rect) = demo_card(ui, "Basic usage", "Basic usage example.", |ui| {
                            ui.add(Input::new(&mut self.basic_text).hint_text("Basic usage"));
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // 2. Three sizes of Input
                        if let Some(rect) = demo_card(
                            ui,
                            "Three sizes of Input",
                            "There are three sizes of an Input box: large (40px), middle (32px) and small (24px).",
                            |ui| {
                                let user_icon = |ui: &mut egui::Ui| {
                                    ui.label(
                                        egui::RichText::new(egui_phosphor::regular::USER)
                                            .color(egui::Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                                    );
                                };
                                ui.add(
                                    Input::new(&mut self.size_large)
                                        .size(InputSize::Large)
                                        .hint_text("large size")
                                        .prefix(user_icon),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    Input::new(&mut self.size_middle)
                                        .hint_text("default size")
                                        .prefix(user_icon),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    Input::new(&mut self.size_small)
                                        .size(InputSize::Small)
                                        .hint_text("small size")
                                        .prefix(user_icon),
                                );
                            },
                        ) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // 3. Variants
                        if let Some(rect) = demo_card(
                            ui,
                            "Variants",
                            "There are four variants for Input: outlined, filled, borderless and underlined.",
                            |ui| {
                                ui.add(
                                    Input::new(&mut self.variant_outlined)
                                        .variant(InputVariant::Outlined)
                                        .hint_text("Outlined"),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    Input::new(&mut self.variant_filled)
                                        .variant(InputVariant::Filled)
                                        .hint_text("Filled"),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    Input::new(&mut self.variant_borderless)
                                        .variant(InputVariant::Borderless)
                                        .hint_text("Borderless"),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    Input::new(&mut self.variant_underlined)
                                        .variant(InputVariant::Underlined)
                                        .hint_text("Underlined"),
                                );
                                ui.add_space(12.0);
                                ui.add(Search::new(&mut self.variant_search).hint_text("Search"));
                            },
                        ) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // 4. Search box
                        if let Some(rect) = demo_card(
                            ui,
                            "Search box",
                            "Example of creating a search box by grouping a standard input with a search button.",
                            |ui| {
                                ui.scope(|ui| {
                                    ui.set_max_width(250.0);
                                    ui.add(Search::new(&mut self.search_basic).hint_text("input search text"));
                                });
                                ui.add_space(8.0);
                                ui.scope(|ui| {
                                    ui.set_max_width(250.0);
                                    ui.add(
                                        Search::new(&mut self.search_clear)
                                            .hint_text("input search text")
                                            .allow_clear(true),
                                    );
                                });
                                ui.add_space(8.0);
                                ui.add(
                                    Input::new(&mut self.search_addon)
                                        .hint_text("input search text")
                                        .allow_clear(true)
                                        .addon_before(|ui| {
                                            ui.label("https://");
                                        })
                                        .suffix(|ui: &mut egui::Ui| {
                                            ui.label(
                                                egui::RichText::new(egui_phosphor::regular::MAGNIFYING_GLASS)
                                                    .color(egui::Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                                            );
                                        }),
                                );
                                ui.add_space(8.0);
                                ui.add(
                                    Search::new(&mut self.search_enter)
                                        .hint_text("input search text")
                                        .enter_button(true),
                                );
                                ui.add_space(8.0);
                                ui.add(
                                    Search::new(&mut self.search_large)
                                        .hint_text("input search text")
                                        .allow_clear(true)
                                        .enter_button_text("Search")
                                        .size(InputSize::Large),
                                );
                                ui.add_space(8.0);
                                ui.add(
                                    Search::new(&mut self.search_suffix)
                                        .hint_text("input search text")
                                        .enter_button_text("Search")
                                        .size(InputSize::Large)
                                        .suffix(|ui: &mut egui::Ui| {
                                            ui.label(
                                                egui::RichText::new(egui_phosphor::regular::MICROPHONE)
                                                    .size(16.0)
                                                    .color(egui::Color32::from_rgb(22, 119, 255)),
                                            );
                                        }),
                                );
                            },
                        ) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // 5. TextArea
                        if let Some(rect) = demo_card(ui, "TextArea", "For multi-line input.", |ui| {
                            ui.add(TextArea::new(&mut self.textarea_basic).auto_size(4, Some(4)));
                            ui.add_space(12.0);
                            ui.add(
                                TextArea::new(&mut self.textarea_max)
                                    .auto_size(4, Some(4))
                                    .hint_text("maxLength is 6")
                                    .max_length(6),
                            );
                        }) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                        // 6. Autosizing the height to fit the content
                        if let Some(rect) = demo_card(
                            ui,
                            "Autosizing the height to fit the content",
                            "autoSize prop for textarea makes the height adjust based on content.",
                            |ui| {
                                ui.add(
                                    TextArea::new(&mut self.autosize_basic)
                                        .hint_text("Autosize height based on content lines")
                                        .auto_size(1, None),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    TextArea::new(&mut self.autosize_minmax)
                                        .hint_text("Autosize height with minimum and maximum number of lines")
                                        .auto_size(2, Some(6)),
                                );
                                ui.add_space(12.0);
                                ui.add(
                                    TextArea::new(&mut self.autosize_controlled)
                                        .hint_text("Controlled autosize")
                                        .auto_size(3, Some(5)),
                                );
                            },
                        ) {
                            self.pending_screenshot = Some(rect);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                        }

                            },
                        );

                        // ============ RIGHT COLUMN ============
                        if two_cols {
                            outer_ui.add_space(gap);
                            outer_ui.allocate_ui_with_layout(
                                egui::vec2(col_width, 0.0),
                                egui::Layout::top_down(egui::Align::LEFT),
                                |ui| {
                                    ui.set_max_width(col_width);

                            // 7. OTP
                            if let Some(rect) = demo_card(ui, "OTP", "One time password input.", |ui| {
                                ui.label(
                                    egui::RichText::new("With formatter (Upcase)")
                                        .strong()
                                        .size(13.0),
                                );
                                ui.add_space(4.0);
                                ui.add(OTP::new(&mut self.otp_formatter).formatter(|s| s.to_uppercase()));
                                ui.add_space(12.0);

                                ui.label(egui::RichText::new("With Disabled").strong().size(13.0));
                                ui.add_space(4.0);
                                ui.add(OTP::new(&mut self.otp_disabled).disabled(true));
                                ui.add_space(12.0);

                                ui.label(egui::RichText::new("With Length (8)").strong().size(13.0));
                                ui.add_space(4.0);
                                ui.add(OTP::new(&mut self.otp_length).length(8));
                                ui.add_space(12.0);

                                ui.label(egui::RichText::new("With variant").strong().size(13.0));
                                ui.add_space(4.0);
                                ui.add(OTP::new(&mut self.otp_variant).variant(InputVariant::Filled));
                                ui.add_space(12.0);

                                ui.label(
                                    egui::RichText::new("With custom display character")
                                        .strong()
                                        .size(13.0),
                                );
                                ui.add_space(4.0);
                                ui.add(OTP::new(&mut self.otp_mask).mask('*'));
                                ui.add_space(12.0);

                                ui.label(
                                    egui::RichText::new("With custom ReactNode separator")
                                        .strong()
                                        .size(13.0),
                                );
                                ui.add_space(4.0);
                                ui.add(
                                    OTP::new(&mut self.otp_sep_node)
                                        .separator(|_, ui| {
                                            ui.label("/");
                                        }),
                                );
                                ui.add_space(12.0);

                                ui.label(
                                    egui::RichText::new("With custom function separator")
                                        .strong()
                                        .size(13.0),
                                );
                                ui.add_space(4.0);
                                ui.add(
                                    OTP::new(&mut self.otp_sep_fn).separator(|i, ui| {
                                        let color = if i & 1 == 1 {
                                            egui::Color32::from_rgb(255, 0, 0)
                                        } else {
                                            egui::Color32::from_rgb(0, 0, 255)
                                        };
                                        ui.label(
                                            egui::RichText::new("\u{2014}").color(color),
                                        );
                                    }),
                                );
                            }) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            // 8. prefix and suffix
                            if let Some(rect) = demo_card(
                                ui,
                                "prefix and suffix",
                                "Add a prefix or suffix icons inside input.",
                                |ui| {
                                    ui.add(
                                        Input::new(&mut self.ps_username)
                                            .hint_text("Enter your username")
                                            .prefix(|ui| {
                                                ui.label(
                                                    egui::RichText::new(egui_phosphor::regular::USER)
                                                        .color(egui::Color32::from_rgb(0, 0, 0).linear_multiply(0.25)),
                                                );
                                            })
                                            .suffix(|ui| {
                                                ui.label(
                                                    egui::RichText::new(egui_phosphor::regular::INFO)
                                                        .color(egui::Color32::from_rgb(0, 0, 0).linear_multiply(0.45)),
                                                )
                                                .on_hover_text("Extra information");
                                            }),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        Input::new(&mut self.ps_money)
                                            .prefix(|ui| {
                                                ui.label("\u{FFE5}");
                                            })
                                            .suffix(|ui| {
                                                ui.label("RMB");
                                            }),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        Input::new(&mut self.ps_money_disabled)
                                            .disabled(true)
                                            .prefix(|ui| {
                                                ui.label("\u{FFE5}");
                                            })
                                            .suffix(|ui| {
                                                ui.label("RMB");
                                            }),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        Password::new(&mut self.ps_password_suffix)
                                            .hint_text("input password support suffix"),
                                    );
                                },
                            ) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            // 9. Password box
                            if let Some(rect) = demo_card(ui, "Password box", "Input type of password.", |ui| {
                                ui.add(Password::new(&mut self.password_basic).hint_text("input password"));
                                ui.add_space(12.0);
                                ui.add(
                                    Password::new(&mut self.password_disabled)
                                        .disabled(true)
                                        .hint_text("disabled input password"),
                                );
                            }) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            // 10. With clear icon
                            if let Some(rect) = demo_card(
                                ui,
                                "With clear icon",
                                "Input box with the remove icon, click the icon to delete everything.",
                                |ui| {
                                    ui.add(
                                        Input::new(&mut self.clear_input)
                                            .hint_text("input with clear icon")
                                            .allow_clear(true),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        TextArea::new(&mut self.clear_textarea)
                                            .hint_text("textarea with clear icon")
                                            .allow_clear(true)
                                            .auto_size(3, Some(3)),
                                    );
                                },
                            ) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            // 11. With character counting
                            if let Some(rect) = demo_card(
                                ui,
                                "With character counting",
                                "Show character counting.",
                                |ui| {
                                    ui.add(
                                        Input::new(&mut self.count_input)
                                            .show_count(true)
                                            .max_length(20),
                                    );
                                    ui.add_space(16.0);
                                    ui.add(
                                        TextArea::new(&mut self.count_textarea)
                                            .show_count(true)
                                            .max_length(100)
                                            .hint_text("can resize")
                                            .auto_size(3, Some(6)),
                                    );
                                },
                            ) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }

                            // 12. Status
                            if let Some(rect) = demo_card(
                                ui,
                                "Status",
                                "Add status to Input with status, which could be error or warning.",
                                |ui| {
                                    ui.add(
                                        Input::new(&mut self.status_error)
                                            .status(InputStatus::Error)
                                            .hint_text("Error"),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        Input::new(&mut self.status_warning)
                                            .status(InputStatus::Warning)
                                            .hint_text("Warning"),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        Input::new(&mut self.status_error_prefix)
                                            .status(InputStatus::Error)
                                            .hint_text("Error with prefix")
                                            .prefix(|ui| {
                                                ui.label(egui_phosphor::regular::CLOCK);
                                            }),
                                    );
                                    ui.add_space(12.0);
                                    ui.add(
                                        Input::new(&mut self.status_warning_prefix)
                                            .status(InputStatus::Warning)
                                            .hint_text("Warning with prefix")
                                            .prefix(|ui| {
                                                ui.label(egui_phosphor::regular::CLOCK);
                                            }),
                                    );
                                },
                            ) {
                                self.pending_screenshot = Some(rect);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                            }
                            }); // end right column with_layout
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
    // Inset slightly so the group's stroke + inner_margin can never spill
    // past the column boundary into the adjacent column.
    let card_width = (ui.available_width() - 8.0).max(0.0);
    let mut screenshot_rect = None;

    let success_id = ui.id().with("screenshot_success").with(title);
    let now = ui.input(|i| i.time);
    let last_click: Option<f64> = ui.ctx().data(|d| d.get_temp(success_id));
    let is_success = last_click.is_some_and(|t| now - t < 2.0);

    if is_success {
        ui.ctx().request_repaint();
    }

    ui.allocate_ui(egui::vec2(card_width, 0.0), |ui| {
        ui.set_max_width(card_width);
        let response = ui
            .group(|ui| {
                ui.set_max_width(ui.available_width());
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
