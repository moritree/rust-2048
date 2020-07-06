use rand;
use std::{thread, time};

enum Action {
    Up,
    Down,
    Left,
    Right,
    Restart,
    Quit
}

fn main() {
    let mut board = empty_board();

    // Main game loop
    loop {
        board = spawn_number(board);
        let mut high_score: u32 = 0;

        clear_console();
        print_board(board);

        if is_board_full(board) {
            let current = board.clone();
            if swipe_up(board) == current && swipe_down(board) == current && swipe_left(board) == current && swipe_right(board) == current {
                let score = calculate_score(board);
                if score > high_score {
                    high_score = score;
                }
                println!("\nYou lost :(\nYour score: {}", score);
                print!("High score: {}\n\nRestarting in 3s...", high_score);

                thread::sleep(time::Duration::from_millis(3000));
                board = empty_board();
                continue;
            }
        } 

        // Get valid action
        let mut quit = false;
        loop {
            let character = get_input();
            let action = get_user_action(character);
            let current = board.clone();
            match action {
                Some(Action::Quit) => {
                    quit = true;
                    break;
                }
                Some(Action::Up) => {
                    board = swipe_up(board);
                    if board != current { break; }
                }
                Some(Action::Down) => {
                    board = swipe_down(board);
                    if board != current { break; }
                }
                Some(Action::Left) => {
                    board = swipe_left(board);
                    if board != current { break; }
                }
                Some(Action::Right) => {
                    board = swipe_right(board);
                    if board != current { break; }
                }
                Some(Action::Restart)   => {
                    board = empty_board();
                    break;
                }
                None                    => continue
            }
        }
        if quit { break; }

        if is_win_condition(board) {
            println!("\nYou win! :D");
            break;
        }
    }
}

fn is_win_condition(board: [[Option<u16>; 4]; 4]) -> bool {
    for row in board.iter() {
        for item in row.iter() {
            match item {
                Some(2048) => return true,
                Some(_x) => continue,
                None => continue
            }
        }
    }
    return false;
}

fn calculate_score(board: [[Option<u16>; 4]; 4]) -> u32 {
    let mut total: u32 = 0;
    for row in board.iter() {
        for item in row.iter() {
            match item {
                Some(x) => total += *x as u32,
                None => continue
            }
        }
    }
    return total;
}

fn empty_board() -> [[Option<u16>; 4]; 4] {
    return [[None; 4]; 4];
}

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn spawn_number(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    let mut new_board = board.clone();
    let mut value = 2;
    let x = rand::random::<usize>();
    if x % 10 == 0 { 
        value = 4; 
    }
    loop {
        let rand_row = rand::random::<usize>() % 4;
        let rand_col = rand::random::<usize>() % 4;
        match new_board[rand_row][rand_col] {
            Some(_x) => continue,
            None => {
                new_board[rand_row][rand_col] = Some(value);
                break;
            }
        }
    }
    return new_board;
}

fn is_board_full(board: [[Option<u16>; 4]; 4]) -> bool {
    for row in board.iter() {
        for col in row.iter() {
            if col.is_none() {
                return false;
            }
        }
    }
    return true
}

fn swipe_right(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    let mut board = board.clone();
    for row in 0..4 {
        for col in (0..4).rev() {
            for test_col in (0..col).rev() {
                match board[row][test_col] {
                    Some(x) => {
                        match board[row][col] {
                            Some(y) => {
                                if x == y {
                                    board[row][col] = Some(x + y);
                                    board[row][test_col] = None;
                                    break;
                                } else {
                                    break;
                                }
                            }
                            None => {
                                board[row][col] = Some(x);
                                board[row][test_col] = None;
                            }
                        }
                    }
                    None => continue
                }
            }
        }
    }
    return board;
}

fn swipe_left(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    return flip_board_y(swipe_right(flip_board_y(board)));
}

fn swipe_up(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    return transpose_board(flip_board_y(swipe_right(flip_board_y(transpose_board(board)))));
}

fn swipe_down(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    return transpose_board(swipe_right(transpose_board(board)));
}

// Returns a transposed version of the supplied board (with rows and columns flipped)
fn transpose_board(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    let mut result_board = [[None; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            result_board[col][row] = board[row][col];
        }
    }
    return result_board
}

// Returns a version of the supplied board that is flipped across the Y axis
fn flip_board_y(board: [[Option<u16>; 4]; 4]) -> [[Option<u16>; 4]; 4] {
    let mut result_board = empty_board();
    for row in 0..4 {
        for col in 0..4 {
            result_board[row][3 - col] = board[row][col]
        }
    }
    return result_board
}

fn print_board(board: [[Option<u16>; 4]; 4]) {
    let mut row = 0;
    while row < 4 {
        let mut row_str = String::from("");
        for col in board[row].iter() {
            let mut string = String::from("[ ");
            match col {
                None => string.push_str("    "),
                Some(x) => {
                    string.push_str(&format!("{}", x));
                    for _s in 0..(4 - n_digits(*x)) {
                        string.push_str(" ");
                    }
                }
            }
            string.push_str(" ] ");
            row_str.push_str(&string);
        }
        println!("{}", row_str);
        row += 1;
    }
}

// Returns the last character of the input string (excepting the newline character)
// If no characters were input before the newline, it returns a space character
fn get_input() -> char {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let count = input.clone().chars().count();
    if count < 2 { 
        return ' ';
    }

    return input.chars().rev().nth(1).unwrap();
}

fn get_user_action(character: char) -> Option<Action> {
    if character == 'w' {
        Some(Action::Up)
    } else if character == 's' {
        Some(Action::Down)
    } else if character == 'a' {
        Some(Action::Left)
    } else if character == 'd' {
        Some(Action::Right)
    } else if character == 'q' {
        Some(Action::Quit)
    } else if character == 'r' {
        Some(Action::Restart)
    } else {
        None
    }
}

// Returns the number of digits in the given u16
fn n_digits(int: u16) -> u16 {
    let mut count = 0;
    let mut num = int;
    while num != 0 {
        num = num / 10;
        count += 1;
    }
    return count;
}