mod tic_toc;
mod ui {
    pub mod ui;
    pub mod cli;
}

fn main() {
    let mut game = tic_toc::game::Game::new();
    game.run();
}
