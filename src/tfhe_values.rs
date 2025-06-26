use crate::secure_game::SecureGameState;

use tfhe::prelude::{FheDecrypt, FheEncrypt};
use tfhe::{ClientKey, FheUint8};

pub fn decrypt_cards(key: &ClientKey, cards: &[FheUint8]) -> Vec<u8> {
    cards.iter().map(|card| card.decrypt(key)).collect()
}

pub fn encrypt_points(key: &ClientKey, points: u8) -> FheUint8 {
    FheUint8::encrypt(points, key)
}

pub fn encrypt_state(key: &ClientKey, state: SecureGameState) -> FheUint8 {
    FheUint8::encrypt(state as u8, key)
}
