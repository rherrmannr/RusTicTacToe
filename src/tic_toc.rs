#[cfg(test)]
mod player_tests {
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
    fn verify_signs() {
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

    #[derive(PartialEq, Debug, Copy, Clone)]
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

#[cfg(test)]
mod game_field_tests {
    use super::*;

    #[test]
    fn create_empty_game_field() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let field = game_field::GameField::new(3, players);

        assert_eq!(field.active_player().sign(), players[0].sign());
        assert_ne!(field.active_player().sign(), players[1].sign());
        assert_eq!(false, field.has_winner());

        assert_eq!(vec![vec![player::Sign::None; 3]; 3], *field.get_field());
        assert_eq!(3, field.size());
    }
    #[test]
    fn set_signs() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);
        let mut result = vec![vec![player::Sign::None; 3]; 3];
        assert_eq!(field.active_player().sign(), players[0].sign());

        field.set_sign(0, 0);
        result[0][0] = player::Sign::X;
        assert_eq!(false, field.has_winner());
        assert_eq!(result, *field.get_field());
        assert_eq!(field.active_player().sign(), players[1].sign());

        // try to set sign on already used field
        field.set_sign(0, 0);
        assert_eq!(false, field.has_winner());
        assert_eq!(result, *field.get_field());
        assert_eq!(field.active_player().sign(), players[1].sign());

        field.set_sign(1, 0);
        result[1][0] = player::Sign::O;
        assert_eq!(false, field.has_winner());
        assert_eq!(result, *field.get_field());
        assert_eq!(field.active_player().sign(), players[0].sign());

        field.set_sign(0, 2);
        result[0][2] = player::Sign::X;

        assert_eq!(false, field.has_winner());
        assert_eq!(result, *field.get_field());
        assert_eq!(field.active_player().sign(), players[1].sign());
    }

    #[test]
    fn set_invalid_sign() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);
        field.set_sign(3, 0);
    }

    #[test]
    fn first_row_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(0, 0); // X
        field.set_sign(1, 0); // O
        field.set_sign(0, 1); // X
        field.set_sign(1, 1); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(0, 2); // X

        assert!(field.has_winner());
    }

    #[test]
    fn second_row_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(1, 0); // X
        field.set_sign(2, 0); // O
        field.set_sign(1, 1); // X
        field.set_sign(2, 1); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(1, 2); // X

        assert!(field.has_winner());
    }
    #[test]
    fn third_row_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(2, 0); // X
        field.set_sign(1, 0); // O
        field.set_sign(2, 1); // X
        field.set_sign(1, 1); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(2, 2); // X

        assert!(field.has_winner());
    }
    #[test]
    fn first_column_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(0, 0); // X
        field.set_sign(0, 1); // O
        field.set_sign(1, 0); // X
        field.set_sign(0, 2); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(2, 0); // X

        assert!(field.has_winner());
    }
    #[test]
    fn second_column_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(0, 1); // X
        field.set_sign(0, 2); // O
        field.set_sign(1, 1); // X
        field.set_sign(2, 2); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(2, 1); // X
        assert!(field.has_winner());
    }
    #[test]
    fn third_column_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(0, 2); // X
        field.set_sign(0, 1); // O
        field.set_sign(1, 2); // X
        field.set_sign(0, 0); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(2, 2); // X
        assert!(field.has_winner());
    }

    #[test]
    fn column_does_not_win() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);

        field.set_sign(1, 1); // X
        field.set_sign(0, 0); // O
        field.set_sign(0, 1); // X
        field.set_sign(1, 0); // O
        assert_eq!(false, field.has_winner());
    }

    #[test]
    fn top_left_bottom_right_diagonal_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);
        field.set_sign(0, 0); // X
        field.set_sign(1, 0); // O
        field.set_sign(1, 1); // X
        field.set_sign(2, 1); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(2, 2); // X
        assert!(field.has_winner());
    }

    #[test]
    fn bottom_left_top_right_diagonal_wins() {
        let players = [
            player::Player::new(1).expect("No error"),
            player::Player::new(2).expect("No error"),
        ];
        let mut field = game_field::GameField::new(3, players);
        field.set_sign(2, 0); // X
        field.set_sign(1, 0); // O
        field.set_sign(1, 1); // X
        field.set_sign(2, 1); // O
        assert_eq!(false, field.has_winner());
        field.set_sign(0, 2); // X
        assert!(field.has_winner());
    }
}
pub mod game_field {
    use super::player;
    pub type Field = Vec<Vec<player::Sign>>;

    pub struct GameField {
        field: Field,
        players: [player::Player; 2],
    }
    pub enum State {
        Playing,
        Draw,
        Winner(player::Player),
    }

    impl GameField {
        pub fn new(size: usize, mut new_players: [player::Player; 2]) -> GameField {
            new_players[0].activate();
            new_players[1].deactivate();
            GameField {
                field: vec![vec![player::Sign::None; size]; size],
                players: new_players,
            }
        }

