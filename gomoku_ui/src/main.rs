use std::process;

//gomoku_ai crate
use gomoku_ai::{find_best_move, Player, Point, MAX};

//iced crate
use iced::{button, Button, Color, Column, Element, Length, Radio, Row, Sandbox, Settings, Text};

//main function
pub fn main() -> iced::Result {
    GomukuUI::run(Settings::default())
}

//GomukuUI struct
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

//The option for player, who do you want to play against
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Player,
    AI,
}

//The state of caro game is running or ending
pub enum GameState {
    Running,
    GameEnding,
}

//Prompt the message for the user cansee then they can make the choice
#[derive(Debug, Clone, Copy)]
enum Message {
    Check(usize, usize),
    RadioSelected(Choice),
    NewGame,
    ExitGame,
}
<<<<<<< Updated upstream

//Impliment Sandbox for GomukuUI
=======
// Adding visual messages for the User interface
>>>>>>> Stashed changes
impl Sandbox for GomukuUI {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }
    // Adding title to display for the users to see it
    fn title(&self) -> String { 
        String::from("Rust Gomoku")
    }
    // Checking the Game's current state and Updating it
    // X is the X coordinates and Y is the Y Coordinates in the 19 x 19 grid MAP
    // Adding different color to 'X' and 'O' depending on the Turn for variation
    fn update(&mut self, message: Message) {
        match message {
            Message::Check(x, y) => { 
                if let Some(GameState::Running) = self.game_state { // Checking game state
                    let choice = self.selected_choice.unwrap();
                    let mut test = 0.0;
                    if self.matrix[x][y] == 0 {
                        if self.turn == self.player1.side as i32 {
                            if self.player1.add_new_point(
                                Point::new(x, y),
                                &mut self.matrix,
                                &mut test,
                            ) {
                                // Displaying Winning Message for P Vs AI
                                if choice == Choice::AI {
                                    self.information = "You won !".to_string();
                                } else {
                                    // Displaying Winning Message for P Vs P
                                    self.information = "Player 1 won !".to_string();
                                }
                                // Adding Graphics to the Player1's Move
                                self.text_color[self.last_point.x][self.last_point.y] =
                                    Color::from_rgb8(0, 191, 255);
                                self.text[x][y] = " X".to_string();
                                self.text_color[x][y] = Color::from_rgb8(0, 100, 0);
                                self.last_point = Point::new(x, y);
                                self.game_state = Some(GameState::GameEnding);
                                return;
                            }
                            // Adding Graphics to the Player1's Move
                            self.text_color[self.last_point.x][self.last_point.y] =
                                Color::from_rgb8(0, 191, 255);
                            self.text[x][y] = " X".to_string();
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
                            
                            find_result =
                                find_best_move(self.ai.clone(), self.player1.clone(), self.matrix);
                        }
                         // If there are no more spaces for the players to play then it is DRAW         
                        if self.player1.point_dic.len() + self.ai.point_dic.len() == MAX * MAX {
                            self.information = "Draw !".to_string();
                            self.game_state = Some(GameState::GameEnding);
                            return;
                        }
                        // If there are no more spaces for the players to play then it is DRAW
                        match find_result {
                            None => {
                                self.information = "Draw !".to_string();
                            }
                            // Adding graphics to the Ai's Move
                            Some(ai_move) => {
                                self.text_color[self.last_point.x][self.last_point.y] =
                                    Color::from_rgb8(255, 0, 0);
                                self.text[ai_move.x][ai_move.y] = " O".to_string();
                                self.text_color[ai_move.x][ai_move.y] = Color::from_rgb8(0, 100, 0);
                                self.last_point = ai_move.clone();

                                if self.ai.add_new_point(
                                    ai_move.clone(),
                                    &mut self.matrix,
                                    &mut test,
                                ) {
                                    if choice == Choice::Player {
                                        // Displaying Winning Message for P Vs P
                                        self.information = "Player 2 won !".to_string();
                                    } else {
                                        // Displaying Winning Message for P Vs AI
                                        self.information = "AI won !".to_string();
                                    }
                                    self.game_state = Some(GameState::GameEnding);
                                    return;
                                }
                                self.turn = self.player1.side as i32;
                                if choice == Choice::Player {
                                    self.information = format!(
                                        // Displaying the Grid of the players move
                                        "Player 2's move is ({},{}\n now is the turn of player 1",
                                        x, y
                                    );
                                } else {
                                    self.information = format!(
                                        // Displaying the Grid of the Ai's move
                                        "AI's move is ({},{})\n now is your turn",
                                        ai_move.x, ai_move.y
                                    );
                                }
                            }
                        }
                    }
                } else {
                    // Message after the Game Ends
                    self.information = "Please start new game to play".to_string();
                }
            }
            // Choice Given to the Players
            Message::RadioSelected(choice) => {
                self.selected_choice = Some(choice);
            }
            // Creating and Starting a New Game
            Message::NewGame => {
                if self.selected_choice.is_some() {
                    self.matrix = [[0; MAX]; MAX];
                    self.player1 = Player::new(2);
                    self.ai = Player::new(1);
                    self.turn = self.player1.side as i32;
                    for i in 0..MAX {
                        for j in 0..MAX {
                            self.text[i][j] = " ".to_string();
                        }
                    }
                    // Message displaying after the Game Starts and updating the Game State
                    self.game_state = Some(GameState::Running);
                    self.information = "Ready to play!".to_string();
                } else {
                    // Validation message for user if they dont select either P V Ai or P V P 
                    self.information = "Please select who \n  you want to play with !".to_string();
                }
            }
            // Exiting the game
            Message::ExitGame => {
                process::exit(0x0);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut iter = self.btn.iter_mut().flat_map(|r| r.iter_mut());
        let mut row_main: Row<Message> = Row::new();

        let mut col_play_area: Column<Message> = Column::new();
        let mut row: Row<Message> = Row::new();
        row = row.push(
            Column::new()
                .push(Row::new().push(Text::new("")).padding(2))
                .height(Length::Units(35))
                .width(Length::Units(35)),
        );
        for i in 0..MAX {
            row = row.push(
                Column::new()
                    .push(Row::new().push(Text::new(i.to_string())).padding(2))
                    .height(Length::Units(35))
                    .width(Length::Units(35))
                    .padding(5),
            );
        }
        col_play_area = col_play_area.push(row);
        for i in 0..MAX {
            let mut row: Row<Message> = Row::new();
            row = row.push(
                Column::new()
                    .push(Row::new().push(Text::new(i.to_string())).padding(2))
                    .height(Length::Units(35))
                    .width(Length::Units(35)),
            );
            for j in 0..MAX {
                row = row.push(
                    Button::new(
                        iter.next().unwrap(),
                        Text::new(&self.text[i][j]).color(self.text_color[i][j]),
                    )
                    .height(Length::Units(35))
                    .width(Length::Units(35))
                    .on_press(Message::Check(i, j)),
                );
            }
            col_play_area = col_play_area.push(row);
        }
        let mut col_control_area: Column<Message> = Column::new();
        let col_padding: Column<Message> = Column::new().width(Length::Units(35));

        col_control_area = col_control_area
            .push(Row::new().height(Length::Units(50)))
            .push(
                Text::new("Rust Gomoku")
                    .color(Color::from_rgb8(87, 78, 206))
                    .size(50),
            )
            .push(Row::new().height(Length::Units(100)))
            .push(
                Row::new().push(
                    Column::new()
                        .push(Radio::new(
                            Choice::AI,
                            "Player vs AI",
                            self.selected_choice,
                            Message::RadioSelected,
                        ))
                        .push(Row::new().height(Length::Units(20)))
                        .push(Radio::new(
                            Choice::Player,
                            "Player vs Player",
                            self.selected_choice,
                            Message::RadioSelected,
                        )),
                ),
            )
            .push(Row::new().height(Length::Units(20)))
            .push(
                Row::new().push(
                    Column::new()
                        .push(
                            Button::new(&mut self.new_game, Text::new("New Game"))
                                .height(Length::Units(35))
                                .width(Length::Units(200))
                                .on_press(Message::NewGame),
                        )
                        .push(Row::new().height(Length::Units(20)))
                        .push(
                            Button::new(&mut self.exit, Text::new("Exit"))
                                .height(Length::Units(35))
                                .width(Length::Units(200))
                                .on_press(Message::ExitGame),
                        ),
                ),
            )
            .push(Row::new().height(Length::Units(20)))
            .push(Text::new(&self.information));

        row_main = row_main.push(col_play_area);
        row_main = row_main.push(col_padding);
        row_main = row_main.push(col_control_area);
        row_main.into()
    }
}
