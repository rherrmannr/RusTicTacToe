use super::ui_base::*;
use crate::tic_toc::game_field::GameField;
use std::io;

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }
}

impl UI for Cli {
    fn display(&mut self, game_field: &GameField) {
        Cli::print_gamefield(game_field);

        match game_field.get_winner() {
            Some(player) => {
                println!();
                println!("{} has won!", player.sign());
                println!();
            }
            None => {
                println!();
                println!("It's {}'s turn.", game_field.active_player().sign());
            }
        }
    }

    fn process_input(&mut self, _game_field: &GameField) -> Event {
        Event::Point(Cli::get_point())
    }
}

impl Cli {
    fn print_gamefield(game_field: &GameField) {
        for row in game_field.get_field() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }

    pub fn get_point() -> (usize, usize) {
        let mut result = (0, 0);
        println!("Type in the row.");
        result.0 = Cli::read_single_input();
        println!("Type in the column.");
        result.1 = Cli::read_single_input();
        println!();
        result
    }

    fn read_single_input() -> usize {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed");
            return match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Type in a number.");
                    continue;
                }
            };
        }
    }
}
