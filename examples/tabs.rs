use eframe::egui;
use egui_antd::tabs::{Tabs, TabPane, TabSize, TabPosition};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-antd tabs demo",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    active_key: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui, frame);
        });
    }
    
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Tabs Demo");
        ui.separator();

        let mut tabs = Tabs::new("demo_tabs")
            .panes(vec![
                TabPane::new("tab1", "Tab 1"),
                TabPane::new("tab2", "Tab 2"),
                TabPane::new("tab3", "Tab 3").disabled(true),
            ]);
            
        if let Some(key) = &self.active_key {
            tabs = tabs.active_key(key);
        }
            
        let res = tabs.show(ui, |ui, key| {
            match key {
                "tab1" => { ui.label("Content of Tab Pane 1"); },
                "tab2" => { ui.label("Content of Tab Pane 2"); },
                "tab3" => { ui.label("Content of Tab Pane 3"); },
                _ => { ui.label("Unknown tab content"); },
            }
        });
        
        // No simple way to extract the new active_key from the basic Tabs implementation unless we store state differently.
        // For basic interaction the Memory caching we added handles the state internally, so tabs work.
            
        ui.add_space(20.0);
        ui.heading("Centered Tabs");
        
        Tabs::new("centered_tabs")
            .centered(true)
            .panes(vec![
                TabPane::new("tab1", "Tab 1"),
                TabPane::new("tab2", "Tab 2"),
            ])
            .show(ui, |ui, key| {
                ui.label(format!("Centered {key} Content"));
            });
            
        ui.add_space(20.0);
        ui.heading("Tab Sizes");
        
        Tabs::new("small_tabs")
            .size(TabSize::Small)
            .panes(vec![
                TabPane::new("tab1", "Small Tab 1"),
                TabPane::new("tab2", "Small Tab 2"),
            ])
            .show(ui, |ui, key| {
                ui.label(format!("{key} Content"));
            });
            
        ui.add_space(20.0);
        ui.heading("Tab Positions");
        
        ui.horizontal(|ui| {
            Tabs::new("left_tabs")
                .tab_position(TabPosition::Start)
                .panes(vec![
                    TabPane::new("tab1", "Left 1"),
                    TabPane::new("tab2", "Left 2"),
                ])
                .show(ui, |ui, key| {
                    ui.label(format!("{key} Content"));
                });
        });
    }
}
