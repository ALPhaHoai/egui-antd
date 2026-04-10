use egui::{Response, Ui, Widget, WidgetInfo, WidgetType, CornerRadius};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonType {
    #[default]
    Default,
    Primary,
    Dashed,
    Link,
    Text,
    Gradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Large,
    #[default]
    Middle,
    Small,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonShape {
    #[default]
    Default,
    Circle,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonPosition {
    #[default]
    None,
    First,
    Middle,
    Last,
    // For 2D compact groups (e.g. within Space)
    TopFirst,
    TopMiddle,
    TopLast,
    BottomFirst,
    BottomMiddle,
    BottomLast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconPlacement {
    #[default]
    Start,
    End,
}

pub struct Button<'a> {
    text: String,
    button_type: ButtonType,
    size: ButtonSize,
    shape: ButtonShape,
    danger: bool,
    disabled: bool,
    loading: bool,
    loading_icon: Option<egui::Image<'a>>,
    block: bool,
    ghost: bool,
    icon: Option<egui::WidgetText>,
    image: Option<egui::Image<'a>>,
    position: ButtonPosition,
    icon_placement: IconPlacement,
    href: Option<String>,
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            button_type: ButtonType::Default,
            size: ButtonSize::Middle,
            shape: ButtonShape::Default,
            danger: false,
            disabled: false,
            loading: false,
            loading_icon: None,
            block: false,
            ghost: false,
            icon: None,
            image: None,
            position: ButtonPosition::None,
            icon_placement: IconPlacement::Start,
            href: None,
        }
    }

    pub fn icon_placement(mut self, placement: IconPlacement) -> Self {
        self.icon_placement = placement;
        self
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn set_position(mut self, position: ButtonPosition) -> Self {
        self.position = position;
        self
    }

    pub fn icon(mut self, icon: impl Into<egui::WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn image(mut self, image: egui::Image<'a>) -> Self {
        self.image = Some(image);
        self
    }

    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn loading_icon(mut self, icon: egui::Image<'a>) -> Self {
        self.loading_icon = Some(icon);
        self
    }

    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    pub fn ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Button {
            text,
            button_type,
            size,
            shape,
            danger,
            disabled,
            loading,
            loading_icon,
            block,
            ghost,
            icon,
            image,
            position,
            icon_placement,
            href,
        } = self;

        // Ant Design 5.0 Tokens
        let color_primary = egui::Color32::from_rgb(22, 119, 255);
        let color_primary_hover = egui::Color32::from_rgb(64, 150, 255);
        let color_primary_active = egui::Color32::from_rgb(9, 88, 217);

        let color_error = egui::Color32::from_rgb(255, 77, 79);
        let color_error_hover = egui::Color32::from_rgb(255, 120, 117);
        let color_error_active = egui::Color32::from_rgb(217, 54, 53);

        let color_border = egui::Color32::from_rgb(217, 217, 217);
        let color_text = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 224); // 0.88 opacity
        let color_text_disabled = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 64); // 0.25 opacity

        let theme = ui.ctx().data(|d| d.get_temp::<crate::Theme>(egui::Id::new("antd_config_provider_theme")).unwrap_or_default());
        let default_color_bg_container_disabled = egui::Color32::from_rgb(245, 245, 245);
        let color_bg_container_disabled_default = theme.components.button.default_bg_disabled.unwrap_or(default_color_bg_container_disabled);
        let color_bg_container_disabled_dashed = theme.components.button.dashed_bg_disabled.unwrap_or(default_color_bg_container_disabled);

        let button_padding = match size {
            ButtonSize::Large => egui::vec2(15.0, 7.0),
            ButtonSize::Middle => egui::vec2(15.0, 4.0),
            ButtonSize::Small => egui::vec2(7.0, 0.0),
        };

        let text_size = match size {
            ButtonSize::Large => 16.0,
            ButtonSize::Middle => 14.0,
            ButtonSize::Small => 12.0,
        };

        let font_id = egui::FontId::proportional(text_size);
        let _wrap_width = ui.available_width();

        // Detect two Chinese characters for special spacing
        let mut display_text = text.clone();
        if text.chars().count() == 2 && text.chars().all(|c| (0x4E00..=0x9FFF).contains(&(c as u32))) {
            let mut chars = text.chars();
            display_text = format!("{}  {}", chars.next().unwrap(), chars.next().unwrap());
        }

        let mut text_color = color_text;
        if disabled {
            text_color = color_text_disabled;
        }

        let galley = ui.painter().layout_no_wrap(display_text, font_id.clone(), text_color);

        let icon_size = egui::vec2(text_size, text_size);
        let icon_gap = if text.is_empty() { 0.0 } else { 8.0 };

        let has_icon = icon.is_some() || image.is_some() || loading;
        let mut desired_size = galley.size() + 2.0 * button_padding;

        if block {
            desired_size.x = ui.available_width();
        }

        if has_icon {
            desired_size.x += icon_size.x + icon_gap;
        }

        if shape == ButtonShape::Circle {
            let side = desired_size.x.max(desired_size.y);
            desired_size = egui::vec2(side, side);
        }

        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        if disabled || loading {
            response = response.on_hover_cursor(egui::CursorIcon::NotAllowed);
        } else {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        if let Some(href) = &href {
            if response.clicked() && !disabled && !loading {
                ui.ctx().open_url(egui::OpenUrl::new_tab(href));
            }
        }

        if ui.is_rect_visible(rect) {
            let is_hover = response.hovered();
            let is_active = response.clicked() || response.dragged();

            // Transitions
            let hover_t = ui.ctx().animate_bool_with_time(response.id.with("hover"), is_hover, 0.1);
            let _active_t = ui.ctx().animate_bool_with_time(response.id.with("active"), is_active, 0.1);

            // Wave effect transition
            let wave_id = response.id.with("wave");
            let now = ui.input(|i| i.time);
            if response.clicked() && !disabled {
                ui.ctx().data_mut(|d| d.insert_temp(wave_id, now));
            }
            let last_click_time: Option<f64> = ui.ctx().data(|d| d.get_temp(wave_id));
            let wave_t = if let Some(t) = last_click_time {
                let elapsed = now - t;
                let duration = 0.4;
                if elapsed < duration {
                    ui.ctx().request_repaint();
                    (elapsed / duration) as f32
                } else {
                    0.0
                }
            } else {
                0.0
            };

            let (bg_fill, stroke, new_text_color) = if disabled {
                let bg = match button_type {
                    ButtonType::Default => color_bg_container_disabled_default,
                    ButtonType::Dashed => color_bg_container_disabled_dashed,
                    ButtonType::Primary | ButtonType::Text | ButtonType::Link | ButtonType::Gradient => default_color_bg_container_disabled,
                };
                (
                    bg,
                    egui::Stroke::new(1.0, color_border),
                    color_text_disabled,
                )
            } else {
                match button_type {
                    ButtonType::Primary => {
                        let base = if danger { color_error } else { color_primary };
                        let hover = if danger { color_error_hover } else { color_primary_hover };
                        let active = if danger { color_error_active } else { color_primary_active };

                        if ghost {
                            let current_color = if is_active { active } else { lerp_color(base, hover, hover_t) };
                            (egui::Color32::TRANSPARENT, egui::Stroke::new(1.0, current_color), current_color)
                        } else {
                            let fill = if is_active { active } else { lerp_color(base, hover, hover_t) };
                            (fill, egui::Stroke::NONE, egui::Color32::WHITE)
                        }
                    }
                    ButtonType::Default | ButtonType::Dashed => {
                        let text_base = if danger { color_error } else if ghost { egui::Color32::WHITE } else { color_text };
                        let text_hover = if danger { color_error_hover } else { color_primary_hover };
                        let text_active = if danger { color_error_active } else { color_primary_active };

                        let current_text = if is_active { text_active } else { lerp_color(text_base, text_hover, hover_t) };

                        let stroke_base = if danger { color_error } else if ghost { egui::Color32::WHITE } else { color_border };
                        let stroke_hover = if danger { color_error_hover } else { color_primary_hover };
                        let stroke_active = if danger { color_error_active } else { color_primary_active };

                        let current_stroke = if is_active { stroke_active } else { lerp_color(stroke_base, stroke_hover, hover_t) };

                        let bg = if ghost {
                            egui::Color32::TRANSPARENT
                        } else {
                            egui::Color32::WHITE
                        };

                        (bg, egui::Stroke::new(1.0, current_stroke), current_text)
                    }
                    ButtonType::Link => {
                        let base = if danger { color_error } else { color_primary };
                        let hover = if danger { color_error_hover } else { color_primary_hover };
                        let active = if danger { color_error_active } else { color_primary_active };

                        let current_text = if is_active { active } else { lerp_color(base, hover, hover_t) };
                        (egui::Color32::TRANSPARENT, egui::Stroke::NONE, current_text)
                    }
                    ButtonType::Text => {
                        let text_base = if danger { color_error } else { color_text };
                        let text_hover = if danger { color_error_hover } else { color_text };
                        let text_active = if danger { color_error_active } else { color_text };

                        let current_text = if is_active { text_active } else { lerp_color(text_base, text_hover, hover_t) };

                        let bg_alpha = if is_active { 15 } else { (hover_t * 10.0) as u8 };
                        let bg = egui::Color32::from_rgba_unmultiplied(0, 0, 0, bg_alpha);
                        (bg, egui::Stroke::NONE, current_text)
                    }
                    ButtonType::Gradient => {
                        (color_primary, egui::Stroke::NONE, egui::Color32::WHITE)
                    }
                }
            };
            text_color = new_text_color;

            let corner_radius = match shape {
                ButtonShape::Circle => CornerRadius::same(u8::MAX),
                ButtonShape::Round => CornerRadius::same(u8::MAX),
                ButtonShape::Default => match position {
                    ButtonPosition::None => CornerRadius::same(6),
                    ButtonPosition::First => CornerRadius {
                        nw: 6,
                        sw: 6,
                        ne: 0,
                        se: 0,
                    },
                    ButtonPosition::Middle => CornerRadius::ZERO,
                    ButtonPosition::Last => CornerRadius {
                        nw: 0,
                        sw: 0,
                        ne: 6,
                        se: 6,
                    },
                    ButtonPosition::TopFirst => CornerRadius {
                        nw: 6,
                        ne: 0,
                        sw: 0,
                        se: 0,
                    },
                    ButtonPosition::TopMiddle => CornerRadius::ZERO,
                    ButtonPosition::TopLast => CornerRadius {
                        nw: 0,
                        ne: 6,
                        sw: 0,
                        se: 0,
                    },
                    ButtonPosition::BottomFirst => CornerRadius {
                        nw: 0,
                        ne: 0,
                        sw: 6,
                        se: 0,
                    },
                    ButtonPosition::BottomMiddle => CornerRadius::ZERO,
                    ButtonPosition::BottomLast => CornerRadius {
                        nw: 0,
                        ne: 0,
                        sw: 0,
                        se: 6,
                    },
                },
            };

            if button_type == ButtonType::Gradient && !disabled {
                let color1 = if is_active {
                    egui::Color32::from_rgb(105, 54, 245)
                } else if is_hover {
                    egui::Color32::from_rgb(149, 117, 255)
                } else {
                    egui::Color32::from_rgb(105, 54, 245)
                };
                let color2 = if is_active {
                    egui::Color32::from_rgb(22, 119, 255)
                } else if is_hover {
                    egui::Color32::from_rgb(64, 150, 255)
                } else {
                    egui::Color32::from_rgb(22, 119, 255)
                };

                // Use a layer to clip the mesh to the rounded rect
                let painter = ui.painter().with_clip_rect(rect);
                // We draw a rounded rect with the background color first to handle edges
                painter.rect_filled(rect, corner_radius, color1);

                let mesh = create_gradient_mesh(rect, corner_radius, color1, color2);
                painter.add(mesh);
            } else {
                ui.painter().rect(
                    rect,
                    corner_radius,
                    bg_fill,
                    if button_type == ButtonType::Dashed && !disabled {
                        egui::Stroke::NONE
                    } else {
                        stroke
                    },
                    egui::StrokeKind::Inside,
                );
            }

            // Custom dashed border painting
            if button_type == ButtonType::Dashed && !disabled {
                let dash_length = 4.0;
                let gap_length = 4.0;
                paint_dashed_rect(
                    ui.painter(),
                    rect,
                    corner_radius,
                    stroke,
                    dash_length,
                    gap_length,
                );
            }

            // Draw Wave effect
            if wave_t > 0.0 && wave_t < 1.0 {
                let wave_color = if danger { color_error } else { color_primary };
                let alpha = (1.0 - wave_t) * 0.4;
                let wave_stroke = egui::Stroke::new(wave_t * 6.0, wave_color.gamma_multiply(alpha));
                let wave_rect = rect.expand(wave_t * 6.0);

                // Increase corner radius for the expanded rect to keep it looking natural
                let mut wave_radius = corner_radius;
                let expansion = wave_t * 6.0;
                wave_radius.nw = (wave_radius.nw as f32 + expansion).min(255.0) as u8;
                wave_radius.ne = (wave_radius.ne as f32 + expansion).min(255.0) as u8;
                wave_radius.sw = (wave_radius.sw as f32 + expansion).min(255.0) as u8;
                wave_radius.se = (wave_radius.se as f32 + expansion).min(255.0) as u8;

                ui.painter().rect_stroke(wave_rect, wave_radius, wave_stroke, egui::StrokeKind::Outside);
            }

            let mut content_width = galley.size().x;
            if has_icon {
                content_width += icon_size.x + icon_gap;
            }

            let mut text_pos = rect.center() - egui::vec2(content_width / 2.0, galley.size().y / 2.0);
            let galley_height = galley.size().y;

            let paint_icon = |ui: &mut Ui, pos: egui::Pos2| {
                if loading {
                    let spinner_rect = egui::Rect::from_center_size(
                        pos + egui::vec2(icon_size.x / 2.0, galley_height / 2.0),
                        icon_size,
                    );

                    if let Some(loading_icon) = &loading_icon {
                        loading_icon.clone().tint(text_color).paint_at(ui, spinner_rect);
                    } else {
                        // Draw a simple spinner
                        let angle = ui.input(|i| i.time as f32 * 6.0);
                        let center = spinner_rect.center();
                        let radius = icon_size.x / 2.0 - 2.0;
                        let start_angle = angle;
                        let end_angle = angle + std::f32::consts::TAU * 0.75;

                        let points: Vec<egui::Pos2> = (0..=20)
                            .map(|i| {
                                let t = i as f32 / 20.0;
                                let a = start_angle + (end_angle - start_angle) * t;
                                center + egui::vec2(a.cos(), a.sin()) * radius
                            })
                            .collect();

                        ui.painter()
                            .add(egui::Shape::line(points, egui::Stroke::new(2.0, text_color)));
                    }
                } else if let Some(image) = &image {
                    let icon_rect = egui::Rect::from_center_size(
                        pos + egui::vec2(icon_size.x / 2.0, galley_height / 2.0),
                        icon_size,
                    );
                    image.clone().tint(text_color).paint_at(ui, icon_rect);
                } else if let Some(icon) = &icon {
                     let icon_galley = ui.painter().layout_no_wrap(icon.text().to_string(), font_id.clone(), text_color);
                     let icon_pos = pos + egui::vec2(icon_size.x / 2.0 - icon_galley.size().x / 2.0, galley_height / 2.0 - icon_galley.size().y / 2.0);
                     ui.painter().galley(icon_pos, icon_galley, text_color);
                }
            };

            match icon_placement {
                IconPlacement::Start => {
                    if has_icon {
                        paint_icon(ui, text_pos);
                        text_pos.x += icon_size.x + icon_gap;
                    }
                    ui.painter().galley_with_override_text_color(text_pos, galley, text_color);
                }
                IconPlacement::End => {
                    let mut text_draw_pos = text_pos;
                    ui.painter().galley_with_override_text_color(text_draw_pos, galley, text_color);
                    if has_icon {
                        text_draw_pos.x += content_width - icon_size.x;
                        paint_icon(ui, text_draw_pos);
                    }
                }
            }
        }

        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), &text));
        response
    }
}

