use std::cmp::PartialEq;
use std::collections::HashMap;

pub const MAX: usize = 20;
pub const DISTANCE: usize = 3;

pub const WIN: i16 = 10;
pub const LOSE: i16 = -10;
pub const DRAW: i16 = 0;
pub const UNKNOWN: i16 = std::i16::MAX;

#[derive(Default, Eq, PartialEq, Hash, Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
/// Points in Game
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Information of Player
#[derive(Default, Debug, Clone)]
pub struct Player {
    pub side: u8,
    pub point_dic: Vec<Point>,
    pub total_score: i32,
}

impl Player {
    // Create new Player
    pub fn new(side: u8) -> Self {
        Self {
            side,
            point_dic: Vec::new(),
            total_score: 0,
        }
    }

    /// Check and give points for a point
    ///
    fn check_line(
        &mut self,
        point: &Point,
        matrix: &[[u8; MAX]; MAX],
        test_point: &mut f32,
    ) -> bool {
        // rubric for AI to evaluate a point
        let compound_samples: Vec<(i16, Vec<String>, f32)> = vec![
            (2, vec!["_XX#XO".to_string(), "_XX#XO".to_string()], 200.0),
            (2, vec!["_XX#XO".to_string(), "_X#XXO".to_string()], 200.0),
            (2, vec!["_XXX#O".to_string(), "_X#XXO".to_string()], 200.0),
            (2, vec!["_#XXXO".to_string(), "_#XXXO".to_string()], 200.0),
            (2, vec!["_#XXXO".to_string(), "_#X_X_".to_string()], 150.0),
            (2, vec!["_#XXXO".to_string(), "_#XX_".to_string()], 150.0),
            (2, vec!["_X#XXO".to_string(), "_#XX_".to_string()], 150.0),
            (1, vec!["_#X_X_".to_string(), "#_XX_".to_string()], 100.0),
            (3, vec!["_#XX_".to_string(), "_X#X_".to_string()], 100.0),
            (3, vec!["_X#X_".to_string(), "_X#X_".to_string()], 100.0),
            (4, vec!["_X#XXO".to_string(), "_X#X_".to_string()], 100.0),
            (5, vec!["#XXX_O".to_string()], 100.0),
            (6, vec!["#XX_XO".to_string()], 100.0),
            (7, vec!["#X_XXO".to_string()], 100.0),
            (8, vec!["_X#X_".to_string(), "_X#X_".to_string()], 100.0),
            (9, vec!["_#XX_".to_string(), "_#XX_".to_string()], 100.0),
            (10, vec!["#XXX_".to_string()], 100.0),
            (11, vec!["_X#XX_".to_string()], 100.0),
            (12, vec!["_X#X_".to_string()], 50.0),
            (12, vec!["_X_#_X_".to_string()], 50.0),
            (12, vec!["_XX_#_".to_string()], 50.0),
            (13, vec!["_#XX_".to_string()], 50.0),
            (14, vec!["_#XXXO".to_string()], 10.0),
            (15, vec!["_#X__".to_string()], 5.0),
            (8, vec!["#X_".to_string()], 1.0),
        ];

        // check for winning
        let winning_samples: Vec<(u16, String, f32)> = vec![
            (1, "#XXXX".to_string(), 1000.0),
            (2, "X#XXX".to_string(), 1000.0),
            (3, "XX#XX".to_string(), 1000.0),
        ];

        // move back c points from the current point on a direction
        fn back(point: &Point, c: usize, direction: u8) -> Result<Point, bool> {
            match direction {
                //Horizon
                1 => {
                    if point.y as i16 - c as i16 >= 0 {
                        return Ok(Point::new(point.x, point.y - c));
                    }
                    Err(false)
                }
                //Vertical ,
                2 => {
                    if point.x as i16 - c as i16 >= 0 {
                        return Ok(Point::new(point.x - c, point.y));
                    }
                    Err(false)
                }
                //Cross down
                3 => {
                    if point.x as i16 - c as i16 >= 0 && point.y as i16 - c as i16 >= 0 {
                        return Ok(Point::new(point.x - c, point.y - c));
                    }
                    Err(false)
                }
                //Cross up
                4 => {
                    if point.x + c < MAX && point.y as i16 - c as i16 >= 0 {
                        return Ok(Point::new(point.x + c, point.y - c));
                    }
                    Err(false)
                }
                _ => Err(false),
            }
        }

        // move forward c points from the current point on a direction
        fn forward(point: &Point, c: usize, direction: u8) -> Result<Point, bool> {
            match direction {
                //Horizon
                1 => {
                    if point.y + c < MAX {
                        return Ok(Point::new(point.x, point.y + c));
                    }
                    Err(false)
                }
                //Vertical,
                2 => {
                    if point.x + c < MAX {
                        return Ok(Point::new(point.x + c, point.y));
                    }
                    Err(false)
                }
                //Cross down
                3 => {
                    if point.x + c < MAX && point.y + c < MAX {
                        return Ok(Point::new(point.x + c, point.y + c));
                    }
                    Err(false)
                }
                //Cross up
                4 => {
                    if point.x as i16 - c as i16 >= 0 && point.y + c < MAX {
                        return Ok(Point::new(point.x - c, point.y + c));
                    }
                    Err(false)
                }
                _ => Err(false),
            }
        }

        // find the position of the point in a sample
        fn find_pos(sample: &str) -> i8 {
            let sample_chars = sample.chars();
            for (i, ch) in sample_chars.enumerate() {
                if ch == '#' {
                    return i as i8;
                }
            }
            -1
        }

        //check whether the sample matches
        fn is_match(
            direction: u8,
            point: &Point,
            side1: u8,
            side2: u8,
            pos: i8,
            sample: &str,
            matrix: &[[u8; MAX]; MAX],
        ) -> bool {
            let r_starting_point = back(point, pos as usize, direction);
            if r_starting_point.is_err() {
                return false;
            }
            let starting_point = r_starting_point.unwrap();
            for c in 0..sample.len() {
                if sample.chars().nth(c).unwrap() == '?' {
                    continue;
                }
                let r_next_point = forward(&starting_point, c, direction);
                if r_next_point.is_err() {
                    return false;
                }
                let next_point = r_next_point.unwrap();
                let ch: char;

                if matrix[next_point.x][next_point.y] == 0 {
                    if c != (pos as usize) {
                        ch = '_';
                    } else {
                        ch = '#';
                    }
                } else if matrix[next_point.x][next_point.y] == side1 {
                    ch = 'X';
                } else if matrix[next_point.x][next_point.y] == side2 {
                    ch = 'O';
                } else {
                    ch = ' ';
                };

                if ch != sample.chars().nth(c).unwrap() {
                    return false;
                }
            }

            true
        }

        if *test_point > 0.0 {
            let mut total_score = 0.0;

            for (_, c_sample, score) in compound_samples {
                let mut found_dir: HashMap<usize, i32> = HashMap::new();
                for sample in &c_sample {
                    let mut exit_flag = false;
                    for d in 1..=4 {
                        if !found_dir.contains_key(&d) {
                            let sample_rev = sample.chars().rev().collect::<String>();
                            let mut collection: Vec<String> = Vec::new();
                            if sample_rev != *sample {
                                collection.push(sample_rev);
                            }
                            collection.push(sample.clone());

                            for test_sample in collection {
                                let pos_result = find_pos(&test_sample);

                                if pos_result >= 0
                                    && is_match(
                                        d as u8,
                                        point,
                                        self.side,
                                        self.opposite_side(),
                                        pos_result,
                                        &test_sample,
                                        matrix,
                                    )
                                {
                                    found_dir.entry(d).or_insert(0);
                                    if c_sample.len() == 1
                                        || (found_dir.keys().len() == c_sample.len())
                                    {
                                        total_score += score;
                                    }
                                    exit_flag = true;
                                    break;
                                }
                            }
                        }
                        if exit_flag && c_sample.len() > 1 {
                            break;
                        }
                    }
                    if found_dir.is_empty() {
                        break;
                    }
                }
                if found_dir.keys().len() == c_sample.len() {
                    break;
                }
            }
            *test_point = total_score;
        }

        for (_, sample, _) in winning_samples {
            let sample_rev = sample.chars().rev().collect::<String>();
            let mut collection: Vec<String> = Vec::new();

            if sample_rev != sample {
                collection.push(sample_rev);
            }
            collection.push(sample);
            for d in 1..=4 {
                for test_sample in &collection {
                    let pos_result = find_pos(&test_sample);
                    // println!("pos = {}",pos_result);
                    if pos_result >= 0
                        && is_match(
                            d,
                            point,
                            self.side,
                            self.opposite_side(),
                            pos_result,
                            &test_sample,
                            matrix,
                        )
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Add new point and evaluate the heuristic points of a new move
    /// # Return: true if winning and false if not winning
    pub fn add_new_point(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test_point: &mut f32,
    ) -> bool {
        let mut yes_test = false;
        if *test_point > 0.0 {
            yes_test = true;
        }

        let origin_test_point = *test_point;

        if self.check_line(&point, matrix, test_point) {
            if !yes_test {
                matrix[point.x][point.y] = self.side;
                self.point_dic.push(point);
                *test_point = origin_test_point;
            } else if *test_point == 0.0 {
                *test_point = origin_test_point;
            }
            return true;
        }
        if yes_test {
            if *test_point == 0.0 {
                *test_point = origin_test_point;
            }
        } else {
            *test_point = origin_test_point;
            matrix[point.x][point.y] = self.side;
            self.point_dic.push(point);
        }
        false
    }

    fn opposite_side(&self) -> u8 {
        if self.side == 1 {
            2
        } else {
            1
        }
    }
}

/// Find the best move (AI)
pub fn find_best_move(
    mut ai: Player,
    mut user: Player,
    mut matrix: [[u8; MAX]; MAX],
) -> Option<Point> {
    let mut hash: HashMap<Point, i32> = HashMap::new();
    get_all_board_move(&mut hash, ai.point_dic.clone(), &matrix);
    get_all_board_move(&mut hash, user.point_dic.clone(), &matrix);
    let new_board = hash.keys().cloned().collect::<Vec<Point>>();
    let mut final_board: Vec<(Point, f32)> = Vec::new();
    for point in new_board {
        let mut test_for_ai = 0.0001;
        let mut test_for_user = 0.0001;

        if ai.add_new_point(point.clone(), &mut matrix, &mut test_for_ai) {
            test_for_ai = 1000.0;
        }

        if user.add_new_point(point.clone(), &mut matrix, &mut test_for_user) {
            test_for_user = 1000.0;
        }
        if test_for_ai < 200.0 {
            final_board.push((point, test_for_ai + test_for_user));
        } else {
            final_board.push((point, 200.0 + test_for_ai + test_for_user));
        }
    }
    final_board.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    final_board.reverse();
    // println!("###Considering points:");
    // for (point, score) in &final_board {
    //     print!("({},{})={} ", point.x, point.y, score);
    // }
    // println!();

    if !final_board.is_empty() {
        let (result, _) = &final_board[0];
        return Some(result.clone());
    }
    None
}

/// Get all possible moves
pub fn get_all_board_move(
    result: &mut HashMap<Point, i32>,
    local_point: Vec<Point>,
    matrix: &[[u8; MAX]; MAX],
) {
    for point in local_point {
        //Horizon
        for c in 1..DISTANCE {
            if point.y + c == MAX {
                break;
            }
            if matrix[point.x][point.y + c] == 0 {
                result.entry(Point::new(point.x, point.y + c)).or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x][point.y - c] == 0 {
                result.entry(Point::new(point.x, point.y - c)).or_insert(0);
            } else {
                break;
            }
        }
        //Vertical
        for c in 1..DISTANCE {
            if point.x + c == MAX {
                break;
            }
            if matrix[point.x + c][point.y] == 0 {
                result.entry(Point::new(point.x + c, point.y)).or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.x as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x - c][point.y] == 0 {
                result.entry(Point::new(point.x - c, point.y)).or_insert(0);
            } else {
                break;
            }
        }
        //Cross down
        for c in 1..DISTANCE {
            if point.x + c == MAX || point.y + c == MAX {
                break;
            }
            if matrix[point.x + c][point.y + c] == 0 {
                result
                    .entry(Point::new(point.x + c, point.y + c))
                    .or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.x as i16 - c as i16 == -1 || point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x - c][point.y - c] == 0 {
                result
                    .entry(Point::new(point.x - c, point.y - c))
                    .or_insert(0);
            } else {
                break;
            }
        }
        //Cross up
        for c in 1..DISTANCE {
            if point.x as i16 - c as i16 == -1 || point.y + c == MAX {
                break;
            }
            if matrix[point.x - c][point.y + c] == 0 {
                result
                    .entry(Point::new(point.x - c, point.y + c))
                    .or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.x + c == MAX || point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x + c][point.y - c] == 0 {
                result
                    .entry(Point::new(point.x + c, point.y - c))
                    .or_insert(0);
            } else {
                break;
            }
        }
    }
}
