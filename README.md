# egui-antd

A port of [Ant Design](https://ant.design/) components to [egui](https://github.com/emilk/egui) Rust.

Currently implemented:
- [Button](https://ant.design/components/button)

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

## Usage

Add `egui` and `eframe` to your `Cargo.toml`. Include the `antd` module in your project.

```rust
use egui_antd::{Button, ButtonType, ButtonSize, ButtonShape};

ui.add(
    Button::new("Click Me")
        .button_type(ButtonType::Primary)
        .size(ButtonSize::Large)
        .shape(ButtonShape::Round)
);
```

## Running the Demo

```bash
cargo run --example demo
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
