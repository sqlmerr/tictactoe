mod game;

use game::game::Game;

fn main() {
    let mut game: Game = Game {
        name: "TicTacToe by sqlmerr".to_string(),
    };
    game.run();
}
