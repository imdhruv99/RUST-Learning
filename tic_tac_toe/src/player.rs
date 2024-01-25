use crate::board::Board;

pub trait Player {
    fn make_move(&self, board: &mut Board);
}

