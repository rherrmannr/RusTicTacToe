use super::tic_toc::game;

use std::io;

pub fn display(game: &mut game::Game) {
    print_gamefield(game);

    match game.gamefield.get_winner() {
        Some(player) => {
            println!();
            println!("{} has won!", player.sign());
            println!();
            game.restart();
            return;
        }
        None => {
            println!();
            println!("It's {}'s turn.", game.gamefield.active_player().sign());
        }
    }
    let point = get_point();
    game.gamefield.set_sign(point.0, point.1);
}

fn print_gamefield(game: &mut game::Game) {
    for row in game.gamefield.get_field() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

pub fn get_point() -> (usize, usize) {
    let mut result = (0, 0);
    println!("Type in the row.");
    result.0 = read_single_input();
    println!("Type in the column.");
    result.1 = read_single_input();
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
