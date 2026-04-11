use egui::{Id, Response, Ui, WidgetText, Color32, Vec2, Align, Layout, Sense};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabType {
    Line,
    Card,
    EditableCard,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabPosition {
    Top,
    End,
    Bottom,
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabSize {
    Small,
    Medium,
    Large,
}

pub struct TabPane<'a> {
    pub key: String,
    pub label: Box<dyn FnOnce(&mut Ui) -> Response + 'a>,
    pub closable: bool,
    pub disabled: bool,
}

impl<'a> TabPane<'a> {
    pub fn new(key: impl Into<String>, label: impl Into<WidgetText>) -> Self {
        let label_text = label.into();
        Self {
            key: key.into(),
            label: Box::new(move |ui: &mut Ui| {
                ui.label(label_text)
            }),
            closable: true,
            disabled: false,
        }
    }

    pub fn custom(key: impl Into<String>, label: impl FnOnce(&mut Ui) -> Response + 'a) -> Self {
        Self {
            key: key.into(),
            label: Box::new(label),
            closable: true,
            disabled: false,
        }
    }

    pub fn icon(key: impl Into<String>, label: impl Into<WidgetText>, icon: impl FnOnce(&mut Ui) -> Response + 'a) -> Self {
        let label_text = label.into();
        Self {
            key: key.into(),
            label: Box::new(move |ui: &mut Ui| {
                ui.horizontal(|ui| {
                    let mut resp = icon(ui);
                    resp |= ui.label(label_text);
                    resp
                }).inner
            }),
            closable: true,
            disabled: false,
        }
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub enum TabEditAction {
    Add,
    Remove(String),
}

pub struct TabBarExtraContent<'a> {
    pub left: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    pub right: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl<'a> TabBarExtraContent<'a> {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.left = Some(Box::new(content));
        self
    }

    pub fn right(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.right = Some(Box::new(content));
        self
    }
}

impl<'a> Default for TabBarExtraContent<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Tabs<'a> {
    id_source: Id,
    tab_type: TabType,
    tab_position: TabPosition,
    size: TabSize,
    panes: Vec<TabPane<'a>>,
    active_key: Option<String>,
    hide_add: bool,
    centered: bool,
    on_edit: Option<Box<dyn FnMut(TabEditAction) + 'a>>,
    extra: TabBarExtraContent<'a>,
}

impl<'a> Tabs<'a> {
    pub fn new(id_source: impl std::hash::Hash) -> Self {
        Self {
            id_source: Id::new(id_source),
            tab_type: TabType::Line,
            tab_position: TabPosition::Top,
            size: TabSize::Medium,
            panes: Vec::new(),
            active_key: None,
            hide_add: false,
            centered: false,
            on_edit: None,
            extra: TabBarExtraContent::new(),
        }
    }

    pub fn tab_type(mut self, tab_type: TabType) -> Self {
        self.tab_type = tab_type;
        self
    }

    pub fn tab_position(mut self, tab_position: TabPosition) -> Self {
        self.tab_position = tab_position;
        self
    }

    pub fn size(mut self, size: TabSize) -> Self {
        self.size = size;
        self
    }

    pub fn hide_add(mut self, hide_add: bool) -> Self {
        self.hide_add = hide_add;
        self
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    pub fn active_key(mut self, active_key: impl Into<String>) -> Self {
        self.active_key = Some(active_key.into());
        self
    }

    pub fn panes(mut self, panes: Vec<TabPane<'a>>) -> Self {
        self.panes = panes;
        self
    }

    pub fn on_edit(mut self, on_edit: impl FnMut(TabEditAction) + 'a) -> Self {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    pub fn extra(mut self, extra: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.extra = TabBarExtraContent::new().right(extra);
        self
    }

    pub fn extra_content(mut self, extra: TabBarExtraContent<'a>) -> Self {
        self.extra = extra;
        self
    }

    pub fn show(mut self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui, &str)) -> Response {
        let _id = self.id_source;
        let mut active_key = self.active_key.clone();

        if active_key.is_none() && !self.panes.is_empty() {
            active_key = Some(self.panes[0].key.clone());
        }

        let mut changed = false;

        let is_horizontal = match self.tab_position {
            TabPosition::Top | TabPosition::Bottom => true,
            TabPosition::End | TabPosition::Start => false,
        };

        let mut res = if is_horizontal {
            ui.vertical(|ui| {
                if self.tab_position == TabPosition::Top {
                    self.render_horizontal_tab_bar(ui, &mut active_key, &mut changed);
                    if let Some(key) = &active_key {
                        add_contents(ui, key);
                    }
                } else {
                    if let Some(key) = &active_key {
                        add_contents(ui, key);
                    }
                    self.render_horizontal_tab_bar(ui, &mut active_key, &mut changed);
                }
            })
        } else {
            ui.horizontal(|ui| {
                if self.tab_position == TabPosition::Start {
                    self.render_vertical_tab_bar(ui, &mut active_key, &mut changed);
                    if let Some(key) = &active_key {
                        ui.vertical(|ui| {
                            add_contents(ui, key);
                        });
                    }
                } else {
                    if let Some(key) = &active_key {
                        ui.vertical(|ui| {
                            add_contents(ui, key);
                        });
                    }
                    self.render_vertical_tab_bar(ui, &mut active_key, &mut changed);
                }
            })
        };

        if changed {
            res.response.mark_changed();
        }
        res.response
    }
}

impl<'a> Tabs<'a> {
    fn render_horizontal_tab_bar(&mut self, ui: &mut Ui, active_key: &mut Option<String>, changed: &mut bool) {
        let tabs_id = self.id_source.with("horiz_tabs");
        let overflow_id = tabs_id.with("overflow");
        let scroll_offset_id = tabs_id.with("scroll_offset");
        let max_scroll_offset_id = tabs_id.with("max_scroll_offset");
        
        let mut is_overflowing = ui.ctx().data_mut(|d| d.get_temp::<bool>(overflow_id).unwrap_or(false));
        let mut scroll_offset = ui.ctx().data_mut(|d| d.get_temp::<f32>(scroll_offset_id).unwrap_or(0.0));
        let _max_scroll_offset = ui.ctx().data_mut(|d| d.get_temp::<f32>(max_scroll_offset_id).unwrap_or(0.0));

        ui.horizontal(|ui| {
            if let Some(left) = self.extra.left.take() {
                left(ui);
                ui.add_space(8.0);
            }

            if is_overflowing {
                let can_scroll_left = scroll_offset > 1.0;
                ui.add_enabled_ui(can_scroll_left, |ui| {
                    if ui.add(egui::Button::new("<").frame(false)).clicked() {
                        scroll_offset -= 150.0;
                        ui.ctx().data_mut(|d| d.insert_temp(scroll_offset_id, scroll_offset));
                    }
                });
            }

            let right_content = self.extra.right.take();
            let extra_right_width = if right_content.is_some() { 50.0 } else { 0.0 };
            let right_arrow_width = if is_overflowing { 24.0 } else { 0.0 };
            let available_width = ui.available_width() - extra_right_width - right_arrow_width;

            let scroll_area = egui::scroll_area::ScrollArea::horizontal()
                .id_salt(tabs_id.with("scroll"))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .horizontal_scroll_offset(scroll_offset)
                .max_width(available_width.max(10.0));

            let scroll_res = scroll_area.show(ui, |ui| {
                if self.centered {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        self.render_tabs(ui, active_key, changed);
                    }).inner
                } else {
                    self.render_tabs(ui, active_key, changed);
                }
            });

            let current_max = (scroll_res.content_size.x - scroll_res.inner_rect.width()).max(0.0);
            let current_offset = scroll_res.state.offset.x;
            
            ui.ctx().data_mut(|d| {
                d.insert_temp(overflow_id, current_max > 0.0);
                d.insert_temp(scroll_offset_id, current_offset);
                d.insert_temp(max_scroll_offset_id, current_max);
            });

            if current_max > 0.0 {
                is_overflowing = true;
                let can_scroll_right = current_offset < current_max - 1.0;
                ui.add_enabled_ui(can_scroll_right, |ui| {
                    if ui.add(egui::Button::new(">").frame(false)).clicked() {
                        scroll_offset = current_offset + 150.0;
                        ui.ctx().data_mut(|d| d.insert_temp(scroll_offset_id, scroll_offset));
                    }
                });
            }

            if let Some(right) = right_content {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    right(ui);
                });
            }
        });
        ui.separator();
    }

    fn render_vertical_tab_bar(&mut self, ui: &mut Ui, active_key: &mut Option<String>, changed: &mut bool) {
        ui.vertical(|ui| {
            if let Some(left) = self.extra.left.take() {
                left(ui);
                ui.add_space(8.0);
            }

            if self.centered {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    self.render_tabs_vertical(ui, active_key, changed);
                });
            } else {
                self.render_tabs_vertical(ui, active_key, changed);
            }

            if let Some(right) = self.extra.right.take() {
                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    right(ui);
                });
            }
        });
        ui.separator();
    }

    fn render_tabs(&mut self, ui: &mut Ui, active_key: &mut Option<String>, changed: &mut bool) {
        let mut on_edit = self.on_edit.take();

        for pane in self.panes.drain(..) {
            let is_active = active_key.as_ref() == Some(&pane.key);

            let _text_color = if is_active {
                Color32::from_rgb(24, 144, 255)
            } else {
                Color32::from_rgb(0, 0, 0)
            };

            let _padding = match self.size {
                TabSize::Small => Vec2::new(8.0, 4.0),
                TabSize::Medium => Vec2::new(16.0, 8.0),
                TabSize::Large => Vec2::new(24.0, 12.0),
            };

            let mut close_clicked = false;

            ui.add_enabled_ui(!pane.disabled, |ui| {
                let resp = (pane.label)(ui);

                if ui.interact(resp.rect, ui.id().with(&pane.key), Sense::click()).clicked() {
                    *active_key = Some(pane.key.clone());
                    *changed = true;
                }

                if is_active {
                    let _ = resp.highlight();
                }

                if (self.tab_type == TabType::EditableCard) && pane.closable {
                    if ui.small_button("x").clicked() {
                        close_clicked = true;
                    }
                }
            });

            if close_clicked {
                if let Some(cb) = &mut on_edit {
                    cb(TabEditAction::Remove(pane.key.clone()));
                }
            }
            ui.add_space(8.0);
        }

        if self.tab_type == TabType::EditableCard && !self.hide_add {
            if ui.button("+").clicked() {
                if let Some(cb) = &mut on_edit {
                    cb(TabEditAction::Add);
                }
            }
        }

        self.on_edit = on_edit;
    }

    fn render_tabs_vertical(&mut self, ui: &mut Ui, active_key: &mut Option<String>, changed: &mut bool) {
        let mut on_edit = self.on_edit.take();

        for pane in self.panes.drain(..) {
            let is_active = active_key.as_ref() == Some(&pane.key);

            let mut close_clicked = false;

            ui.horizontal(|ui| {
                ui.add_enabled_ui(!pane.disabled, |ui| {
                    let resp = (pane.label)(ui);

                    if ui.interact(resp.rect, ui.id().with(&pane.key), Sense::click()).clicked() {
                        *active_key = Some(pane.key.clone());
                        *changed = true;
                    }

                    if is_active {
                        let _ = resp.highlight();
                    }

                    if (self.tab_type == TabType::EditableCard) && pane.closable {
                        if ui.small_button("x").clicked() {
                            close_clicked = true;
                        }
                    }
                });
            });

            if close_clicked {
                if let Some(cb) = &mut on_edit {
                    cb(TabEditAction::Remove(pane.key.clone()));
                }
            }
            ui.add_space(4.0);
        }

        if self.tab_type == TabType::EditableCard && !self.hide_add {
            if ui.button("+").clicked() {
                if let Some(cb) = &mut on_edit {
                    cb(TabEditAction::Add);
                }
            }
        }

        self.on_edit = on_edit;
    }
}
