<p align="center">
  <a href="https://github.com/Cuteesy/ChessLab">
    <img src="branding/banner.png" alt="ChessLab — Stockfish-powered chess analysis for the desktop" width="100%" />
  </a>
</p>

<p align="center">
  <a href="#overview">Overview</a>
  ·
  <a href="#highlights">Highlights</a>
  ·
  <a href="#features">Features</a>
  ·
  <a href="#build-from-source">Build from source</a>
  ·
  <a href="#packaging-and-ci">Packaging</a>
  ·
  <a href="#project-structure">Structure</a>
  ·
  <a href="#license">License</a>
</p>

<p align="center">
  <a href="https://github.com/Cuteesy/ChessLab/actions/workflows/build-linux.yml"><img src="https://github.com/Cuteesy/ChessLab/actions/workflows/build-linux.yml/badge.svg?branch=main" alt="Linux build status" /></a>
  <img src="https://img.shields.io/badge/license-GPL--3.0--only-blue.svg" alt="GPL-3.0-only" />
  <img src="https://img.shields.io/badge/platform-Linux-0E1A2B.svg" alt="Linux" />
</p>

## Overview

ChessLab is an open-source desktop application for studying chess with a local
engine. Analysis, game databases, repertoires and puzzles share one native
window, and every evaluation comes from Stockfish on your own machine — no
analysis server, no upload, no account required to analyse.

It is a GPL-3.0 fork of [En Croissant](https://github.com/franciscoBSalgueiro/en-croissant)
that adds a coach mode, board-level threat arrows, human-like engine strength
and a floating analysis board, and leaves out upstream's telemetry and
self-updater.

<p align="center">
  <img src="branding/app-screenshot.png" alt="ChessLab home screen: analyse a game, play an engine, build a repertoire, import a game or train puzzles" width="100%" />
</p>

<p align="center"><sub>Home screen of a locally built ChessLab package.</sub></p>

## Highlights

What this fork adds on top of En Croissant:

| | |
|---|---|
| **Coach mode** | Surfaces the best move only for the side you are playing, instead of both |
| **Human-like engine** | Caps Stockfish at `UCI_LimitStrength`, `UCI_Elo 1800` and `Skill Level 15`; switching it off restores full strength |
| **Floating board** | Frameless, always-on-top window mirroring the board; opening it again focuses the existing window |
| **Threat arrows** | Paints the opponent's strongest reply directly on the board |
| **No telemetry** | Upstream's analytics and self-updater are not included; installs are explicit |

## Features

**Analysis**

- Multi-PV Stockfish analysis with per-engine UCI configuration
- Move classification and accuracy reports for reviewed games
- Import a position from an image using the bundled vision model

**Practice**

- Opening explorer and repertoire preparation
- Spaced-repetition puzzle training
- Local game databases with position search, player merge and PGN import/export
- Play against an engine or a friend on the same machine

**Desktop**

- Import games from lichess.org and chess.com
- Native file dialogs, single-instance handling and `chesslab://import?url=...` deep links
- Local sound server, board and piece themes, rebindable keyboard shortcuts

## Build from source

**Requirements**

- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform
- Node.js 22 or newer, with pnpm 12.15.0 through [corepack](https://pnpm.io/installation)
- Rust stable

Debian and Ubuntu also need the packages used by the Linux build:

```bash
sudo apt install build-essential cmake ninja-build pkg-config patchelf file \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev libxdo-dev
```

**Run it**

```bash
git clone https://github.com/Cuteesy/ChessLab
cd ChessLab/app
corepack pnpm install
corepack pnpm tauri dev
```

**Check it** — run these before opening a pull request:

```bash
corepack pnpm exec tsc --noEmit
corepack pnpm test
```

## Packaging and CI

```bash
cd app
corepack pnpm tauri build --bundles deb
```

The package lands in `app/src-tauri/target/release/bundle/deb` and the raw
executable in `app/src-tauri/target/release/chesslab`.

The `build-linux-deb` workflow runs this build on a GitHub-hosted runner for
every push to `main` that touches `app/`, and uploads the `chesslab-linux-deb`
and `chesslab-linux-binary` artifacts:

```bash
gh run download -n chesslab-linux-deb
```

A Stockfish binary is bundled at `stockfish/stockfish` for Linux; ChessLab picks
it up automatically when it is not already on `PATH`.

## Project structure

| Path | Contents |
|---|---|
| `app/` | Desktop application: Tauri 2, React, TypeScript, Rust |
| `chesslab/` | Legacy Python implementation, kept as a behaviour reference |
| `stockfish/` | Bundled Linux engine binary |
| `branding/` | Logo, banner and screenshot sources |
| `install.sh`, `run.sh` | Installer and launcher for the legacy Python build |

## Contributing

Bug reports and pull requests are welcome. Read the
[contributing guide](app/CONTRIBUTING.md) before opening a pull request, and
keep the checks above green.

## Credits

ChessLab builds on [En Croissant](https://github.com/franciscoBSalgueiro/en-croissant)
by Francisco Salgueiro and contributors, and on [Stockfish](https://stockfishchess.org/).
Bundled chess piece sets and board themes ship under their own licenses.

## License

ChessLab is distributed under the GNU General Public License, version 3 only.
See [LICENSE](LICENSE) for the full text.