pub struct ButtonGroup {
    size: Option<ButtonSize>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self { size: None }
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut ButtonGroupWriter) -> R,
    ) -> R {
        let mut writer = ButtonGroupWriter {
            ui,
            size: self.size,
            buttons: Vec::new(),
        };
        let res = add_contents(&mut writer);
        writer.render();
        res
    }
}

pub struct ButtonGroupWriter<'a> {
    ui: &'a mut egui::Ui,
    size: Option<ButtonSize>,
    buttons: Vec<Button<'a>>,
}

impl<'a> ButtonGroupWriter<'a> {
    pub fn add(&mut self, mut button: Button<'a>) {
        if let Some(size) = self.size {
            button = button.size(size);
        }
        self.buttons.push(button);
    }

    fn render(self) {
        self.ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = -1.0; // Overlap borders
            let count = self.buttons.len();
            for (i, button) in self.buttons.into_iter().enumerate() {
                let position = if count <= 1 {
                    ButtonPosition::None
                } else if i == 0 {
                    ButtonPosition::First
                } else if i == count - 1 {
                    ButtonPosition::Last
                } else {
                    ButtonPosition::Middle
                };
                ui.add(button.set_position(position));
            }
        });
    }
}

fn paint_dashed_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    corner_radius: CornerRadius,
    stroke: egui::Stroke,
    dash_length: f32,
    gap_length: f32,
) {
    let rect = rect.shrink(stroke.width / 2.0);

    // We need to handle each side with its own rounding
    let nw = corner_radius.nw as f32;
    let ne = corner_radius.ne as f32;
    let sw = corner_radius.sw as f32;
    let se = corner_radius.se as f32;

    // Top
    paint_dashed_line(painter, rect.left_top() + egui::vec2(nw, 0.0), rect.right_top() - egui::vec2(ne, 0.0), stroke, dash_length, gap_length);
    // Bottom
    paint_dashed_line(painter, rect.left_bottom() + egui::vec2(sw, 0.0), rect.right_bottom() - egui::vec2(se, 0.0), stroke, dash_length, gap_length);
    // Left
    paint_dashed_line(painter, rect.left_top() + egui::vec2(0.0, nw), rect.left_bottom() - egui::vec2(0.0, sw), stroke, dash_length, gap_length);
    // Right
    paint_dashed_line(painter, rect.right_top() + egui::vec2(0.0, ne), rect.right_bottom() - egui::vec2(0.0, se), stroke, dash_length, gap_length);

    // Corners
    paint_dashed_arc(painter, rect.left_top() + egui::vec2(nw, nw), nw, std::f32::consts::PI, 1.5 * std::f32::consts::PI, stroke, dash_length, gap_length);
    paint_dashed_arc(painter, rect.right_top() + egui::vec2(-ne, ne), ne, 1.5 * std::f32::consts::PI, 2.0 * std::f32::consts::PI, stroke, dash_length, gap_length);
    paint_dashed_arc(painter, rect.right_bottom() + egui::vec2(-se, -se), se, 0.0, 0.5 * std::f32::consts::PI, stroke, dash_length, gap_length);
    paint_dashed_arc(painter, rect.left_bottom() + egui::vec2(sw, -sw), sw, 0.5 * std::f32::consts::PI, std::f32::consts::PI, stroke, dash_length, gap_length);
}

