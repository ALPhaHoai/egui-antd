mod antd;

use antd::{Button, ButtonSize, ButtonType};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Ant Design Button in egui",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    loading: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { loading: false }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ant Design Button Port");

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.add(Button::new("Primary Button").button_type(ButtonType::Primary)).clicked() {
                    println!("Primary clicked");
                }
                if ui.add(Button::new("Default Button")).clicked() {
                    println!("Default clicked");
                }
                ui.add(Button::new("Dashed Button").button_type(ButtonType::Dashed));
                ui.add(Button::new("Text Button").button_type(ButtonType::Text));
                ui.add(Button::new("Link Button").button_type(ButtonType::Link));
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add(Button::new("Danger Primary").button_type(ButtonType::Primary).danger(true));
                ui.add(Button::new("Danger Default").danger(true));
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add(Button::new("Large").size(ButtonSize::Large).button_type(ButtonType::Primary));
                ui.add(Button::new("Middle").size(ButtonSize::Middle).button_type(ButtonType::Primary));
                ui.add(Button::new("Small").size(ButtonSize::Small).button_type(ButtonType::Primary));
            });

            ui.add_space(20.0);

            ui.checkbox(&mut self.loading, "Loading state");
            ui.horizontal(|ui| {
                ui.add(Button::new("Loading Button").button_type(ButtonType::Primary).loading(self.loading));
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add(Button::new("Disabled").button_type(ButtonType::Primary).disabled(true));
                ui.add(Button::new("Disabled Default").disabled(true));
            });

            ui.add_space(20.0);

            ui.add(Button::new("Block Button").button_type(ButtonType::Primary).block(true));
        });
    }
}
