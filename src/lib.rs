use std::collections::HashMap;

use ark_bn254::Bn254;
use ark_circom::CircomConfig;
use once_cell::sync::OnceCell;

pub mod input;
pub mod prove;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Game {
    GAME2048,
    CRYPTORUMBLE,
}

static CONFIG: OnceCell<HashMap<Game, CircomConfig<Bn254>>> = OnceCell::new();

pub fn init_config() {
    let mut games = HashMap::new();

    let cfg_2048 = CircomConfig::<Bn254>::new(
        "materials/2048/game2048_60.wasm",
        "materials/2048/game2048_60.r1cs",
    )
    .unwrap();
    games.insert(Game::GAME2048, cfg_2048);

    let cfg_cr = CircomConfig::<Bn254>::new(
        "materials/crypto_rumble/crypto_rumble_30.wasm",
        "materials/crypto_rumble/crypto_rumble_30.r1cs",
    )
    .unwrap();
    games.insert(Game::CRYPTORUMBLE, cfg_cr);

    CONFIG.set(games).unwrap();
}

pub fn get_config(game: &Game) -> Option<CircomConfig<Bn254>> {
    match CONFIG.get() {
        Some(c) => c.get(&game).cloned(),
        None => {
            init_config();
            self::get_config(game)
        }
    }
}
