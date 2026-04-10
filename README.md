# egui-antd

A port of [Ant Design](https://ant.design/) components to [egui](https://github.com/emilk/egui) Rust.

Currently implemented:
- [Button](https://ant.design/components/button)

## Button Features

- **Types**: `Primary`, `Default`, `Dashed`, `Link`, `Text`
- **Sizes**: `Large`, `Middle`, `Small`
- **States**: `Danger`, `Disabled`, `Loading`, `Block` (full width)
- **Interactive**: Hover effects that match the Ant Design palette.

## Usage

Add `egui` and `eframe` to your `Cargo.toml`. Copy the `src/antd` directory to your project.

```rust
use antd::{Button, ButtonType, ButtonSize};

ui.add(
    Button::new("Click Me")
        .button_type(ButtonType::Primary)
        .size(ButtonSize::Large)
);
```

## Running the Demo

```bash
cargo run
```

## Implementation Details

The `Button` is implemented as a custom `egui::Widget`. It uses `egui::Painter` to manually render the borders, backgrounds, and text with specific Ant Design colors:

- **Primary Blue**: `#1677ff`
- **Danger Red**: `#ff4d4f`
- **Border**: `#d9d9d9`
- **Disabled Background**: `#f5f5f5`

## Development

This project was generated using [Claude Code](https://claude.ai/).
