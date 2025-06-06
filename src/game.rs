use crate::game_state::GameState;
use crate::tfhe_values::{decrypt_cards, encrypt_points, encrypt_state, SEED_COUNTER};

use std::sync::atomic::Ordering;

use tfhe::prelude::{DivRem, FheDecrypt, FheEq, FheOrd, IfThenElse};
use tfhe::{ClientKey, FheUint8, Seed};

pub struct Game<'info> {
    cards_for_dealer: Vec<FheUint8>,
    cards_for_player: Vec<FheUint8>,
    deck: Vec<FheUint8>,
    key: &'info ClientKey,
    state: GameState,
}

impl<'info> Game<'info> {
    fn check_dealer(&mut self) {
        self.state = GameState::Checking;

        let points_for_player = self.rate_cards(&self.cards_for_player);
        let points_for_dealer = self.rate_cards(&self.cards_for_dealer);

        let state = points_for_dealer.lt(17).select(
            &encrypt_state(self.key, GameState::WaitingForDealer),
            &points_for_dealer.gt(21).select(
                &encrypt_state(self.key, GameState::DealerBusts),
                &self.game_over(&points_for_dealer, &points_for_player),
            ),
        );

        self.decrypt_state(state);
    }

    fn check_dealer_and_player(&mut self) {
        self.state = GameState::Checking;

        let points_for_player = self.rate_cards(&self.cards_for_player);
        let points_for_dealer = self.rate_cards(&self.cards_for_dealer);

        let state = points_for_player.eq(21).select(
            &encrypt_state(self.key, GameState::PlayerWins),
            &points_for_player.gt(21).select(
                &encrypt_state(self.key, GameState::PlayerBusts),
                &points_for_dealer.eq(21).select(
                    &encrypt_state(self.key, GameState::DealerWins),
                    &points_for_dealer.gt(21).select(
                        &encrypt_state(self.key, GameState::DealerBusts),
                        &encrypt_state(self.key, GameState::WaitingForPlayer),
                    ),
                ),
            ),
        );

        self.decrypt_state(state);
    }

    fn check_player(&mut self) {
        self.state = GameState::Checking;

        let points_for_player = self.rate_cards(&self.cards_for_player);

        let state = points_for_player.gt(21).select(
            &encrypt_state(self.key, GameState::PlayerBusts),
            &encrypt_state(self.key, GameState::WaitingForPlayer),
        );

        self.decrypt_state(state);
    }

    pub fn create_game(&mut self) {
        self.deal_player(2);
        self.deal_dealer(2);
        self.check_dealer_and_player();
    }

    fn deal_dealer(&mut self, count: u8) {
        for _ in 0..count {
            let card = if self.deck.is_empty() {
                self.random_card()
            } else {
                self.deck.pop().unwrap()
            };

            self.cards_for_dealer.push(card);
        }
    }

    fn deal_player(&mut self, count: u8) {
        for _ in 0..count {
            let card = if self.deck.is_empty() {
                self.random_card()
            } else {
                self.deck.pop().unwrap()
            };

            self.cards_for_player.push(card);
        }
    }

    fn decrypt_state(&mut self, state: FheUint8) {
        let state_value: u8 = state.decrypt(self.key);

        self.state = GameState::try_from(state_value).unwrap();
    }

    pub fn dump_game(&self) {
        dbg!(
            decrypt_cards(&self.key, &self.cards_for_player),
            decrypt_cards(&self.key, &self.cards_for_dealer),
            self.state
        );
    }

    fn game_over(&self, points_for_dealer: &FheUint8, points_for_player: &FheUint8) -> FheUint8 {
        points_for_dealer.gt(points_for_player).select(
            &encrypt_state(self.key, GameState::DealerWins),
            &points_for_dealer.lt(points_for_player).select(
                &encrypt_state(self.key, GameState::PlayerWins),
                &encrypt_state(self.key, GameState::Tie),
            ),
        )
    }

    pub fn hit_as_dealer(&mut self) {
        self.deal_dealer(1);
        self.check_dealer();
    }

    pub fn hit_as_player(&mut self) {
        self.deal_player(1);
        self.check_player();
    }

    pub fn new(key: &'info ClientKey) -> Self {
        Self {
            cards_for_dealer: vec![],
            cards_for_player: vec![],
            deck: vec![],
            key: key,
            state: GameState::Uninitialized,
        }
    }

    pub fn plant_deck(&mut self, deck: &[u8]) {
        deck.iter().for_each(|&card_value| {
            let card = encrypt_points(self.key, card_value);

            self.deck.push(card);
        });
    }

    fn random_card(&self) -> FheUint8 {
        let seed_counter = SEED_COUNTER.fetch_add(1, Ordering::Relaxed) as u128;

        FheUint8::div_rem(
            FheUint8::generate_oblivious_pseudo_random(Seed(seed_counter)),
            13,
        )
        .1 + encrypt_points(self.key, 2)
    }

    fn rate_card(&self, card: &FheUint8) -> FheUint8 {
        card.lt(11).select(
            card,
            &card
                .lt(14)
                .select(&encrypt_points(self.key, 10), &encrypt_points(self.key, 11)),
        )
    }

    fn rate_cards(&self, cards: &[FheUint8]) -> FheUint8 {
        let zero = encrypt_points(self.key, 0);

        cards
            .iter()
            .fold(zero, |total, card| total + self.rate_card(card))
    }

    pub fn stand(&mut self) {
        self.check_dealer();
    }
}
