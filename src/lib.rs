use anyhow::{anyhow, Error};
use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomBuilder, CircomConfig};
use ark_groth16::{Groth16, Proof};
use ark_snark::SNARK;
use num_bigint::BigInt;
use once_cell::sync::OnceCell;
use rand::thread_rng;
use std::collections::HashMap;

type GrothBn = Groth16<Bn254>;

pub struct Input {
    pub maps: HashMap<String, Vec<BigInt>>,
}

static CONFIG: OnceCell<CircomConfig<Bn254>> = OnceCell::new();

pub fn init_config(wasm: &str, r1cs: &str) {
    let cfg = CircomConfig::<Bn254>::new(wasm, r1cs).unwrap();
    CONFIG.set(cfg).unwrap();
}

pub fn prove<T: TryInto<Input, Error = Error>>(input: T) -> Result<(Vec<Fr>, Proof<Bn254>), Error> {
    let cfg = CONFIG
        .get()
        .ok_or_else(|| anyhow!("Failed to get circom config"))?;

    let mut builder = CircomBuilder::new(cfg.clone());
    let builder_input = input.try_into()?;
    builder.push_inputs(builder_input.maps);

    // TODO: setup
    let circom = builder.setup();
    let mut rng = thread_rng();
    let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)?;

    let circom = builder.build().map_err(|_| anyhow!("Failed to build"))?;
    let pi = circom
        .get_public_inputs()
        .ok_or_else(|| anyhow!("Failed to get public inputs"))?;

    let proof = GrothBn::prove(&params, circom, &mut rng)?;

    Ok((pi, proof))
}
