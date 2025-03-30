# Terminal Horizon

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust Build](https://github.com/<YOUR_GITHUB_USERNAME>/terminal_horizon/actions/workflows/rust.yml/badge.svg)](https://github.com/tomaszbawor/terminal_horizon/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/terminal_horizon.svg)](https://crates.io/crates/terminal_horizon)
A terminal-based roguelike game written in Rust, using the `ratatui` library for its user interface. Explore procedurally generated maps, manage your character, and (eventually) interact with entities within the world.

## ✨ Features

* **Terminal User Interface:** Built with `ratatui` and `crossterm`.
* **Procedural Map Generation:** Basic dungeon generation (walls, floors).
* **Player Movement:** Navigate the map using keyboard controls.
* **Main Menu:** Simple menu for starting or quitting the game.
* **Game State Management:** Tracks player stats, map, turn count, and action log.
* **Basic Entities:** Player character representation.

**Planned / In Development:**

* Enemy entities with simple AI (FSM).
* Field of View (FOV) algorithm.
* Combat system.
* Items and inventory.
* More complex map generation and features.

## 📋 Prerequisites

* **Rust:** Ensure you have a recent Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).

## ⚙️ Building

1.  Clone the repository:
    ```bash
    git clone [https://github.com/](https://github.com/)<YOUR_GITHUB_USERNAME>/terminal_horizon.git
    cd terminal_horizon
    ```
2.  Build the project:
    ```bash
    cargo build
    ```
    For an optimized release build:
    ```bash
    cargo build --release
    ```

## ▶️ Running

Execute the compiled binary:

```bash
cargo run
```
```
