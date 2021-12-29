mod tic_toc;
mod ui {
    pub mod cli;
    pub mod gui;
    pub mod ui;
}

use std::env;

fn main() {
    let mut mode: ui::ui::Mode = ui::ui::Mode::CLI;
    for argument in env::args() {
        if argument.eq("gui") {
            mode = ui::ui::Mode::GUI;
        }
    }
    let mut game = tic_toc::game::Game::new(mode);
    game.run();
}
