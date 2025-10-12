# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based terminal user interface (TUI) application for browsing OSINT (Open Source Intelligence) tools. It provides a CLI interface to navigate various categories of OSINT tools based on the [CIPHER387/OSINT_STUFF_TOOL_COLLECTION](https://github.com/CIPHER387/OSINT_STUFF_TOOL_COLLECTION) cheat sheet.

**Note**: This project has been rewritten using modern Ratatui architecture with component-based design and Action-driven event handling.

The application is built using:
- **ratatui** (0.29.0) - For terminal UI rendering
- **crossterm** (0.28.1 with event-stream feature) - For cross-platform terminal handling and async events
- **tokio** (1.0 with full features) - For async runtime
- **color-eyre** (0.6.3) - For enhanced error handling and reporting
- **serde** (1.0 with derive) - For serialization support
- **strum** (0.26 with derive) - For enum utilities

## Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Run the application
cargo run

# Build for release
cargo build --release

# Run with release optimizations
cargo run --release
```

### Testing and Quality
```bash
# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

## New Architecture (Post-Rewrite)

The application now follows a modern, action-driven component architecture inspired by modern TUI frameworks:

### Core Modules
- **`main.rs`** - Application entry point with async runtime initialization
- **`app.rs`** - Main application coordinator with async event loop
- **`action.rs`** - Centralized Action enum for all user interactions
- **`tui.rs`** - Terminal management with async event handling
- **`config.rs`** - Configuration structures for keybindings and UI settings
- **`models.rs`** - Data models for OSINT categories and tools
- **`components/`** - Modular UI components with Component trait

### Component Architecture
All UI components implement the `Component` trait with these methods:
- `register_action_handler()` - Register action sender for inter-component communication
- `handle_key_events()` - Process keyboard input
- `update()` - Handle actions and update component state
- `draw()` - Render component to terminal

#### Key Components
- **`CategoryList`** - Hierarchical menu navigation with breadcrumb support
- **`PreviewPanel`** - Shows category details and subcategories/tools
- **`SearchBar`** - Tool search functionality (partially implemented)

### Action-Driven Event System
- **Actions**: All user interactions are represented as `Action` enum variants
- **Event Flow**: Key events → Actions → Component updates → UI re-render
- **Async Communication**: Components communicate via `tokio::sync::mpsc` channels

### Data Models
- **`OsintCategory`** - Hierarchical structure for tool categories
- **`OsintTool`** - Individual tool information (name, URL, description, tags)
- Supports nested categories with navigation stack for back/forward movement

### Application Flow
1. **Initialization**: Async TUI setup with event stream
2. **Event Loop**:
   - Async event processing (keyboard, render, etc.)
   - Action queue processing
   - Component state updates
   - UI rendering
3. **Navigation**:
   - `j/k` or arrow keys for vertical navigation
   - `Enter` to enter submenu
   - `h/Left/Backspace` to go back
   - `q/Esc` to exit or go back
   - `/` to activate search (planned)

### UI Layout
- **Top**: Search bar (3 lines)
- **Main**: Two-column layout (50/50 split)
  - Left: Category list with breadcrumb navigation
  - Right: Preview panel showing selected category details

### Focus Management
- **Focus States**: CategoryList, SearchBar
- **Focus Switching**: `/` activates search, `Esc` returns to category list
- **Visual Indicators**: Different border colors for focused components

## Navigation Controls
- `j` / `Down` - Move down in menu
- `k` / `Up` - Move up in menu
- `Enter` - Enter selected submenu or open tool
- `h` / `Left` / `Backspace` - Go back to parent menu
- `q` / `Esc` - Exit application or go back
- `/` - Activate search bar (planned)
- `Home` / `g` - Go to root category (planned)

## Development Notes

### Current Status
- ✅ Core architecture implemented
- ✅ Component system working
- ✅ Navigation fully functional with breadcrumbs
- ✅ Complete OSINT tool database integrated (100+ tools across 8+ major categories)
- ✅ Rich preview panel with tool details (name, URL, description)
- ✅ Tool opening in browser functionality (cross-platform)
- ✅ Visual improvements with icons and color coding
- ✅ Release build successful
- ⚠️ Terminal access issues in some environments
- 🚧 Search functionality partially implemented

### Implemented Features
- **Complete Tool Database**: Integrated real OSINT tools from cipher387's collection
- **Hierarchical Navigation**: Maps/Geolocation, Social Media, Messengers, Domain/IP, Image Search, Search Engines
- **Rich Tool Display**:
  - Tool name with numbering
  - Clickable URLs (🔗)
  - Detailed descriptions (📝)
  - Category statistics
- **Browser Integration**: Cross-platform URL opening (Windows/macOS/Linux)
- **Visual Enhancements**:
  - Color-coded categories
  - Icon indicators (▶ for navigation, 🔧 for tools)
  - Bold highlighting for tool names
  - Count displays for categories

### Sample Tool Categories Available
- **Social Media**: Twitter (8 tools), YouTube (6 tools), TikTok (4 tools), Facebook, Instagram, Reddit
- **Maps & Geolocation**: Social media photos (12 tools), Nature (5 tools), Aviation (3 tools), Maritime (4 tools), Railway (3 tools)
- **Messengers**: Telegram (7 tools), WhatsApp (4 tools), Skype (2 tools)
- **Domain/IP**: Investigation tools (4 tools), Website analysis (4 tools)
- **Image Search**: Reverse search (7 tools), Face recognition (3 tools)
- **Search Engines**: Universal tools (4 tools), IoT search (4 tools)

### Known Issues
- Terminal access error in some environments (likely crossterm/stderr issue)
- Many unused warnings (normal during development)
- Search bar needs completion
- Some categories have placeholders for future tool additions

### Future Enhancements
- Complete search functionality with filtering
- Add more tools to existing categories
- Implement favorites/bookmarks system
- Add configuration file support
- Improve error handling and recovery
- Add help system and keyboard shortcuts display
- Consider adding mouse support
- Add tool tagging and advanced filtering