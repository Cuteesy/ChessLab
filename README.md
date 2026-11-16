# ChessLab

ChessLab is a desktop chess analysis application with a modern Tauri and React
interface. It is a GPL-3.0 fork of [En Croissant](https://github.com/franciscoBSalgueiro/en-croissant),
extended with the coaching, human-like engine, threat display and floating-board
features from the original ChessLab project.

## Features

- Stockfish analysis with multi-PV evaluation and engine management
- Coach mode: shows the best move only for the side you are playing
- Threat arrows: highlights the opponent's strongest reply
- Human-like engine strength for analysis and engine games
- Frameless, always-on-top floating board that mirrors the main window
- Move classification, accuracy reports and opening explorer
- Local game databases, repertoires, puzzles and PGN import/export

## Repository layout

- `app/` - the current ChessLab desktop app (Tauri 2, React, TypeScript, Rust)
- `chesslab/` - the legacy Python/Tkinter implementation, kept temporarily as a
  behaviour reference until feature parity is verified
- `.upstream/en-croissant` - a local reference clone of the upstream project; it
  is not part of the build

## Development

Requirements: Node.js 22 or newer, Rust stable, pnpm (through corepack), and the
Tauri system dependencies for your platform.

```bash
cd app
corepack pnpm install
corepack pnpm tauri dev
```

Frontend checks:

```bash
cd app
corepack pnpm exec tsc --noEmit
corepack pnpm test
corepack pnpm build-vite
```

## License

ChessLab is distributed under the GNU General Public License, version 3 only.
See [LICENSE](LICENSE). En Croissant is copyright its contributors and is also
licensed under GPL-3.0-only.
