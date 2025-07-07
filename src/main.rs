use std::env;
use std::io::{stdout, Write};

use blockjack::tfhe_keys::initialize_keys;
use blockjack::{naive_game::NaiveGame, secure_game::SecureGame};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use tfhe::{set_server_key, ClientKey};

fn blip() {
    stdout().write(b".").unwrap();
    stdout().flush().unwrap();
}

fn play_naive_game() {
    let mut game = NaiveGame::new(0);

    let deck = vec![6, 6, 6, 6, 6, 6];

    game.plant_deck(&deck);
    game.create_game();
    game.hit_as_player();
    game.stand();
    game.hit_as_dealer();
}

fn play_naive_games() {
    for parallel_games in (10..=100).step_by(10) {
        dbg!(parallel_games);

        (0..parallel_games).into_par_iter().for_each(|_| {
            play_naive_game();
            blip();
        });

        println!();
    }
}

fn play_secure_game(key: ClientKey) {
    let mut game = SecureGame::new(&key);

    let deck = vec![6, 6, 6, 6, 6, 6];

    game.plant_deck(&deck);
    game.create_game();
    game.hit_as_player();
    game.stand();
    game.hit_as_dealer();
}

fn play_secure_games() {
    let (client_key, server_key) = initialize_keys();

    rayon::broadcast(|_| set_server_key(server_key.clone()));

    for parallel_games in (10..=100).step_by(10) {
        dbg!(parallel_games);

        (0..parallel_games).into_par_iter().for_each(|_| {
            play_secure_game(client_key.clone());
            blip();
        });

        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        match arg.as_str() {
            "naive" => play_naive_games(),
            "secure" => play_secure_games(),
            other => {
                eprintln!("Unknown command: '{}'", other);
                std::process::exit(1);
            }
        }
    }
}
