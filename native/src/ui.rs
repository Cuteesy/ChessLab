use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Sense, Vec2};
use shakmaty::{
    fen::Fen,
    san::San,
    CastlingMode, Chess, Color, EnPassantMode, File, Move, Position, Rank, Role, Square,
};

use crate::engine::Engine;

const BG: Color32 = Color32::from_rgb(0x0E, 0x1A, 0x2B);
const PANEL: Color32 = Color32::from_rgb(0x15, 0x22, 0x36);
const CARD: Color32 = Color32::from_rgb(0x1B, 0x2B, 0x40);
const LIGHT: Color32 = Color32::from_rgb(0xE8, 0xD9, 0xBF);
const DARK: Color32 = Color32::from_rgb(0x7A, 0x5B, 0x45);
const MINT: Color32 = Color32::from_rgb(0x5E, 0xEA, 0xD4);
const TEXT: Color32 = Color32::from_rgb(0xEA, 0xF7, 0xF3);
const DIM: Color32 = Color32::from_rgb(0x8F, 0xBF, 0xB8);
const WHITE_PIECE: Color32 = Color32::from_rgb(0xF7, 0xF2, 0xE4);
const BLACK_PIECE: Color32 = Color32::from_rgb(0x16, 0x16, 0x18);

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Board,
    Engines,
    About,
}

pub struct ChessLabApp {
    position: Chess,
    history: Vec<Chess>,
    moves: Vec<String>,
    last_move: Option<Move>,
    selected: Option<Square>,
    flipped: bool,
    page: Page,
    engine: Option<Engine>,
    engine_path: String,
    depth: u32,
    analysed_fen: Option<String>,
    fen_input: String,
    status: String,
    styled: bool,
}

impl ChessLabApp {
    pub fn new() -> Self {
        let engine_path = std::env::var("CHESSLAB_ENGINE").unwrap_or_else(|_| "stockfish".to_string());
        let mut app = Self {
            position: Chess::new(),
            history: Vec::new(),
            moves: Vec::new(),
            last_move: None,
            selected: None,
            flipped: false,
            page: Page::Board,
            engine: None,
            engine_path: engine_path.clone(),
            depth: 18,
            analysed_fen: None,
            fen_input: String::new(),
            status: String::from("Ready"),
            styled: false,
        };
        match Engine::spawn(&engine_path) {
            Ok(engine) => {
                app.engine = Some(engine);
                app.status = format!("Engine running: {engine_path}");
                app.analyse_now();
            }
            Err(err) => {
                app.status = format!("No engine at '{engine_path}' ({err}). Set the path under Engines.");
            }
        }
        app
    }

    fn fen(&self) -> String {
        Fen::from_position(&self.position, EnPassantMode::Legal).to_string()
    }

    fn analyse_now(&mut self) {
        let fen = self.fen();
        if let Some(engine) = self.engine.as_mut() {
            if let Err(err) = engine.analyse(&fen, self.depth) {
                self.status = format!("Engine error: {err}");
                return;
            }
            self.analysed_fen = Some(fen);
        }
    }

    fn push_move(&mut self, mv: Move) {
        let san = San::from_move(&self.position, mv).to_string();
        let next = match self.position.clone().play(mv) {
            Ok(next) => next,
            Err(_) => return,
        };
        self.history.push(self.position.clone());
        self.moves.push(san);
        self.position = next;
        self.last_move = Some(mv);
        self.selected = None;
        self.analyse_now();
    }

    fn undo(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.position = previous;
            self.moves.pop();
            self.last_move = None;
            self.selected = None;
            self.analyse_now();
        }
    }

    fn reset(&mut self) {
        self.position = Chess::new();
        self.history.clear();
        self.moves.clear();
        self.last_move = None;
        self.selected = None;
        self.analyse_now();
    }

    fn click_square(&mut self, target: Square) {
        if let Some(from) = self.selected {
            let mut candidates: Vec<Move> = self
                .position
                .legal_moves()
                .into_iter()
                .filter(|mv| move_squares(mv).is_some_and(|(f, t)| f == from && t == target))
                .collect();
            if !candidates.is_empty() {
                candidates.sort_by_key(promotion_rank);
                if let Some(best) = candidates.pop() {
                    self.push_move(best);
                }
                return;
            }
        }
        self.selected = self
            .position
            .board()
            .piece_at(target)
            .filter(|piece| piece.color == self.position.turn())
            .map(|_| target);
    }

    fn load_fen(&mut self) {
        let parsed = Fen::from_ascii(self.fen_input.trim().as_bytes())
            .map_err(|err| err.to_string())
            .and_then(|fen| {
                fen.into_position::<Chess>(CastlingMode::Standard)
                    .map_err(|err| err.to_string())
            });
        match parsed {
            Ok(position) => {
                self.position = position;
                self.history.clear();
                self.moves.clear();
                self.last_move = None;
                self.selected = None;
                self.status = "Position loaded from FEN".to_string();
                self.analyse_now();
            }
            Err(err) => self.status = format!("Invalid FEN: {err}"),
        }
    }

    fn connect_engine(&mut self) {
        self.engine = None;
        match Engine::spawn(&self.engine_path) {
            Ok(engine) => {
                self.status = format!("Engine running: {}", self.engine_path);
                self.engine = Some(engine);
                self.analyse_now();
            }
            Err(err) => self.status = format!("Failed to start engine: {err}"),
        }
    }
}

