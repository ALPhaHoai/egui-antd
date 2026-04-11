use eframe::egui;
use egui_antd::tabs::{Tabs, TabPane, TabPosition, TabType, TabEditAction};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-antd tabs demo",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    active_key: Option<String>,
    card_active_key: Option<String>,
    editable_active_key: Option<String>,
    panes: Vec<(String, String)>,
    next_id: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            active_key: None,
            card_active_key: None,
            editable_active_key: None,
            panes: vec![
                ("tab1".to_string(), "Tab 1".to_string()),
                ("tab2".to_string(), "Tab 2".to_string()),
                ("tab3".to_string(), "Tab 3".to_string()),
            ],
            next_id: 4,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Tabs Demo");
                ui.separator();
                
                ui.heading("Basic Line Tabs");
                let mut tabs = Tabs::new("demo_tabs")
                    .panes(vec![
                        TabPane::new("tab1", "Tab 1"),
                        TabPane::new("tab2", "Tab 2"),
                        TabPane::new("tab3", "Tab 3"),
                    ]);
                    
                if let Some(key) = &self.active_key {
                    tabs = tabs.active_key(key);
                }
                    
                tabs.show(ui, |ui, key| {
                    match key {
                        "tab1" => { ui.label("Content of Tab Pane 1"); },
                        "tab2" => { ui.label("Content of Tab Pane 2"); },
                        "tab3" => { ui.label("Content of Tab Pane 3"); },
                        _ => { ui.label("Unknown tab content"); },
                    }
                });
                    
                ui.add_space(20.0);
                ui.heading("Card Tabs");
                
                let mut card_tabs = Tabs::new("card_tabs")
                    .tab_type(TabType::Card)
                    .panes(vec![
                        TabPane::new("tab1", "Card Tab 1"),
                        TabPane::new("tab2", "Card Tab 2"),
                        TabPane::new("tab3", "Card Tab 3"),
                    ]);
                    
                if let Some(key) = &self.card_active_key {
                    card_tabs = card_tabs.active_key(key);
                }
                
                card_tabs.show(ui, |ui, key| {
                    ui.label(format!("Content of {}", key));
                });
                
                ui.add_space(20.0);
                ui.heading("Editable Card Tabs");
                
                let mut panes = vec![];
                for (key, title) in &self.panes {
                    panes.push(TabPane::new(key.clone(), title.clone()));
                }
                
                let mut edit_action = None;
                
                let mut editable_tabs = Tabs::new("editable_tabs")
                    .tab_type(TabType::EditableCard)
                    .panes(panes)
                    .on_edit(|action| {
                        edit_action = Some(action);
                    });
                    
                if let Some(key) = &self.editable_active_key {
                    editable_tabs = editable_tabs.active_key(key);
                }
                
                editable_tabs.show(ui, |ui, key| {
                    ui.label(format!("Content of {}", key));
                });
                
                if let Some(action) = edit_action {
                    match action {
                        TabEditAction::Add => {
                            let key = format!("tab{}", self.next_id);
                            let title = format!("Tab {}", self.next_id);
                            self.panes.push((key.clone(), title));
                            self.editable_active_key = Some(key);
                            self.next_id += 1;
                        }
                        TabEditAction::Remove(key) => {
                            if let Some(idx) = self.panes.iter().position(|(k, _)| k == &key) {
                                self.panes.remove(idx);
                                if Some(key) == self.editable_active_key {
                                    if !self.panes.is_empty() {
                                        let new_idx = idx.saturating_sub(1);
                                        self.editable_active_key = Some(self.panes[new_idx].0.clone());
                                    } else {
                                        self.editable_active_key = None;
                                    }
                                }
                            }
                        }
                    }
                }
                
                ui.add_space(20.0);
                ui.heading("Card Tabs (Vertical)");
                
                Tabs::new("card_vertical")
                    .tab_type(TabType::Card)
                    .tab_position(TabPosition::Start)
                    .panes(vec![
                        TabPane::new("tab1", "Card Tab 1"),
                        TabPane::new("tab2", "Card Tab 2"),
                        TabPane::new("tab3", "Card Tab 3"),
                    ])
                    .show(ui, |ui, key| {
                        ui.label(format!("Content of {}", key));
                    });
            });
        });
    }
}
