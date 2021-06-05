use gomoku_ai::{find_best_move, Player, Point, MAX};

#[test]
fn test_move() {
    fn string_to_point(s: String) -> Point {
        let strings: Vec<&str> = s.trim().split(",").collect();
        //  println!("strings {:?}", strings);
        let x = strings[0].parse::<u32>().unwrap();
        let y = strings[1].parse::<u32>().unwrap();
        Point::new(x as usize, y as usize)
    }
    fn print_matrix(matrix: &[[u8; MAX]; MAX]) {
        print!("   ");
        for i in 0..MAX {
            if i > 9 {
                print!("{} ", i);
            } else {
                print!("{}  ", i);
            }
        }
        println!("");
        for i in 0..MAX {
            if i > 9 {
                print!("{} ", i);
            } else {
                print!("{}  ", i);
            }
            for j in 0..MAX {
                if matrix[i][j] == 1 {
                    print!("{}  ", 'X');
                } else if matrix[i][j] == 2 {
                    print!("{}  ", 'O');
                } else {
                    print!("{}  ", ' ');
                }
            }
            println!("");
        }
    }

    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player = Player::new(2);
    let mut ai = Player::new(1);
    let mut test = 0.0;
    print_matrix(&matrix);
    loop {
        let mut line = String::new();
        println!("Please enter your move 'x,y':");
        let _input = std::io::stdin().read_line(&mut line).unwrap();
        let point = string_to_point(line);
        println!("You go {:?}", point);
        if matrix[point.x][point.y] != 0 {
            println!("The point you enter already existed!");
            continue;
        }
        if player.add_new_point(point, &mut matrix, &mut test) {
            print_matrix(&matrix);
            println!("You win");
            return;
        }
        if player.point_dic.len() + ai.point_dic.len() == MAX * MAX {
            println!("Draw!");
            return;
        }
        print_matrix(&matrix);
        println!("AI is thinking ...");
        let find_result = find_best_move(ai.clone(), player.clone(), matrix.clone());
        match find_result {
            None => {
                println!("Draw!");
                return;
            }
            Some(ai_move) => {
                println!("AI goes {},{}", ai_move.x, ai_move.y);

                if ai.add_new_point(ai_move, &mut matrix, &mut test) {
                    print_matrix(&matrix);
                    println!("AI win");
                    return;
                }
                print_matrix(&matrix);
            }
        }
    }
}
