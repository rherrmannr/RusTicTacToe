#[cfg(test)]
mod tests {
    use super::player;

    #[test]
    fn create_valid_players() {
        let player1 = player::Player::new(1);
        let player2 = player::Player::new(2);

        assert!(!player1.is_err());
        assert!(!player2.is_err());

        let player1 = player1.expect("Not failed");
        let player2 = player2.expect("Not failed");

        assert_eq!(player::Sign::X, *player1.sign());
        assert_eq!(player::Sign::O, *player2.sign());
    }

    #[test]
    fn create_invalid_player() {
        let player = player::Player::new(0);
        assert!(player.is_err());
        let player = player::Player::new(3);
        assert!(player.is_err());
    }

    #[test]
    fn print_signs() {
        assert_eq!("X", format!("{}", player::Sign::X));
        assert_eq!("O", format!("{}", player::Sign::O));
        assert_eq!("-", format!("{}", player::Sign::None));
    }

    #[test]
    fn activate_deactivate() {
        let mut player = player::Player::new(1).expect("Error creating player");
        assert!(!player.is_active());
        player.activate();
        assert!(player.is_active());
        player.deactivate();
        assert!(!player.is_active());
    }
}
pub mod player {

    use std::fmt;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Sign {
        X,
        O,
        None,
    }

    impl fmt::Display for Sign {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                Sign::X => write!(f, "X"),
                Sign::O => write!(f, "O"),
                Sign::None => write!(f, "-"),
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Player {
        number: u8,
        sign: Sign,
        active: bool,
    }
    impl Player {
        pub fn new(number: u8) -> Result<Self, ()> {
            match number {
                1 => Ok(Player {
                    number,
                    sign: Sign::X,
                    active: false,
                }),
                2 => Ok(Player {
                    number,
                    sign: Sign::O,
                    active: false,
                }),
                _ => Err(()),
            }
        }

        pub fn sign(&self) -> &Sign {
            &self.sign
        }

        pub fn is_active(&self) -> bool {
            self.active
        }
        pub fn activate(&mut self) {
            self.active = true;
        }

        pub fn deactivate(&mut self) {
            self.active = false;
        }
    }
}

pub mod game_field {
    use super::player;
    pub type Field = Vec<Vec<player::Sign>>;

    pub struct GameField {
        field: Field,
        player: [player::Player; 2],
    }

    impl GameField {
        pub fn new(size: usize, player: [player::Player; 2]) -> GameField {
            GameField {
                field: vec![vec![player::Sign::None; size]; size],
                player: player,
            }
        }

        pub fn set_sign(&mut self, row: usize, col: usize) {
            if self.field[row][col] == player::Sign::None {
                self.field[row][col] = *self.active_player().sign();
                GameField::swap_player(self);
            }
        }

        pub fn get_field(&self) -> &Field {
            &self.field
        }

        pub fn swap_player(&mut self) {
            if *self.active_player() == self.player[0] {
                self.player[0].deactivate();
                self.player[1].activate();
            } else {
                self.player[0].activate();
                self.player[1].deactivate();
            }
        }

        pub fn active_player(&self) -> &player::Player {
            if self.player[1].is_active() {
                return &self.player[1];
            }
            return &self.player[0];
        }
    }
}

pub mod game {
    use super::game_field;
    use super::player;
    pub struct Game {
        gamefield: game_field::GameField,
    }
    impl Game {
        pub fn new(&mut self) {
            let mut players = [
                player::Player::new(1).expect("No error"),
                player::Player::new(2).expect("No error"),
            ];
            players[0].activate();
            self.gamefield = game_field::GameField::new(3, players);
        }

        pub fn start(&mut self) {}
    }
}
