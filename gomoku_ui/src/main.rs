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

impl Sandbox for GomukuUI {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rust Gomoku")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Check(x, y) => {
                if let Some(GameState::Running) = self.game_state {
                    let choice = self.selected_choice.unwrap();
                    let mut test = 0.0;
                    if self.matrix[x][y] == 0 {
                        if self.turn == self.player1.side as i32 {
                            if self.player1.add_new_point(
                                Point::new(x, y),
                                &mut self.matrix,
                                &mut test,
                            ) {
                                if choice == Choice::AI {
                                    self.information = "You won !".to_string();
                                } else {
                                    self.information = "Player 1 won !".to_string();
                                }
                                self.text_color[self.last_point.x][self.last_point.y] =
                                    Color::from_rgb8(0, 191, 255);
                                self.text[x][y] = " X".to_string();
                                self.text_color[x][y] = Color::from_rgb8(0, 100, 0);
                                self.last_point = Point::new(x, y);
                                self.game_state = Some(GameState::GameEnding);
                                return;
                            }
                            self.text_color[self.last_point.x][self.last_point.y] =
                                Color::from_rgb8(0, 191, 255);
                            self.text[x][y] = " X".to_string();
                            // self.text_color[x][y] = Color::from_rgb8(220, 20, 60);
                            self.text_color[x][y] = Color::from_rgb8(0, 100, 0);
                            self.last_point = Point::new(x, y);
                            self.turn = self.ai.side as i32;
                            if choice == Choice::Player {
                                self.information = format!(
                                    "Player 1's move is ({},{}\n now is the turn of player 2",
                                    x, y
                                );
                                return;
                            }
                        }
                        let find_result;
                        if choice == Choice::Player {
                            find_result = Some(Point::new(x, y));
                        } else {
                            self.information =
                                format!("Your move is ({},{}\n now is AI's turn", x, y);
                            self.information = "AI is thinking".to_string();
                            // thread::sleep( time::Duration::from_millis(1000));
                            find_result = find_best_move(
                                self.ai.clone(),
                                self.player1.clone(),
                                self.matrix.clone(),
                            );
                        }

                        if self.player1.point_dic.len() + self.ai.point_dic.len() == MAX * MAX {
                            self.information = "Draw !".to_string();
                            self.game_state = Some(GameState::GameEnding);
                            return;
                        }

                        match find_result {
                            None => {
                                self.information = "Draw !".to_string();
                            }
                            Some(ai_move) => {
                                self.text_color[self.last_point.x][self.last_point.y] =
                                    Color::from_rgb8(255, 0, 0);
                                self.text[ai_move.x][ai_move.y] = " O".to_string();
                                // self.text_color[ai_move.x][ai_move.y] = Color::from_rgb8(0, 0, 255);
                                self.text_color[ai_move.x][ai_move.y] = Color::from_rgb8(0, 100, 0);
                                self.last_point = ai_move.clone();

                                if self.ai.add_new_point(
                                    ai_move.clone(),
                                    &mut self.matrix,
                                    &mut test,
                                ) {
                                    if choice == Choice::Player {
                                        self.information = "Player 2 won !".to_string();
                                    } else {
                                        self.information = "AI won !".to_string();
                                    }

                                    self.game_state = Some(GameState::GameEnding);
                                    return;
                                }
                                self.turn = self.player1.side as i32;
                                if choice == Choice::Player {
                                    self.information = format!(
                                        "Player 2's move is ({},{}\n now is the turn of player 1",
                                        x, y
                                    );
                                } else {
                                    self.information = format!(
                                        "AI's move is ({},{})\n now is your turn",
                                        ai_move.x, ai_move.y
                                    );
                                }
                            }
                        }
                    }
                } else {
                    self.information = "Please start new game to play".to_string();
                }
            }