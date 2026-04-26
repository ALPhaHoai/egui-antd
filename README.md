# egui-antd

A port of [Ant Design](https://ant.design/) components to [egui](https://github.com/emilk/egui) Rust.

Currently implemented:
- [Button](https://ant.design/components/button)
- [Input](https://ant.design/components/input)
- [Space](https://ant.design/components/space)
- [Tabs](https://ant.design/components/tabs)
- [Dropdown](https://ant.design/components/dropdown)

## Button Features

- **Types**: `Primary`, `Default`, `Dashed`, `Link`, `Text`, `Gradient`
- **Shapes**: `Default`, `Circle`, `Round`
- **Sizes**: `Large`, `Middle`, `Small`
- **Variants**: `Danger`, `Ghost`
- **States**: `Disabled`, `Loading`, `Block` (full width), `Hover`, `Active`
- **Interactive**: 
  - Smooth color transitions for hover/active states.
  - **Wave Effect**: Ant Design style spreading ripple on click.
- **Icons**: Supports both text-based icons (emojis) and graphical icons (`egui::Image` via SVG/PNG).
- **Typography**: Automatic spacing for two-character Chinese strings (e.g., "确认" -> "确 认").
- **Groups**: `ButtonGroup` for cohesive multi-button layouts.

## Input Features

- **Components**: `Input`, `InputNumber`, `Password`, `Search`, `TextArea`, `OTP`, `InputGroup`
- **Sizes**: `Large` (40px), `Middle` (32px), `Small` (24px)
- **Variants**: `Outlined`, `Filled`, `Borderless`, `Underlined`
- **Status**: `Normal`, `Error`, `Warning` with matching border colors
- **Features**:
  - `prefix` / `suffix`: Custom widgets inside the input
  - `addon_before` / `addon_after`: Grouped addon sections
  - `allow_clear`: Clear icon to reset the input
  - `show_count` / `max_length`: Character counting
  - `disabled`, `read_only`
- **InputNumber**: Text input with up/down spinner controls, min/max clamping, keyboard arrow support
- **Password**: Visibility toggle, clear support
- **Search**: Search icon suffix, optional `enter_button` with primary button style
- **TextArea**: Multiline input with `auto_size(min_rows, max_rows)`, clear icon, character count
- **OTP**: One-time password input with configurable `length`, `mask`, `formatter`, and custom `separator`
- **InputGroup**: Group multiple inputs with optional `compact` mode

## Space Features

- **Direction**: `Horizontal`, `Vertical`
- **Size**: `Small`, `Middle`, `Large`, or custom `f32` value
- **Align**: `Start`, `End`, `Center`, `Baseline`
- **Wrap**: Support for wrapping items in multiple lines
- **Compact**: `SpaceCompact` for grouping components tightly (e.g., buttons, inputs)

## Tabs Features

- **Types**: `Line`, `Card`, `EditableCard`
- **Placements**: `Top`, `Bottom`, `Start`, `End`
- **Sizes**: `Large`, `Medium`, `Small`
- **Features**:
  - `centered`: Center tabs within the tab bar
  - `hide_add`: Hide the add button in `EditableCard` type
  - `tab_bar_extra_content`: Custom content on the left or right of the tab bar
  - `on_edit` support: Callback for adding or removing tabs
  - `icon` support: Tab panes can include icons
- **Overflow handling**: Smooth scrolling with scroll arrows for long tab bars.

## Dropdown Features

- **Trigger**: Supports triggering from any `Button`
- **Menu**: Ant Design 5.0 styled popup menu with `menu_item` helpers.

## Usage

Add `egui` and `eframe` to your `Cargo.toml`. Include the `egui-antd` crate in your project.

```rust
use egui_antd::{Button, ButtonType, ButtonSize, ButtonShape};

ui.add(
    Button::new("Click Me")
        .button_type(ButtonType::Primary)
        .size(ButtonSize::Large)
        .shape(ButtonShape::Round)
);
```

### Input Example

```rust
use egui_antd::{Input, InputSize, InputVariant, Search, Password, TextArea};

let mut text = String::new();

// Basic input with prefix/suffix
ui.add(
    Input::new(&mut text)
        .hint_text("Enter username")
        .size(InputSize::Large)
        .prefix(|ui| { ui.label("@"); })
        .allow_clear(true),
);

// Search with enter button
ui.add(
    Search::new(&mut text)
        .hint_text("Search...")
        .enter_button_text("Search"),
);

// Password
ui.add(Password::new(&mut text).hint_text("Password"));

// TextArea with auto-sizing and character count
ui.add(
    TextArea::new(&mut text)
        .auto_size(2, Some(6))
        .show_count(true)
        .max_length(200),
);
```

### Tabs Example

```rust
use egui_antd::{Tabs, TabPane};

let mut active_key = "1".to_string(); // Managed by your state

Tabs::new("my_tabs")
    .active_key(&mut active_key)
    .items(vec![
        TabPane::new("1", "Tab 1"),
        TabPane::new("2", "Tab 2"),
    ]).show(ui, |ui, key| {
        match key {
            "1" => { ui.label("Content of Tab 1"); },
            "2" => { ui.label("Content of Tab 2"); },
            _ => {}
        }
    });
```

## Running the Demo

```bash
cargo run --example button
cargo run --example input
cargo run --example tabs
```

## Implementation Details

The `Button` is implemented as a custom `egui::Widget`. It uses `egui::Painter` to manually render the borders, backgrounds, and text with specific Ant Design 5.0 design tokens:

- **Primary Blue**: `#1677ff` (Hover: `#4096ff`, Active: `#0958d9`)
- **Error Red**: `#ff4d4f` (Hover: `#ff7875`, Active: `#d93635`)
- **Border Radius**: `6px` (Standard for AntD 5.0)

## Development

This project uses the official `antd` npm package as a reference for components and design specifications.

```bash
npm install
```

---
🤖 Generated with [Claude Code](https://claude.com/claude-code)
