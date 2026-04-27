# egui-antd

A port of [Ant Design](https://ant.design/) components to [egui](https://github.com/emilk/egui) Rust.

Currently implemented:
- [Button](https://ant.design/components/button)
- [Checkbox](https://ant.design/components/checkbox)
- [Input](https://ant.design/components/input)
- [Space](https://ant.design/components/space)
- [Tabs](https://ant.design/components/tabs)
- [Dropdown](https://ant.design/components/dropdown)
- [ConfigProvider](https://ant.design/components/config-provider)

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

## Checkbox Features

- **States**: Checked, unchecked, `indeterminate` (dash)
- **Disabled**: Visual disabled state with correct cursor
- **Label**: Optional inline label text
- **Wave Effect**: Ant Design style ripple on click
- **Hover animation**: Smooth border/fill color transition
- **`CheckboxGroup`**: Render a group from a `Vec<CheckboxOption>`, tracking selected values as `Vec<String>`
- **`CheckboxOption`**: Per-option `value`, `label`, and optional `disabled`

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

## ConfigProvider Features

- Wraps child UI with a scoped `Theme` applied via egui context storage.
- **`ButtonTheme`**: Override per-component token defaults (e.g., disabled background colors).
- Nested `ConfigProvider`s are supported; the inner theme is restored on exit.

## Usage

Add `egui-antd` to your `Cargo.toml`:

```toml
[dependencies]
egui-antd = "0.1.0"
```

### Button Example

```rust
use egui_antd::{Button, ButtonType, ButtonSize, ButtonShape};

ui.add(
    Button::new("Click Me")
        .button_type(ButtonType::Primary)
        .size(ButtonSize::Large)
        .shape(ButtonShape::Round)
);
```

### Checkbox Example

```rust
use egui_antd::{Checkbox, CheckboxGroup, CheckboxOption};

let mut checked = false;

// Basic checkbox with label
ui.add(Checkbox::new(&mut checked).label("Remember me"));

// Indeterminate state
ui.add(Checkbox::new(&mut checked).indeterminate(true));

// Disabled
ui.add(Checkbox::new(&mut checked).disabled(true));

// Group — tracks selected values as Vec<String>
let mut selected: Vec<String> = vec!["apple".to_string()];
CheckboxGroup::new(&mut selected)
    .options(vec![
        CheckboxOption::new("apple", "Apple"),
        CheckboxOption::new("pear", "Pear"),
        CheckboxOption::new("orange", "Orange").disabled(true),
    ])
    .show(ui);
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

let mut active_key = "1".to_string();

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

### ConfigProvider Example

```rust
use egui_antd::{ConfigProvider, Theme, ComponentsTheme, ButtonTheme};

let theme = Theme {
    components: ComponentsTheme {
        button: ButtonTheme {
            default_bg_disabled: Some(egui::Color32::from_gray(230)),
            ..Default::default()
        },
    },
};

ConfigProvider::new()
    .theme(theme)
    .show(ui, |ui| {
        // All components rendered here use the custom theme
    });
```

## Running the Demo

```bash
cargo run --example button
cargo run --example checkbox
cargo run --example input
cargo run --example tabs
```

## Implementation Details

Components are implemented as custom `egui::Widget`s. They use `egui::Painter` to manually render borders, backgrounds, and text with Ant Design 5.0 design tokens:

- **Primary Blue**: `#1677ff` (Hover: `#4096ff`, Active: `#0958d9`)
- **Error Red**: `#ff4d4f` (Hover: `#ff7875`, Active: `#d93635`)
- **Border Radius**: `6px` (Standard for AntD 5.0)
- **Control Size**: `16px` (Checkbox, radio)

## Development

This project uses the official `antd` npm package as a reference for components and design specifications.

```bash
npm install
```

---
🤖 Generated with [Claude Code](https://claude.com/claude-code)
