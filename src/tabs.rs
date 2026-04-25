use crate::button::{Button, ButtonType};
use egui::{Align, Color32, Id, LayerId, Layout, Order, Response, Sense, Ui, Vec2, WidgetText};

const ANIMATION_DURATION: f32 = 0.2;
const SCROLL_INCREMENT: f32 = 150.0;
const ARROW_WIDTH: f32 = 24.0;
const DEFAULT_GUTTER: f32 = 32.0;
const ANTD_BLUE: Color32 = Color32::from_rgb(22, 119, 255);
const ANTD_BORDER_COLOR: Color32 = Color32::from_rgb(240, 240, 240);
const ANTD_COLOR_FILL_ALTER: Color32 = Color32::from_gray(250);

/// Type of the tabs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabType {
    /// Default line style.
    Line,
    /// Card style.
    Card,
    /// Editable card style with add and close buttons.
    EditableCard,
}

/// Position of the tabs bar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabPosition {
    /// Top position (horizontal).
    Top,
    /// End/Right position (vertical).
    End,
    /// Bottom position (horizontal).
    Bottom,
    /// Start/Left position (vertical).
    Start,
}

/// Size of the tabs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabSize {
    /// Small size, suitable for modals.
    Small,
    /// Medium size (default).
    Medium,
    /// Large size, suitable for page headers.
    Large,
}

pub type TabLabelClosure<'a> = Box<dyn Fn(&mut Ui) -> Response + 'a>;
pub type TabExtraClosure<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;
pub type TabEditClosure<'a> = Box<dyn FnMut(TabEditAction) + 'a>;
pub type TabChangeClosure<'a> = Box<dyn FnMut(String) + 'a>;

/// A single tab pane.
pub struct TabPane<'a> {
    /// Unique identifier for the tab.
    pub(crate) key: String,
    /// Content of the tab label.
    pub(crate) label: TabLabelClosure<'a>,
    /// Whether the tab is closable (only for `EditableCard`).
    pub(crate) closable: bool,
    /// Whether the tab is disabled.
    pub(crate) disabled: bool,
}

impl<'a> TabPane<'a> {
    /// Create a new tab pane with a text label.
    pub fn new(key: impl Into<String>, label: impl Into<WidgetText>) -> Self {
        let label_text = label.into();
        Self {
            key: key.into(),
            label: Box::new(move |ui: &mut Ui| {
                let mut text = label_text.clone();
                if let Some(color) = ui.visuals().override_text_color {
                    text = text.color(color);
                }
                if ui.visuals().override_text_color == Some(ANTD_BLUE) {
                    text = text.strong();
                }
                ui.add(
                    egui::Label::new(text)
                        .selectable(false)
                        .sense(egui::Sense::empty()),
                )
            }),
            closable: true,
            disabled: false,
        }
    }

    /// Create a new tab pane with a custom label.
    pub fn custom(key: impl Into<String>, label: impl Fn(&mut Ui) -> Response + 'a) -> Self {
        Self {
            key: key.into(),
            label: Box::new(label),
            closable: true,
            disabled: false,
        }
    }

    /// Create a new tab pane with an icon and a text label.
    pub fn icon(
        key: impl Into<String>,
        label: impl Into<WidgetText>,
        icon: impl Fn(&mut Ui) -> Response + 'a,
    ) -> Self {
        let label_text = label.into();
        Self {
            key: key.into(),
            label: Box::new(move |ui: &mut Ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let mut resp = icon(ui);
                    let mut text = label_text.clone();
                    if let Some(color) = ui.visuals().override_text_color {
                        text = text.color(color);
                    }
                    if ui.visuals().override_text_color == Some(ANTD_BLUE) {
                        text = text.strong();
                    }
                    resp |= ui.add(
                        egui::Label::new(text)
                            .selectable(false)
                            .sense(egui::Sense::empty()),
                    );
                    resp
                })
                .inner
            }),
            closable: true,
            disabled: false,
        }
    }

    /// Set whether the tab is closable.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether the tab is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Get the tab key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get whether the tab is closable.
    pub fn is_closable(&self) -> bool {
        self.closable
    }

    /// Get whether the tab is disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
}

/// Actions for editing tabs.
#[derive(Clone, Debug, PartialEq)]
pub enum TabEditAction {
    /// Add a new tab.
    Add,
    /// Remove a tab with the given key.
    Remove(String),
}

/// Extra content to be displayed in the tab bar.
pub struct TabBarExtraContent<'a> {
    /// Content to be displayed on the left side of the tab bar.
    pub left: Option<TabExtraClosure<'a>>,
    /// Content to be displayed on the right side of the tab bar.
    pub right: Option<TabExtraClosure<'a>>,
}

impl<'a> TabBarExtraContent<'a> {
    /// Create a new empty extra content.
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
        }
    }

    /// Set the left extra content.
    pub fn left(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.left = Some(Box::new(content));
        self
    }

    /// Set the right extra content.
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

