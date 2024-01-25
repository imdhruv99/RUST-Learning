use crate::player::Player;
use crate::board::Board;
use std::io;

pub struct HumanPlayer;

impl Player for HumanPlayer {
    fn make_move(&self, board: &mut Board) {
        loop {
            println!("Enter row (0-2): ");
            let row: usize = read_input();
            println!("Enter column (0-2): ");
            let col: usize = read_input();

            if board.make_move(row, col, 'X') {
                break;
            } else {
                println!("Invalid move. Try again.");
            }
        }
    }
}

fn read_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}

