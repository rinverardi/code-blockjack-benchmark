#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
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

impl TryFrom<u8> for GameState {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameState::Uninitialized),
            1 => Ok(GameState::Checking),
            2 => Ok(GameState::DealerBusts),
            3 => Ok(GameState::DealerWins),
            4 => Ok(GameState::PlayerBusts),
            5 => Ok(GameState::PlayerWins),
            6 => Ok(GameState::Tie),
            7 => Ok(GameState::WaitingForDealer),
            8 => Ok(GameState::WaitingForPlayer),
            _ => Err(()),
        }
    }
}
