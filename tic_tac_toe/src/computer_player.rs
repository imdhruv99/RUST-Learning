use crate::player::Player;
use crate::board::Board;
use rand::Rng;

pub struct ComputerPlayer;

impl Player for ComputerPlayer {
    fn make_move(&self, board: &mut Board) {
        loop {
            let row = rand::thread_rng().gen_range(0..3);
            let col = rand::thread_rng().gen_range(0..3);

            if board.make_move(row, col, 'O') {
                break;
            }
        }
    }
}