/// A tabs component.
pub struct Tabs<'a> {
    id_source: Id,
    tab_type: TabType,
    tab_position: TabPosition,
    size: TabSize,
    panes: Vec<TabPane<'a>>,
    active_key: Option<&'a mut String>,
    default_active_key: Option<String>,
    hide_add: bool,
    centered: bool,
    on_edit: Option<TabEditClosure<'a>>,
    on_change: Option<TabChangeClosure<'a>>,
    extra: TabBarExtraContent<'a>,
    gutter: Option<f32>,
}

/// Response from showing the tabs.
pub struct TabsResponse {
    /// The response from the entire widget.
    pub response: Response,
    /// The currently active tab key.
    pub active_key: String,
    /// The edit action that occurred, if any.
    pub edit_action: Option<TabEditAction>,
}

impl std::ops::Deref for TabsResponse {
    type Target = Response;
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl<'a> Tabs<'a> {
    /// Create a new tabs component with a unique ID source.
    pub fn new(id_source: impl std::hash::Hash) -> Self {
        Self {
            id_source: Id::new(id_source),
            tab_type: TabType::Line,
            tab_position: TabPosition::Top,
            size: TabSize::Medium,
            panes: Vec::new(),
            active_key: None,
            default_active_key: None,
            hide_add: false,
            centered: false,
            on_edit: None,
            on_change: None,
            extra: TabBarExtraContent::new(),
            gutter: None,
        }
    }

    /// Set the tab type.
    pub fn tab_type(mut self, tab_type: TabType) -> Self {
        self.tab_type = tab_type;
        self
    }

    /// Set the tab position.
    pub fn tab_position(mut self, tab_position: TabPosition) -> Self {
        self.tab_position = tab_position;
        self
    }

    /// Set the tab size.
    pub fn size(mut self, size: TabSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether to hide the add button (only for `EditableCard`).
    pub fn hide_add(mut self, hide_add: bool) -> Self {
        self.hide_add = hide_add;
        self
    }

    /// Set whether the tabs are centered (only for horizontal tabs).
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Set the active tab key (two-way binding).
    pub fn active_key(mut self, active_key: &'a mut String) -> Self {
        self.active_key = Some(active_key);
        self
    }

    /// Set the default active tab key.
    pub fn default_active_key(mut self, default_active_key: impl Into<String>) -> Self {
        self.default_active_key = Some(default_active_key.into());
        self
    }

    /// Set the gutter (spacing between tabs).
    pub fn gutter(mut self, gutter: f32) -> Self {
        self.gutter = Some(gutter);
        self
    }

    /// Set the list of tab panes.
    pub fn items(mut self, panes: impl IntoIterator<Item = TabPane<'a>>) -> Self {
        self.panes = panes.into_iter().collect();
        self
    }

    /// Add a single tab pane.
    pub fn pane(mut self, pane: TabPane<'a>) -> Self {
        self.panes.push(pane);
        self
    }

    /// Set the callback for editing tabs (add/remove).
    pub fn on_edit(mut self, on_edit: impl FnMut(TabEditAction) + 'a) -> Self {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    /// Set the callback for when the active tab changes.
    pub fn on_change(mut self, on_change: impl FnMut(String) + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Set the left extra content.
    pub fn extra_left(mut self, extra: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.extra = self.extra.left(extra);
        self
    }

    /// Set the right extra content.
    pub fn extra_right(mut self, extra: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.extra = self.extra.right(extra);
        self
    }

    /// Set the extra content for the tab bar.
    pub fn extra_content(mut self, extra: TabBarExtraContent<'a>) -> Self {
        self.extra = extra;
        self
    }

    /// Show the tabs widget.
    pub fn show(
        mut self,
        ui: &mut Ui,
        mut add_contents: impl FnMut(&mut Ui, &str),
    ) -> TabsResponse {
        let mut edit_action = None;
        let id = ui.make_persistent_id(self.id_source);

        let last_active_key = ui.data(|d| d.get_temp::<String>(id.with("active_key")));
        let mut active_key = if let Some(key) = &self.active_key {
            Some((**key).clone())
        } else {
            last_active_key.clone().or(self.default_active_key.clone())
        };

        if active_key.is_none() && !self.panes.is_empty() {
            active_key = Some(self.panes[0].key.clone());
        }

        // If it was initialized from default_active_key, we need to save it to memory
        if active_key.is_some()
            && last_active_key.is_none()
            && let Some(key) = &active_key
        {
            ui.ctx()
                .data_mut(|d| d.insert_temp(id.with("active_key"), key.clone()));
        }

        // Ensure active_key points to a valid pane
        if let Some(key) = &active_key
            && !self.panes.iter().any(|p| &p.key == key)
        {
            active_key = self.panes.first().map(|p| p.key.clone());
        }

        let mut scroll_to_active = active_key != last_active_key;
        let current_active_key_before = active_key.clone();
        let mut changed = false;

        let mut res = ui
            .push_id(id, |ui| {
                let is_horizontal = match self.tab_position {
                    TabPosition::Top | TabPosition::Bottom => true,
                    TabPosition::End | TabPosition::Start => false,
                };

                // Keyboard navigation
                let has_focus = ui.memory(|m| m.has_focus(id));

                // Make the tab bar focusable
                ui.memory_mut(|m| m.interested_in_focus(id, ui.layer_id()));

                if has_focus {
                    let enabled_indices: Vec<usize> = self
                        .panes
                        .iter()
                        .enumerate()
                        .filter(|(_, p)| !p.disabled)
                        .map(|(i, _)| i)
                        .collect();

                    if !enabled_indices.is_empty() {
                        let current_enabled_pos = active_key.as_ref().and_then(|key| {
                            enabled_indices
                                .iter()
                                .position(|&ei| &self.panes[ei].key == key)
                        });

                        let mut target_enabled_pos = None;

                        ui.input(|i| {
                            if is_horizontal {
                                if i.key_pressed(egui::Key::ArrowLeft) {
                                    target_enabled_pos =
                                        current_enabled_pos.map(|p| p.saturating_sub(1));
                                } else if i.key_pressed(egui::Key::ArrowRight) {
                                    target_enabled_pos = current_enabled_pos
                                        .map(|p| (p + 1).min(enabled_indices.len() - 1));
                                }
                            } else {
                                if i.key_pressed(egui::Key::ArrowUp) {
                                    target_enabled_pos =
                                        current_enabled_pos.map(|p| p.saturating_sub(1));
                                } else if i.key_pressed(egui::Key::ArrowDown) {
                                    target_enabled_pos = current_enabled_pos
                                        .map(|p| (p + 1).min(enabled_indices.len() - 1));
                                }
                            }

                            if i.key_pressed(egui::Key::Home) {
                                target_enabled_pos = Some(0);
                            } else if i.key_pressed(egui::Key::End) {
                                target_enabled_pos = Some(enabled_indices.len() - 1);
                            }
                        });

                        if let Some(pos) = target_enabled_pos {
                            let new_idx = enabled_indices[pos];
                            let new_key = self.panes[new_idx].key.clone();
                            if Some(&new_key) != active_key.as_ref() {
                                active_key = Some(new_key);
                                scroll_to_active = true;
                                changed = true;
                            }
                        }
                    }
                }

                if is_horizontal {
                    ui.vertical(|ui| {
                        if self.tab_position == TabPosition::Top {
                            edit_action = self.render_horizontal_tab_bar(
                                ui,
                                id,
                                &mut active_key,
                                &mut changed,
                                &mut scroll_to_active,
                            );
                            ui.add_space(12.0);
                            self.render_pane_content(ui, id, &active_key, &mut add_contents);
                        } else {
                            self.render_pane_content(ui, id, &active_key, &mut add_contents);
                            ui.add_space(12.0);
                            edit_action = self.render_horizontal_tab_bar(
                                ui,
                                id,
                                &mut active_key,
                                &mut changed,
                                &mut scroll_to_active,
                            );
                        }
                    })
                    .response
                } else {
                    ui.horizontal(|ui| {
                        if self.tab_position == TabPosition::Start {
                            edit_action = self.render_vertical_tab_bar(
                                ui,
                                id,
                                &mut active_key,
                                &mut changed,
                                &mut scroll_to_active,
                            );
                            ui.add_space(12.0);
                            self.render_pane_content(ui, id, &active_key, &mut add_contents);
                        } else {
                            self.render_pane_content(ui, id, &active_key, &mut add_contents);
                            ui.add_space(12.0);
                            edit_action = self.render_vertical_tab_bar(
                                ui,
                                id,
                                &mut active_key,
                                &mut changed,
                                &mut scroll_to_active,
                            );
                        }
                    })
                    .response
                }
            })
            .inner;

        if active_key != current_active_key_before {
            if let Some(key) = &active_key {
                if let Some(user_key) = &mut self.active_key {
                    **user_key = key.clone();
                }
                ui.data_mut(|d| d.insert_temp(id.with("active_key"), key.clone()));
                if let Some(mut on_change) = self.on_change.take() {
                    on_change(key.clone());
                }
            }
            changed = true;
        }

        // Sync back if user's key was different from validated key
        if let Some(user_key) = &mut self.active_key
            && let Some(validated_key) = &active_key
            && *user_key != validated_key
        {
            **user_key = validated_key.clone();
            changed = true;
        }

        if let (Some(action), Some(mut on_edit)) = (edit_action.clone(), self.on_edit.take()) {
            on_edit(action);
            changed = true;
        }

        if changed {
            res.mark_changed();
        }

        TabsResponse {
            response: res,
            active_key: active_key.unwrap_or_default(),
            edit_action,
        }
    }
}

impl<'a> Tabs<'a> {
    /// Get the currently selected tab key for a given Tabs widget ID.
    pub fn selected(ui: &Ui, id_source: impl std::hash::Hash) -> Option<String> {
        let id = ui.make_persistent_id(id_source);
        ui.data(|d| d.get_temp::<String>(id.with("active_key")))
    }

    fn render_pane_content(
        &mut self,
        ui: &mut Ui,
        id: Id,
        active_key: &Option<String>,
        add_contents: &mut impl FnMut(&mut Ui, &str),
    ) {
        if let Some(key) = active_key {
            let fade_id = id.with("fade");
            let prev_key_id = id.with("prev_key");

            let prev_key = ui.data(|d| d.get_temp::<String>(prev_key_id));

            if prev_key.as_ref() != Some(key) {
                // Key changed, restart animation
                ui.data_mut(|d| {
                    d.insert_temp(prev_key_id, key.clone());
                    d.insert_temp(fade_id, false);
                });
            }

            let fade_value = ui
                .ctx()
                .animate_bool_with_time(fade_id, true, ANIMATION_DURATION);

            ui.scope(|ui| {
                ui.set_opacity(fade_value);
                ui.push_id(key, |ui| {
                    add_contents(ui, key);
                });
            });
        }
    }

    fn get_tab_padding(&self) -> Vec2 {
        match (self.tab_type, self.size) {
            (TabType::Line, TabSize::Small) => Vec2::new(12.0, 8.0),
            (TabType::Line, TabSize::Medium) => Vec2::new(16.0, 12.0),
            (TabType::Line, TabSize::Large) => Vec2::new(24.0, 16.0),
            (_, TabSize::Small) => Vec2::new(12.0, 8.0),
            (_, TabSize::Medium) => Vec2::new(16.0, 12.0),
            (_, TabSize::Large) => Vec2::new(24.0, 16.0),
        }
    }

    fn get_font_size(&self) -> f32 {
        match self.size {
            TabSize::Small | TabSize::Medium => 14.0,
            TabSize::Large => 16.0,
        }
    }

    fn get_tab_styles(
        &self,
        ui: &Ui,
        pane: &TabPane,
        _parent_id: egui::Id,
        is_active: bool,
        is_hovered: bool,
    ) -> (Color32, egui::Margin, egui::CornerRadius) {
        let is_card = self.tab_type == TabType::Card || self.tab_type == TabType::EditableCard;
        let text_color = if pane.disabled {
            // If the tab is disabled, we let add_enabled_ui handle the visual dimming.
            // We use the regular text color here so it doesn't get double-dimmed.
            ui.visuals().text_color()
        } else if is_active {
            ANTD_BLUE
        } else if is_hovered {
            Color32::from_rgb(64, 150, 255)
        } else {
            // Inactive tabs (both Line and Card) use a grayed-out color
            ui.visuals().text_color().gamma_multiply(0.88)
        };

        let padding = self.get_tab_padding();
        let mut margin = egui::Margin {
            left: padding.x as i8,
            right: padding.x as i8,
            top: padding.y as i8,
            bottom: padding.y as i8,
        };

        if is_card && is_active {
            match self.tab_position {
                TabPosition::Top => margin.bottom += 1,
                TabPosition::Bottom => margin.top += 1,
                TabPosition::Start => margin.right += 1,
                TabPosition::End => margin.left += 1,
            }
        }

        let corner_radius = if is_card {
            match self.tab_position {
                TabPosition::Top => egui::CornerRadius {
                    nw: 6,
                    ne: 6,
                    sw: 0,
                    se: 0,
                },
                TabPosition::Bottom => egui::CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 6,
                    se: 6,
                },
                TabPosition::Start => egui::CornerRadius {
                    nw: 6,
                    ne: 0,
                    sw: 6,
                    se: 0,
                },
                TabPosition::End => egui::CornerRadius {
                    nw: 0,
                    ne: 6,
                    sw: 0,
                    se: 6,
                },
            }
        } else {
            egui::CornerRadius::ZERO
        };

        (text_color, margin, corner_radius)
    }
}

impl<'a> Tabs<'a> {
    fn render_horizontal_tab_bar(
        &mut self,
        ui: &mut Ui,
        parent_id: Id,
        active_key: &mut Option<String>,
        changed: &mut bool,
        scroll_to_active: &mut bool,
    ) -> Option<TabEditAction> {
        let tabs_id = ui.id().with("horiz_tabs");
        let overflow_id = tabs_id.with("overflow");
        let scroll_command_id = tabs_id.with("scroll_command");

        let is_overflowing = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(overflow_id).unwrap_or(false));
        let mut scroll_command = ui
            .ctx()
            .data(|d| d.get_temp::<Option<f32>>(scroll_command_id).flatten());

        let mut edit_action = None;

        let full_width = ui.available_width();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let mut left_width = 0.0;
            if let Some(left) = self.extra.left.take() {
                let res = ui.scope(|ui| left(ui)).response;
                left_width = res.rect.width();
                ui.add_space(8.0);
                left_width += 8.0;
            }

            if is_overflowing {
                // We use a dummy ScrollArea state to check if we can scroll left
                let id = tabs_id.with("scroll");
                let can_scroll_left =
                    if let Some(state) = egui::scroll_area::State::load(ui.ctx(), id) {
                        state.offset.x > 1.0
                    } else {
                        false
                    };

                if can_scroll_left {
                    if ui
                        .add_sized(
                            [ARROW_WIDTH, ui.available_height()],
                            egui::Button::new("<").frame(false),
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        let id = tabs_id.with("scroll");
                        let current_offset = egui::scroll_area::State::load(ui.ctx(), id)
                            .map(|s| s.offset.x)
                            .unwrap_or(0.0);
                        scroll_command = Some(current_offset - SCROLL_INCREMENT);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(scroll_command_id, scroll_command));
                    }
                } else {
                    // Reserve space
                    ui.add_space(ARROW_WIDTH);
                }
            }

            let mut scroll_area = egui::scroll_area::ScrollArea::horizontal()
                .id_salt(tabs_id.with("scroll"))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden);

            if let Some(target) = scroll_command {
                scroll_area = scroll_area.horizontal_scroll_offset(target);
                // Clear the command after applying it
                ui.ctx()
                    .data_mut(|d| d.insert_temp(scroll_command_id, Option::<f32>::None));
            }

            let right_content = self.extra.right.take();
            let scroll_res = ui
                .with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let mut right_width = 0.0;
                    if let Some(right) = right_content {
                        let res = ui.scope(|ui| right(ui)).response;
                        right_width = res.rect.width();
                        ui.add_space(8.0);
                        right_width += 8.0;
                    }

                    if is_overflowing {
                        let current_max = ui.ctx().data(|d| {
                            d.get_temp::<f32>(tabs_id.with("current_max"))
                                .unwrap_or(0.0)
                        });
                        let current_offset = ui.ctx().data(|d| {
                            d.get_temp::<f32>(tabs_id.with("current_offset"))
                                .unwrap_or(0.0)
                        });
                        let can_scroll_right = current_offset < current_max - 1.0;

                        if can_scroll_right {
                            if ui
                                .add_sized(
                                    [ARROW_WIDTH, ui.available_height()],
                                    egui::Button::new(">").frame(false),
                                )
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                let cmd = Some(current_offset + SCROLL_INCREMENT);
                                ui.ctx().data_mut(|d| d.insert_temp(scroll_command_id, cmd));
                            }
                        } else {
                            ui.add_space(ARROW_WIDTH);
                        }
                    }

