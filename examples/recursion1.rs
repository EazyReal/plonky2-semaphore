// HACK: Ideally this would live in `benches/`, but `cargo bench` doesn't allow
// custom CLI argument parsing (even with harness disabled). We could also have
// put it in `src/bin/`, but then we wouldn't have access to
// `[dev-dependencies]`.
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use plonky2::field::goldilocks_field::GoldilocksField;
//use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::Proof;

use plonky2::field::field_types::Field;
use plonky2::hash::merkle_tree::MerkleTree;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::plonk::config::Hasher;
use plonky2_semaphore::access_set::AccessSet;
//use plonky2_semaphore::recursion::aggregate_n_signals;

use anyhow::Result;

pub type F = GoldilocksField;
pub type Digest = [F; 4];
pub type C = PoseidonGoldilocksConfig;
pub type PlonkyProof = Proof<F, PoseidonGoldilocksConfig, 2>;

#[derive(Debug, Clone)]
pub struct Signal {
    pub nullifier: Digest,
    pub proof: PlonkyProof,
}

fn main() -> Result<()> {
    let n = 1 << 20;
    let private_keys: Vec<Digest> = (0..n).map(|_| F::rand_arr()).collect();
    let public_keys: Vec<Vec<F>> = private_keys
        .iter()
        .map(|&sk| {
            PoseidonHash::hash_no_pad(&[sk, [F::ZERO; 4]].concat())
                .elements
                .to_vec()
        })
        .collect();
    let access_set = AccessSet(MerkleTree::new(public_keys, 0));

    let topic = F::rand_arr();
    let (signal, vd) = access_set.make_signal(private_keys[0], topic, 0)?;
    println!("before recursion");
    let (_, recursive_proof) = access_set.recursive_proof(topic, signal, &vd);
    let proof_len = recursive_proof.to_bytes()?.len();
    println!("{}", proof_len);
    Ok(())
}