        pub fn set_sign(&mut self, row: usize, col: usize) {
            if self.has_winner() {
                return;
            }
            if !self.sign_is_valid(row, col) {
                return;
            }
            if self.field[row][col] == player::Sign::None {
                self.field[row][col] = *self.active_player().sign();
                if self.has_winner() {
                    return;
                }
                GameField::swap_player(self);
            }
        }

        fn sign_is_valid(&self, row: usize, col: usize) -> bool {
            row < self.field.len() && col < self.field.len()
        }

        pub fn has_winner(&self) -> bool {
            self.check_rows() || self.check_columns() || self.check_diagonals()
        }

        pub fn is_draw(&self) -> bool {
            self.field
                .iter()
                .flatten()
                .filter(|&&sign| sign == player::Sign::None)
                .count()
                == 0
        }

        pub fn get_state(&self) -> State {
            if self.has_winner() {
                return State::Winner(*self.active_player());
            }
            if self.is_draw() {
                return State::Draw;
            }
            State::Playing
        }

        pub fn get_winner(&self) -> Option<&player::Player> {
            if self.has_winner() {
                return Some(self.active_player());
            }
            None
        }

        fn check_rows(&self) -> bool {
            let iter = (0..self.field.len()).map(|index| {
                self.field
                    .iter()
                    .flatten()
                    .skip(index)
                    .step_by(self.field.len())
            });
            for (_, rows) in iter.enumerate() {
                if GameField::check_lines(rows) {
                    return true;
                }
            }
            false
        }

        fn check_columns(&self) -> bool {
            for column in self.field.iter() {
                if GameField::check_lines(column.iter()) {
                    return true;
                }
            }
            false
        }

        fn check_lines<'a, I>(iter: I) -> bool
        where
            I: Iterator<Item = &'a player::Sign> + Clone,
        {
            GameField::check_line(iter.clone(), player::Sign::O)
                || GameField::check_line(iter, player::Sign::X)
        }

        fn check_line<'a, I>(iter: I, sign: player::Sign) -> bool
        where
            I: Iterator<Item = &'a player::Sign>,
        {
            iter.into_iter().all(|&it| it == sign)
        }

        fn check_diagonals(&self) -> bool {
            let mut top_left_bottom_right: Vec<player::Sign> = Vec::new();
            let mut bottom_left_top_right: Vec<player::Sign> = Vec::new();
            for i in 0..self.field.len() {
                top_left_bottom_right.push(self.field[i][i]);
                bottom_left_top_right.push(self.field[self.field.len() - i - 1][i]);
            }
            if GameField::check_line(top_left_bottom_right.iter(), player::Sign::O) {
                return true;
            }
            if GameField::check_line(top_left_bottom_right.iter(), player::Sign::X) {
                return true;
            }
            if GameField::check_line(bottom_left_top_right.iter(), player::Sign::O) {
                return true;
            }
            if GameField::check_line(bottom_left_top_right.iter(), player::Sign::X) {
                return true;
            }
            false
        }

        pub fn get_field(&self) -> &Field {
            &self.field
        }

        pub fn swap_player(&mut self) {
            if *self.active_player() == self.players[0] {
                self.players[0].deactivate();
                self.players[1].activate();
            } else {
                self.players[0].activate();
                self.players[1].deactivate();
            }
        }

        pub fn active_player(&self) -> &player::Player {
            if self.players[1].is_active() {
                return &self.players[1];
            }
            &self.players[0]
        }

        pub fn size(&self) -> usize {
            self.field.len()
        }
    }
}
pub mod game {
    use crate::ui::cli::Cli;
    use crate::ui::gui::Gui;
    use crate::ui::ui_base::*;

    use super::game_field;
    use super::player;
    pub struct Game {
        gamefield: game_field::GameField,
        ui: Box<dyn UI>,
        active: bool,
    }
    impl Game {
        pub fn new(mode: Mode) -> Game {
            let gamefield = game_field::GameField::new(3, Game::create_players());
            let ui: Box<dyn UI>;
            match mode {
                Mode::Cli => ui = Box::new(Cli::new()),
                Mode::Gui => ui = Box::new(Gui::new()),
            }
            Game {
                gamefield,
                ui,
                active: false,
            }
        }

        fn create_players() -> [player::Player; 2] {
            [
                player::Player::new(1).expect("No error"),
                player::Player::new(2).expect("No error"),
            ]
        }

        fn update(&mut self, event: Event) {
            match event {
                Event::Quit => self.active = false,
                Event::Point((row, column)) => self.gamefield.set_sign(row, column),
                Event::Restart => self.restart(),
                Event::None => {}
            }
        }

        pub fn restart(&mut self) {
            self.gamefield = game_field::GameField::new(3, Game::create_players());
        }

        pub fn run(&mut self) {
            self.active = true;
            while self.active {
                let event = self.ui.process_input(&self.gamefield);
                self.update(event);
                self.ui.display(&self.gamefield);
            }
        }
    }
}
