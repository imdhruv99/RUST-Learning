mod player;
mod human_player;
mod computer_player;
mod board;
mod game;

use human_player::HumanPlayer;
use computer_player::ComputerPlayer;
use game::Game;
use player::Player;

fn main() {
    let player1 = Box::new(HumanPlayer);
    let player2 = Box::new(ComputerPlayer);

    let mut game = Game::new(player1, player2);
    game.play();
}
