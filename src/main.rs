use std::io;

fn main() {
    let n = 8;



    loop {
        println!("Input your move:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        let _move = str_to_move(&trimmed);
        if let Some((first, second)) = _move {
            println!("move: {first},{second}");
            break;
        } else {
            println!("Invalid move:{}", trimmed);
        }
    }
}
// convert input string to moves
fn str_to_move(s: &str) -> Option<(u8, u8)> {
    if s.len() < 2 || !s.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let mut chars = s.chars();
    let first = (chars.next().unwrap() as u8 - b'a' + 1) as u8;
    let second = (chars.next().unwrap() as u8 - b'a' + 1) as u8;
    if first< 9 && second< 9 && first>0 && second>0 {
        Some((first, second))
    }else {
        None
    }
}

fn matrix_init(n: i32) -> Vec<Vec<i32>> {
    let
}
