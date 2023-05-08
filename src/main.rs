use conway_rust::Game;
use conway_rust::GameSize;

fn main() {
    let mut game = Game::new(
        GameSize {
            width: 80,
            height: 80,
        },
        125,
    );
    game.run();
}
