# 📼 Changelog

All notable changes to FM84 will be documented in this file.

*The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).*

---

## [0.4.0] - 2026-02-08

### ✨ Added
- 💻 **F9 Terminal** — open external terminal window in current directory
  - Uses `$TERMINAL` env var, falls back to common terminal emulators

### 🛠️ Changed
- Version constant now reads from `Cargo.toml` at compile time (`env!("CARGO_PKG_VERSION")`)

---

## [0.3.2] - 2026-02-08

### 🛠️ Changed
- Display version number in title bar
- Updated screenshot

---

## [0.3.1] - 2026-02-07

### ✨ Added
- 📦 **F6 Move** — move files and directories to the other panel
  - Cross-filesystem support (copy + delete fallback)
- 🖱️ **Double-click** — open directories or view files with double-click
- ✅ **Multiple selection** — select multiple files for batch operations
- 📏 **Directory size** — show calculated size for selected directories
- 🎨 **File type colors** — files colored by extension for easy visual distinction
- 📂 **Sort by extension** — files ordered by extension by default

### 🛠️ Fixed
- Cross-filesystem copy/move operations
- Error handling on directory size calculation
- Rename click behavior
- Panel click behavior

---

## [0.2.0] - 2025-02-07

### ✨ Added
- 📋 **F5 Copy** — copy files and directories to the other panel
  - Recursive directory copying
  - Confirmation dialog with destination path
  - Duplicate detection

---

## [0.1.1] - 2025-02-07

### 🛠️ Fixed
- Cross-platform compatibility for Windows builds

---

## [0.1.0] - 2025-02-07

### ✨ Added
- 📁 **Dual-pane file manager** — navigate with style
- ⌨️ **Keyboard navigation** — Arrow keys, Home/End, PageUp/PageDown
- 🔀 **Tab switching** — flip between panels like cassettes
- ↩️ **Enter/Backspace** — dive into directories, ascend to parent
- 🔍 **Quick search** — type-ahead filtering with Up/Down navigation
- 💡 **F1 Help** — in-app help popup
- ✏️ **F2 Rename** — rename files and folders
- 👁️ **F3 View** — file viewer with syntax highlighting
  - Support for Rust, Python, JS, TS, JSON, TOML, YAML, Markdown, Shell, C/C++, HTML, CSS
  - Line numbers in gutter
  - Binary file detection
- 📝 **F4 Edit** — built-in text editor
  - Full cursor navigation
  - Insert, delete, backspace
  - F2/Ctrl+S to save
  - Modified indicator
- 📂 **F7 Create** — create new directories
- 🗑️ **F8 Delete** — delete files and folders with confirmation
- 🚪 **F10 Quit** — exit to the void
- 🎨 **Synthwave aesthetic** — violet borders, purple selections, magenta directories
- 🕐 **Live clock** — retro vibes in the header
- 📀 **GitHub Actions release workflow** — cross-platform binaries

---

<p align="center">
  <code>▀▄▀▄ SYNTHWAVE FOREVER ▄▀▄▀</code>
</p>
