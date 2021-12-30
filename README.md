[![TicToc](https://github.com/rherrmannr/RustTicTacToe/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rherrmannr/RustTicTacToe/actions/workflows/rust.yml)
# RusTicTacToe
CLI and GUI based TicTacToe implemented in Rust. Using SDL2 for GUI https://github.com/Rust-SDL2/rust-sdl2.

## Build, Run and Test
In order to build the project run:
```
$ cargo build 
# run with CLI
$ cargo run
# run with GUI
$ cargo run -- gui
```
Tests can be executed with:
```
$ cargo test
```

## Playing the Game
Payer `X` starts the game. The command line expects numbers between `0` and `2` for selecting the row and column. If the field is already in use, the user can repeat the selection, until the sign is set.
```
It's X's turn.
Type in the row.
1
Type in the column.
1

---
-X-
---

It's O's turn.
Type in the row.
0
Type in the column.
0

O--
-X-
---
```

After one player has one, the game will be restarted automatically.
``` 
OXO
-X-
-X-

X has won!

---
---
---

It's X's turn.
Type in the row.
```