impl eframe::App for ChessLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.styled {
            ui.ctx().set_visuals(egui::Visuals::dark());
            self.styled = true;
        }
        if let Some(engine) = self.engine.as_mut() {
            engine.poll();
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }

        self.nav(ui);
        self.analysis_panel(ui);

        egui::CentralPanel::no_frame()
            .frame(egui::Frame::new().fill(BG).inner_margin(12))
            .show(ui, |ui| match self.page {
                Page::Board => self.board_page(ui),
                Page::Engines => self.engines_page(ui),
                Page::About => self.about_page(ui),
            });
    }
}

impl ChessLabApp {
    fn nav(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("nav")
            .exact_size(196.0)
            .resizable(false)
            .frame(egui::Frame::new().fill(PANEL).inner_margin(12))
            .show(ui, |ui| {
                ui.add_space(16.0);
                ui.label(egui::RichText::new("ChessLab").size(24.0).strong().color(TEXT));
                ui.label(egui::RichText::new("native build").size(12.0).color(DIM));
                ui.add_space(18.0);
                for (page, label) in [
                    (Page::Board, "Board"),
                    (Page::Engines, "Engines"),
                    (Page::About, "About"),
                ] {
                    let active = self.page == page;
                    let fill = if active { CARD } else { Color32::TRANSPARENT };
                    let button = egui::Button::new(
                        egui::RichText::new(label)
                            .size(15.0)
                            .color(if active { TEXT } else { DIM }),
                    )
                    .fill(fill)
                    .min_size(Vec2::new(160.0, 34.0));
                    if ui.add(button).clicked() {
                        self.page = page;
                    }
                    ui.add_space(4.0);
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new(&self.status).size(11.0).color(DIM));
                    ui.add_space(4.0);
                });
            });
    }

    fn analysis_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::right("analysis")
            .default_size(330.0)
            .min_size(280.0)
            .frame(egui::Frame::new().fill(PANEL).inner_margin(12))
            .show(ui, |ui| {
                ui.add_space(14.0);
                ui.label(egui::RichText::new("Analysis").size(16.0).strong().color(TEXT));
                ui.add_space(8.0);

                let (score, depth, nodes, pv) = match self.engine.as_ref() {
                    Some(engine) => (
                        engine.info.score_text(),
                        engine.info.depth,
                        engine.info.nodes,
                        engine.info.pv.iter().take(6).cloned().collect::<Vec<_>>().join(" "),
                    ),
                    None => ("—".to_string(), 0, 0, String::new()),
                };

                ui.label(egui::RichText::new(score).size(34.0).strong().color(MINT));
                ui.label(
                    egui::RichText::new(format!("depth {depth}   nodes {nodes}"))
                        .size(12.0)
                        .color(DIM),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(if self.position.turn() == Color::White {
                        "White to move"
                    } else {
                        "Black to move"
                    })
                    .size(13.0)
                    .color(TEXT),
                );
                ui.add_space(10.0);
                ui.label(egui::RichText::new(pv).size(12.0).color(DIM).monospace());

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Moves").size(15.0).strong().color(TEXT));
                ui.add_space(6.0);
                egui::ScrollArea::vertical()
                    .max_height(260.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for (i, mv) in self.moves.iter().enumerate() {
                            let prefix = if i % 2 == 0 {
                                format!("{}. ", i / 2 + 1)
                            } else {
                                String::new()
                            };
                            ui.label(
                                egui::RichText::new(format!("{prefix}{mv}")).size(13.0).color(TEXT),
                            );
                        }
                        if self.moves.is_empty() {
                            ui.label(egui::RichText::new("No moves yet").size(13.0).color(DIM));
                        }
                    });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(12.0);
                    ui.add(
                        egui::Slider::new(&mut self.depth, 6..=30).text("Depth"),
                    );
                    if ui.button("Re-analyse position").clicked() {
                        self.analyse_now();
                    }
                    ui.add_space(6.0);
                });
            });
    }

    fn board_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                self.reset();
            }
            if ui.button("Undo").clicked() {
                self.undo();
            }
            if ui.button("Flip board").clicked() {
                self.flipped = !self.flipped;
            }
            ui.add_space(10.0);
            ui.label(egui::RichText::new(self.fen()).size(11.0).color(DIM).monospace());
        });
        ui.add_space(8.0);

        let available = ui.available_size();
        // Leave room for the eval bar, its gap and the control rows below.
        let board_size = (available.x - 34.0).min(available.y - 96.0).clamp(240.0, 760.0);
        ui.horizontal_top(|ui| {
            self.eval_bar(ui, board_size);
            ui.add_space(10.0);
            self.board(ui, board_size);
        });

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.fen_input)
                    .hint_text("Paste a FEN")
                    .desired_width((ui.available_width() - 170.0).max(180.0)),
            );
            if ui.button("Load FEN").clicked() {
                self.load_fen();
            }
            if ui.button("Copy FEN").clicked() {
                self.fen_input = self.fen();
            }
        });
    }

    fn eval_bar(&self, ui: &mut egui::Ui, height: f32) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(18.0, height), Sense::hover());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, egui::CornerRadius::same(4), Color32::from_rgb(0x1B, 0x1B, 0x1D));
        let cp = self
            .engine
            .as_ref()
            .map(|engine| engine.info.eval_cp())
            .unwrap_or(0.0);
        let fraction = ((cp + 900.0) / 1800.0).clamp(0.0, 1.0);
        let white_height = rect.height() * fraction;
        let white_rect = Rect::from_min_max(
            Pos2::new(rect.left(), rect.bottom() - white_height),
            rect.right_bottom(),
        );
        painter.rect_filled(white_rect, egui::CornerRadius::same(4), WHITE_PIECE);
    }

    fn board(&mut self, ui: &mut egui::Ui, size: f32) {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
        let painter = ui.painter_at(rect);
        let cell = size / 8.0;
        let flipped = self.flipped;

        let legal_targets: Vec<Square> = match self.selected {
            Some(from) => self
                .position
                .legal_moves()
                .into_iter()
                .filter_map(|mv| move_squares(&mv))
                .filter(|(f, _)| *f == from)
                .map(|(_, t)| t)
                .collect(),
            None => Vec::new(),
        };

        for rank in 0..8u32 {
            for file in 0..8u32 {
                let square = Square::from_coords(File::new(file), Rank::new(rank));
                let square_rect = square_rect(rect, cell, square, flipped);
                let base = if (file + rank) % 2 == 0 { DARK } else { LIGHT };
                painter.rect_filled(square_rect, egui::CornerRadius::ZERO, base);

                if Some(square) == self.selected {
                    painter.rect_filled(
                        square_rect,
                        egui::CornerRadius::ZERO,
                        Color32::from_rgba_unmultiplied(0x5E, 0xEA, 0xD4, 96),
                    );
                }
                if let Some(mv) = self.last_move {
                    if let Some((from, to)) = move_squares(&mv) {
                        if square == from || square == to {
                            painter.rect_filled(
                                square_rect,
                                egui::CornerRadius::ZERO,
                                Color32::from_rgba_unmultiplied(0xEA, 0xD9, 0x8B, 60),
                            );
                        }
                    }
                }

                if let Some(piece) = self.position.board().piece_at(square) {
                    let glyph = glyph(piece.color, piece.role);
                    let color = if piece.color == Color::White {
                        WHITE_PIECE
                    } else {
                        BLACK_PIECE
                    };
                    let center = square_rect.center();
                    painter.text(
                        center + Vec2::new(1.0, 1.5),
                        Align2::CENTER_CENTER,
                        glyph,
                        FontId::proportional(cell * 0.78),
                        Color32::from_black_alpha(70),
                    );
                    painter.text(
                        center,
                        Align2::CENTER_CENTER,
                        glyph,
                        FontId::proportional(cell * 0.78),
                        color,
                    );
                }

                if legal_targets.contains(&square) {
                    painter.circle_filled(
                        square_rect.center(),
                        if self.position.board().piece_at(square).is_some() {
                            cell * 0.42
                        } else {
                            cell * 0.14
                        },
                        Color32::from_rgba_unmultiplied(0x5E, 0xEA, 0xD4, 120),
                    );
                    if self.position.board().piece_at(square).is_some() {
                        painter.circle_stroke(
                            square_rect.center(),
                            cell * 0.42,
                            egui::Stroke::new(3.0, Color32::from_rgba_unmultiplied(0x5E, 0xEA, 0xD4, 150)),
                        );
                    }
                }
            }
        }

        if response.clicked() {
            if let Some(pointer) = response.interact_pointer_pos() {
                if let Some(square) = square_at(rect, cell, pointer, flipped) {
                    self.click_square(square);
                }
            }
        }
    }

    fn engines_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(18.0);
        ui.heading(egui::RichText::new("Engines").color(TEXT));
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(
                "ChessLab talks to any UCI engine. The bundled Stockfish binary lives at stockfish/stockfish in the repository.",
            )
            .size(13.0)
            .color(DIM),
        );
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.engine_path)
                    .hint_text("/path/to/stockfish")
                    .desired_width(440.0),
            );
            if ui.button("Start engine").clicked() {
                self.connect_engine();
            }
            if ui.button("Stop engine").clicked() {
                self.engine = None;
                self.status = "Engine stopped".to_string();
            }
        });
        ui.add_space(12.0);
        let state = if self.engine.is_some() { "running" } else { "not running" };
        ui.label(egui::RichText::new(format!("Status: {state}")).size(13.0).color(TEXT));
        ui.label(egui::RichText::new(&self.status).size(12.0).color(DIM));
    }

    fn about_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(18.0);
        ui.heading(egui::RichText::new("ChessLab Native").color(TEXT));
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(concat!("Version ", env!("CARGO_PKG_VERSION")))
                .size(13.0)
                .color(DIM),
        );
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(
                "A native Rust interface for ChessLab: egui for rendering and input, Stockfish over UCI for analysis.",
            )
            .size(13.0)
            .color(TEXT),
        );
        ui.add_space(10.0);
        ui.label(egui::RichText::new("GPL-3.0-only. Built on En Croissant and Stockfish.").size(12.0).color(DIM));
    }
}

