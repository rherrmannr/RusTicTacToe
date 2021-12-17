
mod tic_toc;
use tic_toc::{player, game_field};

mod ui {
    use super::*;
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
                Err(_) => {println! ("Type in a number"); continue},
            };
        }
    }
}

fn main() {
    let player1 = player::Player::new(1).expect("Failed to create Player");
    let player2 = player::Player::new(2).expect("Failed to create Player");
    let players = [player1, player2];
    let mut gf = game_field::GameField::new(4, &players);
    gf.set_sign(0, 2);
    gf.set_sign(1, 2);
    gf.set_sign(2, 2);
    gf.set_sign(0, 1);
    gf.set_sign(1, 1);
    gf.set_sign(2, 1);
    ui::display(&gf);
    println!("Read input {}", ui::read_input());
}
