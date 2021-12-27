mod tic_toc;
mod ui;

fn main() {
    let mut game = tic_toc::game::Game::new();
    game.run();
}
