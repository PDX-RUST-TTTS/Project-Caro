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

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}



#[derive(Default, Debug, Clone)]
pub struct Player {
    pub side: u8,
    pub point_dic: Vec<Point>,
    pub total_score: i32,
}


impl Player { 

   pub fn new(side: u8) -> Self {
        Self {
            side,
            point_dic: Vec::new(),
            total_score: 0,
        }
    }

    fn check_line(
        &mut self,
        point: &Point,
        matrix: &[[u8; MAX]; MAX],
        test_point: &mut f32,
    ) -> bool {
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
        let winning_samples: Vec<(u16, String, f32)> = vec![
            (1, "#XXXX".to_string(), 1000.0),
            (2, "X#XXX".to_string(), 1000.0),
            (3, "XX#XX".to_string(), 1000.0),
        ];

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

        fn find_pos(sample: &str) -> i8 {
            let sample_chars = sample.chars();
            for (i, ch) in sample_chars.enumerate() {
                if ch == '#' {
                    return i as i8;
                }
            }
            -1
        }
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

    fn opposite_side(&self) -> u8 {
        if self.side == 1 {
            2
        } else {
            1
        }
    }

}