---
description: Ensure code correctness by running cargo check after edits
globs: ["**/*.rs", "Cargo.toml"]
---

# Cargo Check Rule

After editing any Rust source files or `Cargo.toml`, you must run `cargo check` to ensure the project still compiles and has no obvious errors.

## Usage

1. Complete your code edits.
2. Run `cargo check` in the terminal.
3. Fix any errors or warnings reported before finishing the task.
