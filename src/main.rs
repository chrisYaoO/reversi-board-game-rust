use std::io;
use std::io::Write;

const B_SIZE: i8 = 8;

fn main() {
    let mut player: u8 = 0; //player 0 for Black, 1 for White

    let mut board: Vec<Vec<char>> = board_init(B_SIZE as usize);
    board_show(&board, B_SIZE as usize);

    loop {
        if player == 0 {
            print!("Enter move for colour B (RowCol): ");
        } else {
            print!("Enter move for colour W (RowCol): ");
        }

        io::stdout().flush().expect("Failed to flush stdout.");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed: &str = input.trim();
        let _move = str_to_move(&board, &trimmed, player);
        if let Some((first, second)) = _move {
            println!("move: {first},{second}");
            board_show(&board, B_SIZE as usize);
            player = 1 - player;
        } else {
            println!("Invalid move. Try again.");
            board_show(&board, B_SIZE as usize);
        }
    }
}
// convert input string to moves
fn str_to_move(board: &Vec<Vec<char>>, s: &str, player: u8) -> Option<(i8, i8)> {
    if s.len() < 2 || !s.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let mut chars = s.chars();
    let first = (chars.next().unwrap() as u8 - b'a') as i8;
    let second = (chars.next().unwrap() as u8 - b'a') as i8;
    if on_board(first, second) {
        match valid_move(&board, first, second, player) {
            true => Some((first, second)),
            false => None,
        }
    } else {
        None
    }
}

fn valid_move(board: &Vec<Vec<char>>, row: i8, col: i8, player: u8) -> bool {
    //rule0: no overlapping
    if board[row as usize][col as usize] != '.' {
        return false;
    }
    // rule1: adjacent to the opponent + rule2: sandwiching condition
    let oppo_color: char = player_to_color(1 - player);
    let player_color: char = player_to_color(player);

    let mut flag: bool = false;
    'outer: for i in (row - 1)..=(row + 1) {
        for j in (col - 1)..=(col + 1) {
            if on_board(i, j) && board[i as usize][j as usize] == oppo_color {
                flag = true;
                break 'outer;
            }
        }
    }
    if flag == false {
        return false;
    }

    for i in 0..B_SIZE {
        if board[i as usize][col as usize] == player_color && i != row {
            return true;
        }
        if board[row as usize][i as usize] == player_color && i != col {
            return true;
        }
    }
    for i in 1..B_SIZE {
        if on_board(row + i, col + i) {
            if board[(row + i) as usize][(col + i) as usize] == player_color {
                return true;
            }
        }
        if on_board(row + i, col - i) {
            if board[(row + i) as usize][(col - i) as usize] == player_color {
                return true;
            }
        }
        if on_board(row - i, col - i) {
            if board[(row - i) as usize][(col - i) as usize] == player_color {
                return true;
            }
        }
        if on_board(row - i, col + i) {
            if board[(row - i) as usize][(col + i) as usize] == player_color {
                return true;
            }
        }
    }
    false
}

fn on_board(row: i8, col: i8) -> bool {
    if row < B_SIZE && col < B_SIZE && row >= 0 && col >= 0 {
        return true;
    }
    false
}
fn player_to_color(player: u8) -> char {
    match player {
        0 => 'B',
        1 => 'W',
        _ => '?',
    }
}
fn num_to_char(num: u8) -> Option<char> {
    if (1..=26).contains(&num) {
        Some((b'a' + (num - 1)) as char)
    } else {
        None
    }
}

fn board_init(n: usize) -> Vec<Vec<char>> {
    let mut board = vec![vec!['.'; n]; n];
    board[n / 2 - 1][n / 2 - 1] = 'W';
    board[n / 2 - 1][n / 2] = 'B';
    board[n / 2][n / 2] = 'W';
    board[n / 2][n / 2 - 1] = 'B';

    board
}

fn board_show(board: &Vec<Vec<char>>, n: usize) {
    print!("  ");
    for i in 0..n {
        if let Some(c) = num_to_char((i + 1) as u8) {
            print!("{c}");
        } else {
            println!("invalid size");
        }
    }
    print!("\n");
    for i in 0..n {
        if let Some(c) = num_to_char((i + 1) as u8) {
            print!("{c} ");
        } else {
            println!("invalid size");
        }
        for j in 0..n {
            let c: char = board[i][j];
            print!("{c}");
        }
        print!("\n");
    }
}

fn point_counter(board: &Vec<Vec<char>>, n: usize) -> (i32, i32) {
    let mut counter_w: i32 = 0;
    let mut counter_b: i32 = 0;
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == 'W' {
                counter_w += 1;
            } else if board[i][j] == 'B' {
                counter_b += 1;
            }
        }
    }
    (counter_w, counter_b)
}
