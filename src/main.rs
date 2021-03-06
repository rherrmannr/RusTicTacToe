mod tic_tac_toe;
mod ui {
    pub mod cli;
    pub mod gui;
    pub mod ui_base;
}

use std::env;

fn main() {
    let mut mode: ui::ui_base::Mode = ui::ui_base::Mode::Cli;
    for argument in env::args() {
        if argument.eq("gui") {
            mode = ui::ui_base::Mode::Gui;
        }
    }
    let mut game = tic_tac_toe::game::Game::new(mode);
    game.run();
}
