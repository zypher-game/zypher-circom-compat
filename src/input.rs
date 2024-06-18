use std::{collections::HashMap, str::FromStr};

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game2048Input {
    pub board: Vec<Vec<u8>>,
    pub packed_board: Vec<u128>,
    pub packed_dir: u128,
    pub direction: Vec<u8>,
    pub address: String,
    pub nonce: String,
    pub step: u8,
    pub step_after: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoRumbleInput {
    pub from_seed: String,
    pub to_seed: String,
    pub from_board: Vec<Vec<u8>>,
    pub to_board: Vec<Vec<u8>>,
    pub step: u8,
    pub step_after: u8,
    pub from_board_packed: String,
    pub to_board_packed: String,
    pub score_packed: u128,
    pub pos_packed: String,
    pub item_packed: String,
    pub moves: Vec<Vec<u8>>,
    pub arg: Vec<u64>,
}

pub struct Input {
    pub maps: HashMap<String, Vec<BigInt>>,
}

impl TryFrom<Game2048Input> for Input {
    type Error = anyhow::Error;

    fn try_from(i: Game2048Input) -> Result<Self, Self::Error> {
        let mut board = vec![];
        for x in i.board.iter().flatten() {
            board.push(BigInt::from(*x))
        }

        let mut packed_board = vec![];
        for x in i.packed_board {
            packed_board.push(BigInt::from(x))
        }

        let mut direction = vec![];
        for x in i.direction {
            direction.push(BigInt::from(x))
        }

        let packed_dir = BigInt::from(i.packed_dir);
        let address = BigInt::from_str(&i.address)?;
        let nonce = BigInt::from_str(&i.nonce)?;
        let step = BigInt::from(i.step);
        let step_after = BigInt::from(i.step_after);

        let mut maps = HashMap::new();
        maps.insert("board".to_string(), board);
        maps.insert("packedBoard".to_string(), packed_board);
        maps.insert("packedDir".to_string(), vec![packed_dir]);
        maps.insert("direction".to_string(), direction);
        maps.insert("address".to_string(), vec![address]);
        maps.insert("step".to_string(), vec![step]);
        maps.insert("stepAfter".to_string(), vec![step_after]);
        maps.insert("nonce".to_string(), vec![nonce]);

        Ok(Input { maps })
    }
}

impl TryFrom<CryptoRumbleInput> for Input {
    type Error = anyhow::Error;

    fn try_from(i: CryptoRumbleInput) -> Result<Self, Self::Error> {
        let mut from_board = vec![];
        for x in i.from_board.iter().flatten() {
            from_board.push(BigInt::from(*x))
        }

        let mut to_board = vec![];
        for x in i.to_board.iter().flatten() {
            to_board.push(BigInt::from(*x))
        }

        let mut moves = vec![];
        for x in i.moves.iter().flatten() {
            moves.push(BigInt::from(*x))
        }

        let mut arg = vec![];
        for x in i.arg {
            arg.push(BigInt::from(x))
        }

        let from_seed = BigInt::from_str(&i.from_seed)?;
        let to_seed = BigInt::from_str(&i.to_seed)?;
        let step = BigInt::from(i.step);
        let step_after = BigInt::from(i.step_after);
        let from_board_packed = BigInt::from_str(&i.from_board_packed)?;
        let to_board_packed = BigInt::from_str(&i.to_board_packed)?;
        let score_packed = BigInt::from(i.score_packed);
        let pos_packed = BigInt::from_str(&i.pos_packed)?;
        let item_packed = BigInt::from_str(&i.item_packed)?;

        let mut maps = HashMap::new();
        maps.insert("fromSeed".to_string(), vec![from_seed]);
        maps.insert("toSeed".to_string(), vec![to_seed]);
        maps.insert("fromBoard".to_string(), from_board);
        maps.insert("toBoard".to_string(), to_board);
        maps.insert("step".to_string(), vec![step]);
        maps.insert("stepAfter".to_string(), vec![step_after]);
        maps.insert("fromBoardPacked".to_string(), vec![from_board_packed]);
        maps.insert("toBoardPacked".to_string(), vec![to_board_packed]);
        maps.insert("scorePacked".to_string(), vec![score_packed]);
        maps.insert("posPacked".to_string(), vec![pos_packed]);
        maps.insert("itemPacked".to_string(), vec![item_packed]);
        maps.insert("move".to_string(), moves);
        maps.insert("arg".to_string(), arg);

        Ok(Input { maps })
    }
}

#[cfg(test)]
mod test {
    use super::{CryptoRumbleInput, Game2048Input, Input};

    // cargo test --release --package zypher-circom-compat --lib -- crypto_rumble::input::test::convert_input --exact --show-output
    #[test]
    fn test_2048_input() {
        let input = r##"
        {
            "board": [
                [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 2, 4, 6, 0, 1, 2, 4, 0, 0, 0, 5, 0, 0, 1, 3]
            ],
            "packed_board": [35218731827200, 2515675923718842875939],
            "packed_dir": 311800516178808354245949615821275955,
            "direction": [0, 3, 3, 0, 0, 0, 3, 0, 3, 3, 0, 3, 3, 0, 3, 0, 2, 0, 3, 3, 0, 2, 0, 3, 0, 0, 3, 0, 2, 0, 3, 3, 0, 0, 3, 0, 3, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 2, 3, 3, 0, 3, 3, 0, 0, 3, 0, 3, 0, 3],
            "address": "6789",
            "step": 0,
            "step_after": 60,
            "nonce": "456"
        }
        "##;

        let input: Game2048Input = serde_json::from_str(input).unwrap();
        let _ = Input::try_from(input).unwrap();
    }

    #[test]
    fn test_cr_input() {
        let input = r##"
        {
            "from_seed": "16938986816621673014406792984620325385232245869428348395053494538472250137768",
            "to_seed": "18809534718515133310982073931212903285152506282303066166330452480033125747936",
            "from_board": [
                [2, 5, 2, 3, 3, 4],
                [4, 4, 5, 1, 4, 3],
                [1, 2, 4, 5, 2, 3],
                [1, 4, 2, 3, 5, 1],
                [2, 3, 2, 1, 5, 3],
                [1, 3, 3, 2, 2, 5]
            ],
            "to_board": [
                [5, 2, 2, 3, 4, 5],
                [4, 2, 2, 5, 1, 1],
                [2, 3, 5, 2, 3, 4],
                [5, 1, 2, 2, 4, 3],
                [2, 5, 3, 3, 2, 1],
                [3, 2, 2, 1, 4, 2]
            ],
            "step": 0,
            "step_after": 19,
            "from_board_packed": "103361923205923181585452685177869704657870687575312453",
            "to_board_packed": "242543694228480640306188505874996485086797052824847490",
            "score_packed": 387165653630999,
            "pos_packed": "407069173718415000365340272682837370791232631116922880",
            "item_packed": "13803492696795028375627839078134363494882806125467409972850288729522176",
            "moves": [
                [2, 1, 2],
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
                [4, 0, 2],
                [0, 0, 0],
                [1, 3, 1],
                [3, 4, 2],
                [2, 4, 2],
                [1, 5, 1],
                [0, 0, 0],
                [0, 0, 0],
                [0, 2, 1],
                [2, 2, 2],
                [0, 1, 1],
                [0, 0, 0],
                [0, 3, 1],
                [0, 1, 1],
                [0, 3, 1],
                [0, 3, 2],
                [0, 3, 1],
                [0, 1, 1],
                [0, 2, 1],
                [2, 2, 2],
                [0, 0, 0],
                [0, 2, 1],
                [0, 5, 1],
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0]
            ],
            "arg": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }
        "##;

        let input: CryptoRumbleInput = serde_json::from_str(input).unwrap();
        let _ = Input::try_from(input).unwrap();
    }
}