fn square_rect(board: Rect, cell: f32, square: Square, flipped: bool) -> Rect {
    let file = u32::from(square.file());
    let rank = u32::from(square.rank());
    let (col, row) = if flipped {
        (7 - file, rank)
    } else {
        (file, 7 - rank)
    };
    let min = Pos2::new(
        board.left() + col as f32 * cell,
        board.top() + row as f32 * cell,
    );
    Rect::from_min_size(min, Vec2::splat(cell))
}

fn square_at(board: Rect, cell: f32, pos: Pos2, flipped: bool) -> Option<Square> {
    if !board.contains(pos) {
        return None;
    }
    let col = ((pos.x - board.left()) / cell).floor() as i64;
    let row = ((pos.y - board.top()) / cell).floor() as i64;
    if !(0..8).contains(&col) || !(0..8).contains(&row) {
        return None;
    }
    let (file, rank) = if flipped {
        (7 - col as u32, row as u32)
    } else {
        (col as u32, 7 - row as u32)
    };
    Some(Square::from_coords(File::new(file), Rank::new(rank)))
}

fn move_squares(mv: &Move) -> Option<(Square, Square)> {
    match *mv {
        Move::Normal { from, to, .. } => Some((from, to)),
        Move::EnPassant { from, to } => Some((from, to)),
        Move::Castle { king, rook } => {
            let to = if u32::from(rook.file()) > u32::from(king.file()) {
                Square::from_coords(File::G, king.rank())
            } else {
                Square::from_coords(File::C, king.rank())
            };
            Some((king, to))
        }
        Move::Put { to, .. } => Some((to, to)),
    }
}

