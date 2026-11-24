# ChessLab Native

A Rust-only interface for ChessLab. No Tauri, no webview, no React: the window,
board and input handling are [`egui`](https://github.com/emilk/egui), the rules
come from [`shakmaty`](https://github.com/niklasf/shakmaty), and analysis is
Stockfish over UCI.

This is the first slice of the native rewrite, not a finished replacement for
the Tauri application in `../app`.

## Status

| Area | Native build |
|---|---|
| Board, legal moves, castling, en passant, promotion (auto-queen) | working |
| Stockfish analysis: score, depth, nodes, PV, eval bar | working |
| Move list (SAN), undo, reset, flip, FEN load | working |
| Engine path, depth control | working |
| PGN import, databases, repertoires, puzzles, multi-engine | not yet |

The Tauri build remains the feature-complete application and keeps the DuckDB
extension resolution fixed in `app/src-tauri/src/main.rs`. Moving databases here
requires linking DuckDB plus porting that resolver, which is the next step.

## Build and run

```bash
cd native
cargo build --release
./target/release/chesslab-native
```

The bundled engine lives at `../stockfish/stockfish`; point the Engines page at
it, or export `CHESSLAB_ENGINE=/path/to/stockfish`, or put `stockfish` on `PATH`.

## Layout

- `src/main.rs` — window setup and font installation
- `src/ui.rs` — app shell, board widget, analysis panel, pages
- `src/engine.rs` — UCI process, command queue and `info` parsing (unit tested)

## Font

`assets/DejaVuSans.ttf` is bundled because the board draws pieces as Unicode
chess glyphs. DejaVu is distributed under the Bitstream Vera license; see
<https://dejavu-fonts.github.io/License.html>.
