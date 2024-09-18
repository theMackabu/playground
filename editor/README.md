# Meow Editor

A simple, yet powerful terminal-based text editor written in Rust.

## Features

- Theming support
- Tree-sitter syntax highlighting
- Customizable tab width

## Usage

```
meow --[OPTIONS] <FILE_PATH>
```

### Options

- `--disable-mouse-interaction`, `-d`: Disable mouse navigation
- `--tab-width`, `-t <TAB_WIDTH>`: Set custom tab width (default: 4)
- `--theme`, `-s <THEME>`: Specify a custom theme
- `--relative-line-numbers`, `-r`: Use relative line numbers

## Keyboard Shortcuts

- `Ctrl+Q`: Quit (with save prompt if changes are unsaved)
- `Ctrl+S`: Save file
- `Ctrl+Z`: Undo
- `Ctrl+Y`: Redo
- `Ctrl+C/X/V`: Copy/Cut/Paste (internal clipboard)
- `Alt+C/X/V`: Copy/Cut/Paste (system clipboard)
- `Ctrl+B/F`: Page Up/Down (full page)
- `Ctrl+U/D`: Page Up/Down (half page)