fn paint_dashed_arc(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    stroke: egui::Stroke,
    dash_length: f32,
    gap_length: f32,
) {
    if radius <= 0.0 { return; }
    let arc_length = (end_angle - start_angle).abs() * radius;
    let mut current_dist = 0.0;

    while current_dist < arc_length {
        let segment_start_angle = start_angle + (current_dist / radius);
        let segment_end_dist = (current_dist + dash_length).min(arc_length);
        let segment_end_angle = start_angle + (segment_end_dist / radius);

        let points: Vec<egui::Pos2> = (0..=5)
            .map(|i| {
                let t = i as f32 / 5.0;
                let a = segment_start_angle + (segment_end_angle - segment_start_angle) * t;
                center + egui::vec2(a.cos(), a.sin()) * radius
            })
            .collect();

        painter.add(egui::Shape::line(points, stroke));
        current_dist += dash_length + gap_length;
    }
}

fn paint_dashed_line(
    painter: &egui::Painter,
    start: egui::Pos2,
    end: egui::Pos2,
    stroke: egui::Stroke,
    dash_length: f32,
    gap_length: f32,
) {
    let dir = (end - start).normalized();
    let dist = start.distance(end);
    let mut current_dist = 0.0;

    while current_dist < dist {
        let segment_end_dist = (current_dist + dash_length).min(dist);
        painter.line_segment(
            [start + dir * current_dist, start + dir * segment_end_dist],
            stroke,
        );
        current_dist += dash_length + gap_length;
    }
}

