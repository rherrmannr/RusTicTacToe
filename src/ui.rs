use super::tic_toc::game_field;

use std::io;

pub fn display(game: &game_field::GameField) {
    for row in game.get_field() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

pub fn read_input() -> u8 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        return match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type in a number");
                continue;
            }
        };
    }
}
