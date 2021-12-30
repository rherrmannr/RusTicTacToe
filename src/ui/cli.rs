use super::ui::*;
use crate::tic_toc::game_field::GameField;
use std::io;

pub struct CLI {}

impl CLI {
    pub fn new() -> CLI {
        CLI {}
    }
}

impl UI for CLI {
    fn display(&mut self, game_field: &GameField) {
        CLI::print_gamefield(game_field);

        match game_field.get_winner() {
            Some(player) => {
                println!();
                println!("{} has won!", player.sign());
                println!();
                return;
            }
            None => {
                println!();
                println!("It's {}'s turn.", game_field.active_player().sign());
            }
        }
    }

    fn process_input(&mut self) -> Event {
        Event::Point(CLI::get_point())
    }
}

impl CLI {
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
        result.0 = CLI::read_single_input();
        println!("Type in the column.");
        result.1 = CLI::read_single_input();
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