fn create_gradient_mesh(
    rect: egui::Rect,
    _corner_radius: CornerRadius,
    color1: egui::Color32,
    color2: egui::Color32,
) -> egui::Mesh {
    let mut mesh = egui::Mesh::default();

    // Simple 4-vertex mesh for the gradient
    // Clipping handles the rounding
    mesh.vertices.push(egui::epaint::Vertex { pos: rect.left_top(), uv: egui::epaint::WHITE_UV, color: color1 });
    mesh.vertices.push(egui::epaint::Vertex { pos: rect.right_top(), uv: egui::epaint::WHITE_UV, color: color2 });
    mesh.vertices.push(egui::epaint::Vertex { pos: rect.right_bottom(), uv: egui::epaint::WHITE_UV, color: color2 });
    mesh.vertices.push(egui::epaint::Vertex { pos: rect.left_bottom(), uv: egui::epaint::WHITE_UV, color: color1 });
    mesh.indices.extend([0, 1, 2, 0, 2, 3]);

    mesh
}

fn lerp_color(c1: egui::Color32, c2: egui::Color32, t: f32) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(
        egui::lerp((c1.r() as f32)..=(c2.r() as f32), t) as u8,
        egui::lerp((c1.g() as f32)..=(c2.g() as f32), t) as u8,
        egui::lerp((c1.b() as f32)..=(c2.b() as f32), t) as u8,
        egui::lerp((c1.a() as f32)..=(c2.a() as f32), t) as u8,
    )
}
