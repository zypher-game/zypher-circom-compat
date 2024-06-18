use std::time::Instant;

use super::input::Input;
use crate::{get_config, Game};
use anyhow::{anyhow, Error};
use ark_bn254::Bn254;
use ark_circom::CircomBuilder;
use ark_groth16::{Groth16, Proof};
use ark_snark::SNARK;
use rand::thread_rng;

type GrothBn = Groth16<Bn254>;

pub fn prove<T: TryInto<Input, Error = Error>>(
    game: &Game,
    input: T,
) -> Result<Proof<Bn254>, Error> {
    let total_start = Instant::now();
    let internal_input = input.try_into()?;

    let start = Instant::now();
    let cfg = get_config(game).ok_or_else(|| anyhow!("Failed to get configuration"))?;
    println!("cfg time:{:.2?}", start.elapsed());

    let mut builder = CircomBuilder::new(cfg);
    builder.push_inputs(internal_input.maps);

    let circom = builder.setup();

    let start = Instant::now();
    // todo setup
    let mut rng = thread_rng();
    let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)?;
    println!("setup time:{:.2?}", start.elapsed());

    let start = Instant::now();
    let circom = builder.build().map_err(|_| anyhow!("Failed to build"))?;
    println!("build time:{:.2?}", start.elapsed());

    let inputs = circom
        .get_public_inputs()
        .ok_or_else(|| anyhow!("Failed to get inputs"))?;

    let start = Instant::now();
    let proof = GrothBn::prove(&params, circom, &mut rng)?;
    println!("prove time:{:.2?}", start.elapsed());

    {
        // todo remevoe
        // let pvk = GrothBn::process_vk(&params.vk).unwrap();
        // let verified = GrothBn::verify_with_processed_vk(&pvk, &inputs, &proof).unwrap();
        // assert!(verified);
    }

    println!("total time:{:.2?}", total_start.elapsed());

    //  public input
    Ok(proof)
}

#[cfg(test)]
mod test {
    use crate::{
        init_config,
        input::{CryptoRumbleInput, Game2048Input},
        Game,
    };

    use super::prove;

    // cargo test --release --package zypher-circom-compat --lib -- prove::test::test_2048 --exact --show-output
    #[test]
    fn test_2048() {
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

        init_config();
        let _ = prove(&Game::GAME2048, input).unwrap();
    }

    // cargo test --release --package zypher-circom-compat --lib -- prove::test::test_crypto_rumble --exact --show-output
    #[test]
    fn test_crypto_rumble() {
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

        init_config();
        let _ = prove(&Game::CRYPTORUMBLE, input).unwrap();
    }
}
