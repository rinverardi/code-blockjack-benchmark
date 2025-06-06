mod game;
mod game_state;
mod tfhe_keys;
mod tfhe_values;

use game::Game;

use crate::tfhe_keys::initialize_keys;

fn main() {
    let key = initialize_keys();

    let mut game = Game::new(&key);

    let deck = vec![8, 8, 7, 6, 7, 6];

    game.plant_deck(&deck);
    game.create_game();
    game.hit_as_player();
    game.stand();
    game.hit_as_dealer();

    game.dump_game();
}
