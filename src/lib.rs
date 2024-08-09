use anyhow::{anyhow, Error};
use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomBuilder, CircomConfig, CircomReduction, WitnessCalculator, circom::{R1CS, R1CSFile}};
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_snark::SNARK;
use num_bigint::BigInt;
use once_cell::sync::OnceCell;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::{collections::HashMap, fs::File};
use wasmer::{Module, Store};
use std::io::{BufReader, Cursor};

type GrothBn = Groth16<Bn254, CircomReduction>;

pub struct Input {
    pub maps: HashMap<String, Vec<BigInt>>,
}

static CONFIG: OnceCell<(CircomConfig<Bn254>, ProvingKey<Bn254>)> = OnceCell::new();

pub fn init_config(wasm: &str, r1cs: &str, zkey: &str) {
    let cfg = CircomConfig::<Bn254>::new(wasm, r1cs).unwrap();

    let mut zkey_file = File::open(zkey).unwrap();
    let (prover_key, _) = ark_circom::read_zkey(&mut zkey_file).unwrap();

    CONFIG.set((cfg, prover_key)).unwrap();
}

pub fn init_from_bytes(wasm: &[u8], r1cs: &[u8], zkey: &[u8]) {
    let store = Store::default();
    let module = Module::new(&store, wasm).unwrap();

    let reader = BufReader::new(Cursor::new(r1cs));
    let r1cs_file = R1CSFile::<Bn254>::new(reader).unwrap();

    let cfg = CircomConfig {
        wtns: WitnessCalculator::from_module(module).unwrap(),
        r1cs: R1CS::from(r1cs_file),
        sanity_check: false,
    };

    let mut zkey_reader = BufReader::new(Cursor::new(zkey));
    let (prover_key, _) = ark_circom::read_zkey(&mut zkey_reader).unwrap();

    CONFIG.set((cfg, prover_key)).unwrap();
}

pub fn prove(input: Input) -> Result<(Vec<Fr>, Proof<Bn254>), Error> {
    let (cfg, prover_key) = CONFIG
        .get()
        .ok_or_else(|| anyhow!("Failed to get circom config"))?;

    let mut builder = CircomBuilder::new(cfg.clone());
    builder.push_inputs(input.maps);

    let circom = builder.build().map_err(|_| anyhow!("Failed to build"))?;
    let pi = circom
        .get_public_inputs()
        .ok_or_else(|| anyhow!("Failed to get public inputs"))?;

    let mut rng = ChaChaRng::from_entropy();
    let proof = GrothBn::prove(prover_key, circom, &mut rng)?;

    Ok((pi, proof))
}
