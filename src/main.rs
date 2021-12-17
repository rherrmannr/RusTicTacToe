mod tic_toc;
use tic_toc::{game_field, player};
mod ui;


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
