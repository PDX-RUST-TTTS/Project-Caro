use gomoku_ai::Player;
use gomoku_ai::Point;
use gomoku_ai::MAX;

#[test]
fn test_horizon1() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(10, 11);
    let point3 = Point::new(10, 12);
    let point4 = Point::new(10, 13);
    let point5 = Point::new(10, 14);

    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), true);
}

#[test]
fn test_horizon2() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(10, 11);
    let point3 = Point::new(10, 12);
    let point4 = Point::new(10, 13);
    let point5 = Point::new(10, 14);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), true);
}

#[test]
fn test_horizon3() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(10, 11);
    let point3 = Point::new(10, 12);
    let point4 = Point::new(10, 13);
    let point5 = Point::new(10, 14);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), true);
}

#[test]
fn test_vertical1() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 10);
    let point3 = Point::new(12, 10);
    let point4 = Point::new(13, 10);
    let point5 = Point::new(14, 10);

    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), true);
}

#[test]
fn test_vertical2() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 10);
    let point3 = Point::new(12, 10);
    let point4 = Point::new(13, 10);
    let point5 = Point::new(14, 10);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), true);
}
#[test]
fn test_vertical3() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 10);
    let point3 = Point::new(12, 10);
    let point4 = Point::new(13, 10);
    let point5 = Point::new(14, 10);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), true);
}

#[test]
fn test_cross_down1() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 11);
    let point3 = Point::new(12, 12);
    let point4 = Point::new(13, 13);
    let point5 = Point::new(14, 14);

    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), true);
}

#[test]
fn test_cross_down2() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 11);
    let point3 = Point::new(12, 12);
    let point4 = Point::new(13, 13);
    let point5 = Point::new(14, 14);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), true);
}

#[test]
fn test_cross_down3() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 11);
    let point3 = Point::new(12, 12);
    let point4 = Point::new(13, 13);
    let point5 = Point::new(14, 14);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), true);
}

#[test]
fn test_cross_up1() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 9);
    let point3 = Point::new(12, 8);
    let point4 = Point::new(13, 7);
    let point5 = Point::new(14, 6);

    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), true);
}

#[test]
fn test_cross_up2() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 9);
    let point3 = Point::new(12, 8);
    let point4 = Point::new(13, 7);
    let point5 = Point::new(14, 6);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), true);
}

#[test]
fn test_cross_up3() {
    let mut test = 0.0;
    let mut matrix: [[u8; MAX]; MAX] = [[0; MAX]; MAX];
    let mut player1 = Player::new(1);
    let point1 = Point::new(10, 10);
    let point2 = Point::new(11, 9);
    let point3 = Point::new(12, 8);
    let point4 = Point::new(13, 7);
    let point5 = Point::new(14, 6);

    assert_eq!(player1.add_new_point(point1, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point2, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point3, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point4, &mut matrix, &mut test), false);
    assert_eq!(player1.add_new_point(point5, &mut matrix, &mut test), true);
}