fn promotion_rank(mv: &Move) -> u8 {
    match *mv {
        Move::Normal {
            promotion: Some(role),
            ..
        } => match role {
            Role::Queen => 4,
            Role::Rook => 3,
            Role::Bishop => 2,
            Role::Knight => 1,
            _ => 0,
        },
        _ => 0,
    }
}

fn glyph(_color: Color, role: Role) -> &'static str {
    match role {
        Role::King => "♚",
        Role::Queen => "♛",
        Role::Rook => "♜",
        Role::Bishop => "♝",
        Role::Knight => "♞",
        Role::Pawn => "♟",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_position_square_mapping_wsquares() {
        let board = Rect::from_min_size(Pos2::ZERO, Vec2::splat(800.0));
        let a8 = Square::from_coords(File::A, Rank::new(7));
        let rect = square_rect(board, 100.0, a8, false);
        assert_eq!(rect.min, Pos2::new(0.0, 0.0));
        let back = square_at(board, 100.0, Pos2::new(50.0, 50.0), false);
        assert_eq!(back, Some(a8));
    }

    #[test]
    fn flip_inverts_mapping() {
        let board = Rect::from_min_size(Pos2::ZERO, Vec2::splat(800.0));
        let a1 = Square::from_coords(File::A, Rank::new(0));
        assert_eq!(
            square_at(board, 100.0, Pos2::new(50.0, 50.0), true),
            Some(Square::from_coords(File::H, Rank::new(0)))
        );
        assert_eq!(
            square_rect(board, 100.0, a1, true).min,
            Pos2::new(700.0, 0.0)
        );
        let _ = color_name(Color::White);
    }

    fn color_name(color: Color) -> &'static str {
        if color == Color::White { "white" } else { "black" }
    }
}
