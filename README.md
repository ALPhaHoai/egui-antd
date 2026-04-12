# egui-antd

A port of [Ant Design](https://ant.design/) components to [egui](https://github.com/emilk/egui) Rust.

Currently implemented:
- [Button](https://ant.design/components/button)
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
