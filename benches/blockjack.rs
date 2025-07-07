use std::time::Duration;

use blockjack::{naive_game::NaiveGame, secure_game::SecureGame};
use blockjack::tfhe_keys::initialize_keys;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn blockjack(criterion: &mut Criterion) {
    let (client_key, _) = initialize_keys();

    let mut group = criterion.benchmark_group("blockjack");

    group.bench_function(BenchmarkId::new("playNaive", 0), |bench| {
        bench.iter(|| {
            let mut game = NaiveGame::new(0);

            let deck = vec![6, 6, 6, 6, 6, 6];

            game.plant_deck(&deck);
            game.create_game();
            game.hit_as_player();
            game.stand();
            game.hit_as_dealer();

            game.dump_game();
        })
    });

    group.bench_function(BenchmarkId::new("playSecure", 0), |bench| {
        bench.iter(|| {
            let mut game = SecureGame::new(&client_key);

            let deck = vec![6, 6, 6, 6, 6, 6];

            game.plant_deck(&deck);
            game.create_game();
            game.hit_as_player();
            game.stand();
            game.hit_as_dealer();

            game.dump_game();
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(120)).sample_size(10);
    targets = blockjack
}

criterion_main!(benches);
