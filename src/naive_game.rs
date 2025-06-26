use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct NaiveGame {
    cards_for_dealer: Vec<u8>,
    cards_for_player: Vec<u8>,
    deck: Vec<u8>,
    rng: ChaCha8Rng,
    state: NaiveGameState,
}

impl NaiveGame {
    fn check_dealer(&mut self) {
        self.state = NaiveGameState::Checking;

        let points_for_player = self.rate_cards(&self.cards_for_player);
        let points_for_dealer = self.rate_cards(&self.cards_for_dealer);

        self.state = if points_for_dealer < 17 {
            NaiveGameState::WaitingForDealer
        } else {
            if points_for_dealer > 21 {
                NaiveGameState::DealerBusts
            } else {
                self.game_over(points_for_dealer, points_for_player)
            }
        };
    }

    fn check_dealer_and_player(&mut self) {
        self.state = NaiveGameState::Checking;

        let points_for_player = self.rate_cards(&self.cards_for_player);
        let points_for_dealer = self.rate_cards(&self.cards_for_dealer);

        self.state = if points_for_player == 21 {
            NaiveGameState::PlayerWins
        } else {
            if points_for_player > 21 {
                NaiveGameState::PlayerBusts
            } else {
                if points_for_dealer == 21 {
                    NaiveGameState::DealerWins
                } else {
                    if points_for_dealer > 21 {
                        NaiveGameState::DealerBusts
                    } else {
                        NaiveGameState::WaitingForPlayer
                    }
                }
            }
        }
    }

    fn check_player(&mut self) {
        self.state = NaiveGameState::Checking;

        let points_for_player = self.rate_cards(&self.cards_for_player);

        self.state = if points_for_player > 21 {
            NaiveGameState::PlayerBusts
        } else {
            NaiveGameState::WaitingForPlayer
        };
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

    pub fn dump_game(&self) {
        dbg!(&self.cards_for_player, &self.cards_for_dealer, &self.state);
    }

    fn game_over(&self, points_for_dealer: u8, points_for_player: u8) -> NaiveGameState {
        if points_for_dealer > points_for_player {
            NaiveGameState::DealerWins
        } else {
            if points_for_dealer < points_for_player {
                NaiveGameState::PlayerWins
            } else {
                NaiveGameState::Tie
            }
        }
    }

    pub fn hit_as_dealer(&mut self) {
        self.deal_dealer(1);
        self.check_dealer();
    }

    pub fn hit_as_player(&mut self) {
        self.deal_player(1);
        self.check_player();
    }

    pub fn new(seed: u64) -> Self {
        Self {
            cards_for_dealer: vec![],
            cards_for_player: vec![],
            deck: vec![],
            rng: ChaCha8Rng::seed_from_u64(seed),
            state: NaiveGameState::Uninitialized,
        }
    }

    pub fn plant_deck(&mut self, deck: &[u8]) {
        self.deck.extend(deck);
    }

    fn random_card(&mut self) -> u8 {
        (self.rng.next_u32() % 13 + 2) as u8
    }

    fn rate_card(&self, card: u8) -> u8 {
        if card < 11 {
            card
        } else {
            if card < 14 {
                10
            } else {
                11
            }
        }
    }

    fn rate_cards(&self, cards: &[u8]) -> u8 {
        cards
            .iter()
            .fold(0, |total, &card| total + self.rate_card(card))
    }

    pub fn stand(&mut self) {
        self.check_dealer();
    }
}

#[derive(Debug, PartialEq)]
pub enum NaiveGameState {
    Uninitialized,
    Checking,
    DealerBusts,
    DealerWins,
    PlayerBusts,
    PlayerWins,
    Tie,
    WaitingForDealer,
    WaitingForPlayer,
}

#[cfg(test)]
mod tests {
    use super::*;

    const J: u8 = 11;
    const Q: u8 = 12;
    const K: u8 = 13;
    const A: u8 = 14;

    #[test]
    fn create_game() {
        let mut game = NaiveGame::new(0);

        let deck = vec![9, 8, 7, 6];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(vec!(6, 7), game.cards_for_player);
        assert_eq!(vec!(8, 9), game.cards_for_dealer);
        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);
    }

