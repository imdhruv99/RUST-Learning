use crate::player::Player;
use crate::board::Board;

#[derive(PartialEq)]
pub enum GameState {
    Ongoing,
    Won(char),
    Tied,
}

pub struct Game {
    board: Board,
    players: [Box<dyn Player>; 2],
    current_player: usize,
    state: GameState,
}

impl Game {
    pub fn new(player1: Box<dyn Player>, player2: Box<dyn Player>) -> Self {
        Game {
            board: Board::new(),
            players: [player1, player2],
            current_player: 0,
            state: GameState::Ongoing,
        }
    }

    pub fn play(&mut self) {
        while self.state == GameState::Ongoing {
            self.board.display();

            let current_marker = if self.current_player == 0 { 'X' } else { 'O' };

            println!("Player {}'s turn.", current_marker);
            self.players[self.current_player].make_move(&mut self.board);

            if let Some(winner) = self.board.check_winner() {
                self.state = GameState::Won(winner);
            } else if self.board.is_full() {
                self.state = GameState::Tied;
            }

            self.current_player = 1 - self.current_player; // Switch players
        }

        self.board.display();
        match self.state {
            GameState::Won(winner) => println!("Player {} wins!", winner),
            GameState::Tied => println!("The game is a tie!"),
            _ => unreachable!(),
        }
    }
}
