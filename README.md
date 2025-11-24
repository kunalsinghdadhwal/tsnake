# TSNAKE

A beautiful, fully-featured terminal Snake game written in Rust.

## Features

- **Smooth gameplay** with 120ms base tick rate
- **Multiple control schemes**: Arrow keys, WASD, and HJKL (Vim-style)
- **Progressive difficulty**: Speed increases every 8 food items eaten
- **Persistent high scores**: Top 10 scores saved locally
- **Beautiful UI**: Colorful theme with green snake, red food, cyan borders, and yellow score
- **Game states**: Main menu, pause, game over, and high scores screen
- **Responsive design**: Adapts to terminal size

## Installation

### From source

```bash
git clone https://github.com/kunalsinghdadhwal/tsnake.git
cd tsnake
cargo install --path .
```

### From crates.io 

```bash
cargo install tsnake
```

## Usage

Simply run the binary:

```bash
tsnake
```

### Controls

**Main Menu:**
- Arrow Keys / W/S / K/J: Navigate menu
- Enter / Space: Select option
- Q / Esc: Quit

**In-Game:**
- Arrow Keys / WASD / HJKL: Move snake
- P: Pause game
- Q / Esc: Quit to menu

**Paused:**
- P / Space / Enter: Resume
- Q / Esc: Quit to menu

**Game Over:**
- Arrow Keys / W/S / K/J: Select option
- Enter / Space: Confirm selection

## Gameplay

- Eat the red hearts (❤) to grow and score points
- Each food item gives you 10 points
- Avoid running into yourself
- Avoid hitting the walls (game over!)
- Speed increases every 8 food items eaten
- Modern 2025 aesthetic with neon colors and sleek design

## High Scores

High scores are automatically saved to:
- Linux/macOS: `~/.config/tsnake/highscores.json`
- Windows: `%APPDATA%\tsnake\highscores.json`

Top 10 scores are preserved with timestamp, score, and snake length.

## Requirements

- Rust 2021 edition or later
- A terminal with Unicode support for best experience

## Dependencies

- ratatui 0.28+ (Terminal UI framework)
- crossterm 0.28+ (Terminal manipulation)
- rand 0.8 (Random number generation)
- serde & serde_json (High score serialization)
- dirs-next (Config directory detection)
- chrono (Timestamps)

## Building

### Debug build

```bash
cargo build
```

### Optimized release build

```bash
cargo build --release
```
## Author

[Kunal Singh Dadhwal](https://kunalsinghdev.xyz)