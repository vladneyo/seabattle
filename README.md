# SeaBattle (console)

Console-based “Sea Battle” / “Battleship” implemented in Rust.

## Overview

This project renders **two 10×10 grids** (“your field” and “enemy field”) in the terminal and lets you **enter strike coordinates**. Ships for both sides are **randomly spawned** at the start.

At the moment, the game is primarily a **board + spawning + rendering + strike marking** implementation:
- You can strike enemy cells and see hits/misses.
- Your strike history is printed.
- There is **no enemy turn** and **no win/lose condition** yet.

## Gameplay & rules (as implemented)

- **Board size**: 10×10 (`Field` stores `grid: Vec<Vec<FieldCell>>`)
- **Cell types**:
  - Sea cell (`FieldCell::Sea(SeaCell)`)
  - Ship cell (`FieldCell::Ship(ShipPart)`)
- **Strike**:
  - A strike calls `Field::strike(&Coord)` which calls `on_strike()` on the targeted `FieldCell`.
  - Both `SeaCell` and `ShipPart` store a `hit: bool`.
- **Fleet / ship spawning**:
  - Done by `ShipSpawner`.
  - Default fleet config is `[(4,1), (3,2), (2,3), (1,4)]` (blocks, quantity) — classic Battleship layout.
  - Ships are spawned as **straight segments** (horizontal/vertical) using a random start + a randomly chosen valid direction.
  - Spawner enforces **no overlap and 1-cell spacing** between ships (including diagonals) by checking candidates against a padded occupancy matrix.

## Controls / input format

- **Strike input**: type a coordinate like `a0`, `c7`, `j9`
  - Rows are letters `a..j`
  - Columns are digits `0..9`
- **Quit**: type `stop`

Notes:
- Input parsing currently assumes exactly 2 characters for the coordinate (e.g. `a0`).
- Invalid input will likely panic due to `unwrap()` usage in the main loop.

## Rendering (symbols)

Cells are drawn using `colored`:
- **Unknown sea**: `_` (blue)
- **Miss (struck sea)**: `•` (yellow)
- **Ship**: `#`
- **Hit ship part**: `X` (orange)

Both boards are drawn using the same logic, so **enemy ships are visible** right now.

## How it works (code map)

- **Entry point**: `src/main.rs`
  - Creates `Game` and calls `run()`.
- **Game loop**: `src/game.rs`
  - Creates `user_field` and `enemy_field`
  - Spawns ships on both fields via `ShipSpawner`
  - Loop:
    - clears screen (`Console::clear_screen`)
    - draws both grids (`Display`)
    - reads input (`read_line!`)
    - parses `Coord` and applies `enemy_field.strike()`
    - stores strike in `user_strike_history`
- **Board model**: `src/field.rs`, `src/cell.rs`
  - `FieldCell` is an enum of sea vs ship, both implement the `Cell` trait (`on_strike`, `draw`, `is_hit`).
- **Coordinates**: `src/coord.rs`, `src/point.rs`
  - `Point` is 0..9 and maps to row letters `a..j`.
  - `Coord` parses from strings like `a0` and prints as `a-0`.
- **Random ship spawning**: `src/ship_spawner.rs`, `src/matrix.rs`
  - `ShipSpawner::get_end_coord()` chooses a valid end coordinate in one of four directions.
  - `Matrix` is used to detect intersections between already-placed ships and a candidate ship.

## Run

Requires Rust (edition 2024).

```bash
cargo run
```

## Current limitations / TODOs

- Add enemy AI turn (and use `enemy_strike_history` which already exists in `Game`).
- Add win/lose detection (e.g., all `ShipPart`s hit).
- Hide enemy ships when drawing (fog-of-war).
- Validate input and prevent striking the same cell repeatedly.