                    if self.centered {
                        let side_margin = left_width.max(right_width);
                        if side_margin * 2.0 < full_width {
                            let right_spacer = side_margin - right_width;
                            if right_spacer > 0.0 {
                                ui.add_space(right_spacer);
                            }
                        }
                    }

                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if self.centered {
                            let side_margin = left_width.max(right_width);
                            if side_margin * 2.0 < full_width {
                                let left_spacer = side_margin - left_width;
                                if left_spacer > 0.0 {
                                    ui.add_space(left_spacer);
                                }
                            }
                        }

                        scroll_area.max_width(ui.available_width()).show(ui, |ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            if self.centered {
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        self.render_tabs(
                                            ui,
                                            parent_id,
                                            active_key,
                                            changed,
                                            scroll_to_active,
                                        )
                                    })
                                    .inner
                                })
                                .inner
                            } else {
                                self.render_tabs(
                                    ui,
                                    parent_id,
                                    active_key,
                                    changed,
                                    scroll_to_active,
                                )
                            }
                        })
                    })
                    .inner
                })
                .inner;

            edit_action = scroll_res.inner;

            let current_max = (scroll_res.content_size.x - scroll_res.inner_rect.width()).max(0.0);
            let current_offset = scroll_res.state.offset.x;

            ui.ctx().data_mut(|d| {
                d.insert_temp(overflow_id, current_max > 0.0);
                d.insert_temp(tabs_id.with("current_max"), current_max);
                d.insert_temp(tabs_id.with("current_offset"), current_offset);
            });
        });

        let stroke = egui::Stroke::new(1.0, ANTD_BORDER_COLOR);
        let (rect, y) = if self.tab_position == TabPosition::Top {
            (ui.clip_rect(), ui.min_rect().max.y)
        } else {
            (ui.clip_rect(), ui.min_rect().min.y)
        };

        // Paint background line on a separate layer to avoid overlapping with active tab content
        ui.ctx()
            .layer_painter(LayerId::new(Order::Background, ui.id()))
            .line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                stroke,
            );

        edit_action
    }

    fn render_vertical_tab_bar(
        &mut self,
        ui: &mut Ui,
        parent_id: Id,
        active_key: &mut Option<String>,
        changed: &mut bool,
        scroll_to_active: &mut bool,
    ) -> Option<TabEditAction> {
        let tabs_id = ui.id().with("vertical_tabs");
        let overflow_id = tabs_id.with("overflow");
        let scroll_command_id = tabs_id.with("scroll_command");

        let is_overflowing = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(overflow_id).unwrap_or(false));
        let mut scroll_command = ui
            .ctx()
            .data(|d| d.get_temp::<Option<f32>>(scroll_command_id).flatten());

        let mut edit_action = None;
        let full_height = ui.available_height();
        let res = ui.vertical(|ui| {
            let mut top_height = 0.0;
            if let Some(left) = self.extra.left.take() {
                let res = ui.scope(|ui| left(ui)).response;
                top_height = res.rect.height();
                ui.add_space(8.0);
                top_height += 8.0;
            }

            if is_overflowing {
                let id = tabs_id.with("scroll");
                let can_scroll_up =
                    if let Some(state) = egui::scroll_area::State::load(ui.ctx(), id) {
                        state.offset.y > 1.0
                    } else {
                        false
                    };

                if can_scroll_up {
                    if ui
                        .add_sized(
                            [ui.available_width(), ARROW_WIDTH],
                            egui::Button::new("^").frame(false),
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        let id = tabs_id.with("scroll");
                        let current_offset = egui::scroll_area::State::load(ui.ctx(), id)
                            .map(|s| s.offset.y)
                            .unwrap_or(0.0);
                        scroll_command = Some(current_offset - SCROLL_INCREMENT);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(scroll_command_id, scroll_command));
                    }
                } else {
                    // Reserve space
                    ui.add_space(ARROW_WIDTH);
                }
            }

            let mut scroll_area = egui::scroll_area::ScrollArea::vertical()
                .id_salt(tabs_id.with("scroll"))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden);

            if let Some(target) = scroll_command {
                scroll_area = scroll_area.vertical_scroll_offset(target);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(scroll_command_id, Option::<f32>::None));
            }

            let right_content = self.extra.right.take();
            let scroll_res = ui
                .with_layout(Layout::bottom_up(Align::Center), |ui| {
                    let mut bottom_height = 0.0;
                    if let Some(right) = right_content {
                        let res = ui.scope(|ui| right(ui)).response;
                        bottom_height = res.rect.height();
                    }

                    if is_overflowing {
                        let current_max = ui.ctx().data(|d| {
                            d.get_temp::<f32>(tabs_id.with("current_max_v"))
                                .unwrap_or(0.0)
                        });
                        let current_offset = ui.ctx().data(|d| {
                            d.get_temp::<f32>(tabs_id.with("current_offset_v"))
                                .unwrap_or(0.0)
                        });
                        let can_scroll_down = current_offset < current_max - 1.0;

                        if can_scroll_down {
                            if ui
                                .add_sized(
                                    [ui.available_width(), ARROW_WIDTH],
                                    egui::Button::new("v").frame(false),
                                )
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                let cmd = Some(current_offset + SCROLL_INCREMENT);
                                ui.ctx().data_mut(|d| d.insert_temp(scroll_command_id, cmd));
                            }
                        } else {
                            ui.add_space(ARROW_WIDTH);
                        }
                    }

                    if self.centered {
                        let side_margin = top_height.max(bottom_height);
                        if side_margin * 2.0 < full_height {
                            let bottom_spacer = side_margin - bottom_height;
                            if bottom_spacer > 0.0 {
                                ui.add_space(bottom_spacer);
                            }
                        }
                    }

                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        if self.centered {
                            let side_margin = top_height.max(bottom_height);
                            if side_margin * 2.0 < full_height {
                                let top_spacer = side_margin - top_height;
                                if top_spacer > 0.0 {
                                    ui.add_space(top_spacer);
                                }
                            }
                        }

                        scroll_area
                            .max_height(ui.available_height())
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                if self.centered {
                                    ui.vertical_centered(|ui| {
                                        self.render_tabs(
                                            ui,
                                            parent_id,
                                            active_key,
                                            changed,
                                            scroll_to_active,
                                        )
                                    })
                                    .inner
                                } else {
                                    self.render_tabs(
                                        ui,
                                        parent_id,
                                        active_key,
                                        changed,
                                        scroll_to_active,
                                    )
                                }
                            })
                    })
                    .inner
                })
                .inner;

            edit_action = scroll_res.inner;

            let current_max = (scroll_res.content_size.y - scroll_res.inner_rect.height()).max(0.0);
            let current_offset = scroll_res.state.offset.y;

            ui.ctx().data_mut(|d| {
                d.insert_temp(overflow_id, current_max > 0.0);
                d.insert_temp(tabs_id.with("current_max_v"), current_max);
                d.insert_temp(tabs_id.with("current_offset_v"), current_offset);
            });
        });

        let tabs_rect = res.response.rect;
        let container_rect = ui.clip_rect();
        let stroke = egui::Stroke::new(1.0, ANTD_BORDER_COLOR);
        let x = if self.tab_position == TabPosition::Start {
            tabs_rect.max.x
        } else {
            tabs_rect.min.x
        };

        // Paint background line on a separate layer to avoid overlapping with active tab content
        ui.ctx()
            .layer_painter(LayerId::new(Order::Background, ui.id()))
            .line_segment(
                [
                    egui::pos2(x, container_rect.min.y),
                    egui::pos2(x, container_rect.max.y),
                ],
                stroke,
            );

        edit_action
    }

    fn render_tabs(
        &mut self,
        ui: &mut Ui,
        parent_id: Id,
        active_key: &mut Option<String>,
        changed: &mut bool,
        scroll_to_active: &mut bool,
    ) -> Option<TabEditAction> {
        let has_focus = ui.memory(|m| m.has_focus(parent_id));
        self.render_tab_items(
            ui,
            parent_id,
            active_key,
            changed,
            scroll_to_active,
            has_focus,
        )
    }

    fn render_tab_items(
        &mut self,
        ui: &mut Ui,
        parent_id: Id,
        active_key: &mut Option<String>,
        changed: &mut bool,
        scroll_to_active: &mut bool,
        has_focus: bool,
    ) -> Option<TabEditAction> {
        let mut edit_action = None;

        for (i, pane) in self.panes.iter().enumerate() {
            let is_active = active_key.as_ref() == Some(&pane.key);
            let tab_id = parent_id.with("tab").with(&pane.key);

            if is_active && *scroll_to_active {
                ui.scroll_to_cursor(Some(Align::Center));
                *scroll_to_active = false;
            }

            // Get interaction state from previous frame for smooth hover and selection
            let last_rect = ui.data(|d| {
                d.get_temp::<egui::Rect>(tab_id)
                    .unwrap_or(egui::Rect::NOTHING)
            });
            let close_id = tab_id.with("close");
            let is_close_hovered = ui.data(|d| d.get_temp::<bool>(close_id).unwrap_or(false));

            let interact_res = ui.interact(last_rect, tab_id, Sense::hover());
            let is_hovered = interact_res.hovered() && !is_close_hovered;
            let (text_color, margin, corner_radius) =
                self.get_tab_styles(ui, pane, parent_id, is_active, is_hovered);

            ui.push_id(&pane.key, |ui| {
                let mut close_clicked = false;
                let (tab_rect, label_rect) = ui
                    .add_enabled_ui(!pane.disabled, |ui| {
                        let is_card = self.tab_type == TabType::Card
                            || self.tab_type == TabType::EditableCard;
                        let mut bg_color = Color32::TRANSPARENT;
                        let mut stroke = egui::Stroke::NONE;

                        if is_card {
                            stroke = egui::Stroke::new(1.0, ANTD_BORDER_COLOR);
                            if is_active {
                                bg_color = ui.visuals().window_fill();
                            } else {
                                bg_color = if ui.visuals().dark_mode {
                                    ui.visuals().faint_bg_color
                                } else {
                                    ANTD_COLOR_FILL_ALTER
                                };
                            }
                        }

                        let mut inner_label_rect = egui::Rect::NOTHING;
                        let frame_res = egui::Frame::NONE
                            .fill(bg_color)
                            .stroke(stroke)
                            .inner_margin(margin)
                            .corner_radius(corner_radius)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.visuals_mut().override_text_color = Some(text_color);
                                    ui.scope(|ui| {
                                        ui.style_mut().interaction.selectable_labels = false;
                                        let font_id = if is_active {
                                            let mut id = ui
                                                .style()
                                                .text_styles
                                                .get(&egui::TextStyle::Heading)
                                                .cloned()
                                                .unwrap_or_else(|| {
                                                    egui::FontId::proportional(self.get_font_size())
                                                });
                                            id.size = self.get_font_size();
                                            id
                                        } else {
                                            egui::FontId::proportional(self.get_font_size())
                                        };
                                        ui.style_mut().override_font_id = Some(font_id);
                                        let res = (pane.label)(ui);
                                        inner_label_rect = res.rect;
                                    });
                                    ui.visuals_mut().override_text_color = None;

                                    if (self.tab_type == TabType::EditableCard) && pane.closable {
                                        ui.add_space(8.0);
                                        let is_close_hovered = ui.data(|d| {
                                            d.get_temp::<bool>(close_id).unwrap_or(false)
                                        });

                                        let color = if is_active {
                                            ui.visuals().text_color().gamma_multiply(0.88)
                                        } else {
                                            ui.visuals().text_color().gamma_multiply(0.45)
                                        };
                                        let color =
                                            if is_close_hovered { ANTD_BLUE } else { color };

                                        let close_btn = egui::Button::new(
                                            egui::RichText::new(egui_phosphor::regular::X)
                                                .size(12.0)
                                                .color(color),
                                        )
                                        .frame(false);

                                        let close_resp = ui
                                            .add(close_btn)
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);

                                        // Save hover state for the next frame
                                        ui.data_mut(|d| {
                                            d.insert_temp(close_id, close_resp.hovered())
                                        });

                                        if close_resp.clicked() {
                                            close_clicked = true;
                                            edit_action =
                                                Some(TabEditAction::Remove(pane.key.clone()));
                                        }
                                    }
                                })
                                .inner
                            });

                        let tab_rect = frame_res.response.rect;
                        ui.data_mut(|d| d.insert_temp(tab_id, tab_rect));

                        (tab_rect, inner_label_rect)
                    })
                    .inner;

                // Check for tab click after rendering contents to ensure correct overlap handling
                let interact_res = ui.interact(tab_rect, tab_id, Sense::click());
                if interact_res.clicked() && !close_clicked && !pane.disabled {
                    ui.memory_mut(|m| m.request_focus(parent_id));
                    *active_key = Some(pane.key.clone());
                    *changed = true;
                }

                if interact_res.hovered() && !is_close_hovered && !pane.disabled {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                } else if interact_res.hovered() && pane.disabled {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::NotAllowed);
                }

                if is_active {
                    if has_focus {
                        // Focus ring
                        let focus_rect = tab_rect.expand(1.0);
                        ui.painter().rect_stroke(
                            focus_rect,
                            corner_radius,
                            egui::Stroke::new(2.0, ANTD_BLUE.gamma_multiply(0.3)),
                            egui::StrokeKind::Outside,
                        );
                    }

                    if self.tab_type == TabType::Line {
                        let ink_id = parent_id.with("ink_bar");

                        let (target_min, target_max) = if self.tab_position == TabPosition::Start
                            || self.tab_position == TabPosition::End
                        {
                            (label_rect.min.y, label_rect.max.y)
                        } else {
                            (label_rect.min.x, label_rect.max.x)
                        };

                        let anim_min = ui.ctx().animate_value_with_time(
                            ink_id.with("min"),
                            target_min,
                            ANIMATION_DURATION,
                        );
                        let anim_max = ui.ctx().animate_value_with_time(
                            ink_id.with("max"),
                            target_max,
                            ANIMATION_DURATION,
                        );

                        let line_rect = if self.tab_position == TabPosition::Start
                            || self.tab_position == TabPosition::End
                        {
                            let x = if self.tab_position == TabPosition::Start {
                                tab_rect.max.x
                            } else {
                                tab_rect.min.x
                            };
                            let x_offset = if self.tab_position == TabPosition::Start {
                                -2.0
                            } else {
                                2.0
                            };
                            egui::Rect::from_min_max(
                                egui::pos2(x + x_offset - 1.0, anim_min),
                                egui::pos2(x + x_offset + 1.0, anim_max),
                            )
                        } else {
                            let y = if self.tab_position == TabPosition::Top {
                                tab_rect.max.y
                            } else {
                                tab_rect.min.y
                            };
                            let y_offset = if self.tab_position == TabPosition::Top {
                                -2.0
                            } else {
                                2.0
                            };
                            egui::Rect::from_min_max(
                                egui::pos2(anim_min, y + y_offset - 1.0),
                                egui::pos2(anim_max, y + y_offset + 1.0),
                            )
                        };
                        ui.painter().rect_filled(line_rect, 2.0, ANTD_BLUE);
                    } else if self.tab_type == TabType::Card
                        || self.tab_type == TabType::EditableCard
                    {
                        let bg_color = ui.visuals().window_fill();
                        match self.tab_position {
                            TabPosition::Top => {
                                let y = tab_rect.max.y;
                                let line_rect = egui::Rect::from_min_max(
                                    egui::pos2(tab_rect.min.x + 1.0, y - 1.0),
                                    egui::pos2(tab_rect.max.x - 1.0, y + 3.0),
                                );
                                ui.painter().rect_filled(line_rect, 0.0, bg_color);
                            }
                            TabPosition::Bottom => {
                                let y = tab_rect.min.y;
                                let line_rect = egui::Rect::from_min_max(
                                    egui::pos2(tab_rect.min.x + 1.0, y - 3.0),
                                    egui::pos2(tab_rect.max.x - 1.0, y + 1.0),
                                );
                                ui.painter().rect_filled(line_rect, 0.0, bg_color);
                            }
                            TabPosition::Start => {
                                let x = tab_rect.max.x;
                                let line_rect = egui::Rect::from_min_max(
                                    egui::pos2(x - 1.0, tab_rect.min.y + 1.0),
                                    egui::pos2(x + 3.0, tab_rect.max.y - 1.0),
                                );
                                ui.painter().rect_filled(line_rect, 0.0, bg_color);
                            }
                            TabPosition::End => {
                                let x = tab_rect.min.x;
                                let line_rect = egui::Rect::from_min_max(
                                    egui::pos2(x - 3.0, tab_rect.min.y + 1.0),
                                    egui::pos2(x + 1.0, tab_rect.max.y - 1.0),
                                );
                                ui.painter().rect_filled(line_rect, 0.0, bg_color);
                            }
                        }
                    }
                }
            });

            if i < self.panes.len() - 1 {
                let spacing = if self.tab_type == TabType::Line {
                    self.gutter.unwrap_or(DEFAULT_GUTTER)
                } else {
                    self.gutter.unwrap_or(0.0)
                };
                ui.add_space(spacing);
            }
        }

        if self.tab_type == TabType::EditableCard && !self.hide_add {
            let add_btn = Button::new("")
                .button_type(ButtonType::Text)
                .icon(egui::RichText::new(egui_phosphor::regular::PLUS).size(14.0));
            if ui
                .add(add_btn)
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                edit_action = Some(TabEditAction::Add);
            }
        }

        edit_action
    }
}
