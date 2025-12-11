# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Defira is a secret management application using PGP for encryption. It provides a GUI for managing encrypted secrets with support for multiple recipients.

## Build and Run Commands

```bash
# Run the application (with Vulkan/NVIDIA setup)
RUST_LOG=defira=debug VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json VK_LAYER_PATH=/usr/share/vulkan/explicit_layer.d cargo run

# Build
cargo build

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Format code
cargo fmt

# Check lints
cargo clippy
```

## Architecture

Defira is a Rust GUI application built with the [iced](https://iced.rs/) framework (v0.13.1), using The Elm Architecture (TEA) pattern.

### Core Structure

- **main.rs** - Application entry point, initializes logging and launches the iced application using `file_explorer` module's update/view functions
- **file_explorer.rs** - Main UI component implementing a file tree browser with expand/collapse, selection, and right-click context menus. Contains:
  - `State` struct holding expanded/selected paths and context menu state
  - `FileAction` enum for all user interactions
  - `update()` function for state mutations
  - `view()` function for rendering
- **context_menu.rs** - Reusable generic context menu component with `MenuItem<Message>` that can be used by any module
- **assets.rs** - Embedded SVG icons loaded via `include_bytes!` macro
- **lib.rs** - Library crate for PGP operations (editing secrets and managing recipients)

### Key Patterns

The app follows iced's TEA pattern: `State` + `Message` enum + `update()` + `view()`. The file explorer tracks cursor position to position context menus at click location.

### Assets

SVG icons are in `assets/` directory and embedded at compile time via `assets.rs`.