    #[test]
    fn dealer_busts_early() {
        let mut game = NaiveGame::new(0);

        let deck = vec![A, A, 8, 7];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(vec!(7, 8), game.cards_for_player);
        assert_eq!(vec!(A, A), game.cards_for_dealer);
        assert_eq!(NaiveGameState::DealerBusts, game.state);
    }

    #[test]
    fn dealer_busts_late() {
        let mut game = NaiveGame::new(0);

        let deck = vec![9, 8, 7, 8, 7];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.stand();

        assert_eq!(NaiveGameState::WaitingForDealer, game.state);

        game.hit_as_dealer();

        assert_eq!(vec!(7, 8), game.cards_for_player);
        assert_eq!(vec!(7, 8, 9), game.cards_for_dealer);
        assert_eq!(NaiveGameState::DealerBusts, game.state);
    }

    #[test]
    fn dealer_wins() {
        let mut game = NaiveGame::new(0);

        let deck = vec![Q, J, 9, 8];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.stand();

        assert_eq!(vec!(8, 9), game.cards_for_player);
        assert_eq!(vec!(J, Q), game.cards_for_dealer);
        assert_eq!(NaiveGameState::DealerWins, game.state);
    }

    #[test]
    fn dealer_wins_early() {
        let mut game = NaiveGame::new(0);

        let deck = vec![A, K, 7, 6];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(vec!(6, 7), game.cards_for_player);
        assert_eq!(vec!(K, A), game.cards_for_dealer);
        assert_eq!(NaiveGameState::DealerWins, game.state);
    }

    #[test]
    fn dealer_wins_late() {
        let mut game = NaiveGame::new(0);

        let deck = vec![8, 7, 6, Q, J];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.stand();

        assert_eq!(NaiveGameState::WaitingForDealer, game.state);

        game.hit_as_dealer();

        assert_eq!(vec!(J, Q), game.cards_for_player);
        assert_eq!(vec!(6, 7, 8), game.cards_for_dealer);
        assert_eq!(NaiveGameState::DealerWins, game.state);
    }

    #[test]
    fn game_ends_in_a_tie() {
        let mut game = NaiveGame::new(0);

        let deck = vec![9, 8, 9, 8];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.stand();

        assert_eq!(vec!(8, 9), game.cards_for_player);
        assert_eq!(vec!(8, 9), game.cards_for_dealer);
        assert_eq!(NaiveGameState::Tie, game.state);
    }

    #[test]
    fn player_busts_early() {
        let mut game = NaiveGame::new(0);

        let deck = vec![8, 7, A, A];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(vec!(A, A), game.cards_for_player);
        assert_eq!(vec!(7, 8), game.cards_for_dealer);
        assert_eq!(NaiveGameState::PlayerBusts, game.state);
    }

    #[test]
    fn player_busts_late() {
        let mut game = NaiveGame::new(0);

        let deck = vec![9, 8, 7, 8, 7];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.hit_as_player();

        assert_eq!(vec!(7, 8, 9), game.cards_for_player);
        assert_eq!(vec!(7, 8), game.cards_for_dealer);
        assert_eq!(NaiveGameState::PlayerBusts, game.state);
    }

    #[test]
    fn player_wins() {
        let mut game = NaiveGame::new(0);

        let deck = vec![9, 8, Q, J];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.stand();

        assert_eq!(vec!(J, Q), game.cards_for_player);
        assert_eq!(vec!(8, 9), game.cards_for_dealer);
        assert_eq!(NaiveGameState::PlayerWins, game.state);
    }

    #[test]
    fn player_wins_early() {
        let mut game = NaiveGame::new(0);

        let deck = vec![7, 6, A, K];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(vec!(K, A), game.cards_for_player);
        assert_eq!(vec!(6, 7), game.cards_for_dealer);
        assert_eq!(NaiveGameState::PlayerWins, game.state);
    }

    #[test]
    fn player_wins_late() {
        let mut game = NaiveGame::new(0);

        let deck = vec![8, Q, J, 7, 6];

        game.plant_deck(&deck);
        game.create_game();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.hit_as_player();

        assert_eq!(NaiveGameState::WaitingForPlayer, game.state);

        game.stand();

        assert_eq!(vec!(6, 7, 8), game.cards_for_player);
        assert_eq!(vec!(J, Q), game.cards_for_dealer);
        assert_eq!(NaiveGameState::PlayerWins, game.state);
    }
}
