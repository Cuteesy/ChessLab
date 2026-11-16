# ChessLab

ChessLab is a Tauri 2 desktop chess analysis application built on
[En Croissant](https://github.com/franciscoBSalgueiro/en-croissant).

The interface uses React, Mantine and Chessground, with a Rust backend for
engine processes, database access and native window management. ChessLab adds
coach arrows, threat analysis, human-like engine limiting, a floating
always-on-top board and the ChessLab move classification and opening workflow.

## Development

```bash
corepack pnpm install
corepack pnpm tauri dev
```

Useful checks:

```bash
corepack pnpm exec tsc --noEmit
corepack pnpm test
corepack pnpm build-vite
```

The Rust backend can be checked independently:

```bash
cd src-tauri
cargo check
```

## Floating board

Open it from the application menu (`ChessLab` -> `Floating board`). The overlay
is a separate frameless window that mirrors the active board. Close it with the
close button in its title bar.

## License

GPL-3.0-only. See [LICENSE](LICENSE). This project is derived from En Croissant,
copyright its contributors.
