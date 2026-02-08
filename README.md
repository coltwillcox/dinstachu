# 🌆 FM84 — File Manager '84

```
███████╗███╗   ███╗ █████╗ ██╗  ██╗
██╔════╝████╗ ████║██╔══██╗██║  ██║
█████╗  ██╔████╔██║╚█████╔╝███████║
██╔══╝  ██║╚██╔╝██║██╔══██╗╚════██║
██║     ██║ ╚═╝ ██║╚█████╔╝     ██║
╚═╝     ╚═╝     ╚═╝ ╚════╝      ╚═╝
```

> 💜 *A synthwave-infused dual-pane TUI file manager, forged in Rust* 💜

**Version 0.4.1** ▀▄▀▄ *Neon Dreams Edition*

---

> ⚠️ **WARNING: ALPHA SOFTWARE** ⚠️
>
> 🚧 *This is a work in progress!* 🚧
>
> Things **will** break. Features **may** eat your files. Use at your own risk.
> Back up your data. Trust no one. Not even this README.
>
> *We're still soldering the circuits on this one, choom.* 🔧

<img src="https://raw.githubusercontent.com/coltwillcox/fm84/master/images/screen-main-0.png" width="801">

---

## 🔮 The Vibe

Step into the neon-lit streets of '84. Where files flow like synth waves and directories pulse with purple energy. FM84 is your retro-futuristic companion for navigating the filesystem — dual-pane style, just like the legends intended.

Built with 💜 in **Rust** using **Ratatui** + **Crossterm**.

---

## ⚡ Features

### 🗂️ Navigation
- 📁 **Dual-pane layout** — because one panel is never enough
- ⌨️ **Arrow keys** — glide through your files
- 🏠 **Home/End** — teleport to the edges
- 📄 **PageUp/PageDown** — cruise in style
- 🔀 **Tab** — switch between panels like flipping cassettes
- ↩️ **Enter** — dive into directories
- ⬅️ **Backspace** — ascend to parent realm

### 🔍 Quick Search
- 🔎 **Type-ahead search** — just start typing to find files
- ⬆️⬇️ **Navigate matches** — Up/Down arrows jump between results
- 🧹 **Esc** — clear the search vibes

### 📝 File Operations
- **F1** 💡 — Help/About
- **F2** ✏️ — Rename files & folders
- **F3** 👁️ — View files with **syntax highlighting**
- **F4** 📝 — Edit files (built-in editor, Ctrl+S to save)
- **F5** 📋 — Copy to other panel
- **F6** 📦 — Move to other panel
- **F7** 📂 — Create new directories
- **F8** 🗑️ — Delete files & folders (with confirmation)
- **F9** 💻 — Open external terminal in current directory
- **F10** 🚪 — Exit to the void
- **Space** ✅ — Select/deselect files for batch operations
- 🖱️ **Double-click** — open directories or view files

### 🎨 Viewer (F3)
- 🌈 **Syntax highlighting** for Rust, Python, JS, TS, JSON, TOML, YAML, Markdown, Shell, C/C++, HTML, CSS
- 📊 **Line numbers** in the gutter
- 🔢 **Status bar** — filename, line count, file size, detected syntax
- 🚫 **Binary detection** — won't melt your terminal with garbage

### ✍️ Editor (F4)
- 📄 **Full text editing** — cursor navigation, insert, delete
- 💾 **Save** — F2 or Ctrl+S
- 📍 **Line/Column tracking** — always know where you are
- ⚠️ **Modified indicator** — never lose unsaved changes

---

## 🎹 Keybindings

| Key | Action |
|-----|--------|
| `↑` `↓` `←` `→` | Navigate |
| `Tab` | Switch panels |
| `Enter` | Open directory / Execute |
| `Backspace` | Go to parent directory |
| `Home` / `End` | Jump to first / last item |
| `PageUp` / `PageDown` | Page navigation |
| `[a-z0-9]` | Quick search |
| `Esc` | Clear search / Close dialogs |
| `F1` | Help |
| `F2` | Rename |
| `F3` | View file |
| `F4` | Edit file |
| `F5` | Copy to other panel |
| `F6` | Move to other panel |
| `F7` | Create directory |
| `F8` | Delete |
| `F9` | Open terminal |
| `F10` | Quit |
| `Space` | Select/deselect file |

---

## 🛠️ Build & Run

```bash
# 🦀 Clone the future
git clone https://github.com/coltwillcox/fm84.git
cd fm84

# ⚙️ Compile with cargo
cargo build --release

# 🚀 Launch into the neon grid
cargo run --release
```

---

## 📀 Releases

Pre-built binaries beam down from the neon sky:

| Platform | Architecture | Format |
|----------|--------------|--------|
| 🐧 **Linux** | x86_64, ARM64 | `.tar.gz` |
| 🍎 **macOS** | Intel, Apple Silicon | `.tar.gz` |
| 🪟 **Windows** | x86_64 | `.zip` |

```bash
# 📥 Download from GitHub Releases
# https://github.com/coltwillcox/fm84/releases

# 🎮 Extract and run
tar -xzf fm84-v*.tar.gz
./fm84
```

*No cargo? No problem. Grab a binary and jack in.* 🔌

---

## 📦 Dependencies

- 🦀 **Rust** (2024 edition)
- 🖥️ **ratatui** — TUI framework
- ⌨️ **crossterm** — Terminal magic
- 🎨 **syntect** — Syntax highlighting
- 🕐 **chrono** — Time vibes

---

## 🌃 Aesthetic

```
    ╔══════════════════════════════════════╗
    ║  VIOLET DREAMS • PURPLE HAZE • NEON  ║
    ║    ▓▓▓▓▓ SYNTHWAVE FOREVER ▓▓▓▓▓     ║
    ╚══════════════════════════════════════╝
```

The color palette channels pure 80s energy:
- 💜 **Violet borders** — `#743AD5`
- 🔮 **Purple selections** — `#9400D3`
- 💗 **Magenta directories** — `#FF00FF`
- 🩵 **Cyan accents** — `#00FFFF`
- 💙 **Soft purple files** — `#7289DA`

---

## 🎵 Inspired By

- 🌅 FM-84 (the band, obviously)
- 🖥️ Midnight Commander
- 🎮 Total Commander
- 🌆 Synthwave aesthetics
- 📼 That retro terminal feel

---

## 📜 License

*Ride free through the neon grid.*

---

<p align="center">
  <strong>💜 FM84 💜</strong><br>
  <em>Where every file operation feels like a synth drop</em><br>
  <code>▀▄▀▄▀▄ v0.4.1 ▄▀▄▀▄▀</code>
</p>
