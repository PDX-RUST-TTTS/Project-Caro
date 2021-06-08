use gomoku_ai::{find_best_move, Player, Point, MAX};

use iced::{button, Button, Color, Column, Element, Length, Radio, Row, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    GomukuUI::run(Settings::default())
}

#[derive(Default)]
struct GomukuUI {
    text_color: [[Color; MAX]; MAX],
    text: [[String; MAX]; MAX],
    btn: [[button::State; MAX]; MAX],
    new_game: button::State,
    exit: button::State,
    selected_choice: Option<Choice>,
    ai: Player,
    player1: Player,
    matrix: [[u8; MAX]; MAX],
    turn: i32,
    information: String,
    game_state: Option<GameState>,
    last_point: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Player,
    AI,
}

pub enum GameState {
    Running,
    GameEnding,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Check(usize, usize),
    RadioSelected(Choice),
    NewGame,
    ExitGame,
}
