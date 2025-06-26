mod naive_game;
mod secure_game;
mod tfhe_keys;
mod tfhe_values;

use crate::naive_game::NaiveGame;
use crate::secure_game::SecureGame;
use crate::tfhe_keys::initialize_keys;

fn main() {
    run_naive_game();
    run_secure_game();
}

fn run_naive_game() {
    let mut game = NaiveGame::new(0);

    let deck = vec![8, 8, 7, 6, 7, 6];

    game.plant_deck(&deck);
    game.create_game();
    game.hit_as_player();
    game.stand();
    game.hit_as_dealer();

    game.dump_game();
}

fn run_secure_game() {
    let key = initialize_keys();

    let mut game = SecureGame::new(&key);

    let deck = vec![8, 8, 7, 6, 7, 6];

    game.plant_deck(&deck);
    game.create_game();
    game.hit_as_player();
    game.stand();
    game.hit_as_dealer();

    game.dump_game();
}
