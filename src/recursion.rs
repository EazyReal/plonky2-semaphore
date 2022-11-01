use plonky2::fri::proof;
use plonky2::iop::witness::{PartialWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, VerifierCircuitData, VerifierCircuitTarget};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};

use crate::access_set::AccessSet;
use crate::signal::{Digest, PlonkyProof, Signal, C, F};

//use itertools::Itertools::zip_eq;

impl AccessSet {
    pub fn aggregate_signals(
        &self,
        topic0: Digest,
        signal0: Signal,
        topic1: Digest,
        signal1: Signal,
        verifier_data: &VerifierCircuitData<F, C, 2>,
    ) -> (Digest, Digest, PlonkyProof) {
        let config = CircuitConfig::standard_recursion_zk_config();
        let mut builder = CircuitBuilder::new(config);
        let mut pw = PartialWitness::new();

        let public_inputs0: Vec<F> = self
            .0
            .cap
            .0
            .iter()
            .flat_map(|h| h.elements)
            .chain(signal0.nullifier)
            .chain(topic0)
            .collect();
        let public_inputs1: Vec<F> = self
            .0
            .cap
            .0
            .iter()
            .flat_map(|h| h.elements)
            .chain(signal1.nullifier)
            .chain(topic1)
            .collect();

        let proof_target0 = builder.add_virtual_proof_with_pis(&verifier_data.common);
        pw.set_proof_with_pis_target(
            &proof_target0,
            &ProofWithPublicInputs {
                proof: signal0.proof,
                public_inputs: public_inputs0,
            },
        );
        let proof_target1 = builder.add_virtual_proof_with_pis(&verifier_data.common);
        pw.set_proof_with_pis_target(
            &proof_target1,
            &ProofWithPublicInputs {
                proof: signal1.proof,
                public_inputs: public_inputs1,
            },
        );

        let vd_target = VerifierCircuitTarget {
            constants_sigmas_cap: builder
                .add_virtual_cap(verifier_data.common.config.fri_config.cap_height),
        };
        pw.set_cap_target(
            &vd_target.constants_sigmas_cap,
            &verifier_data.verifier_only.constants_sigmas_cap,
        );

        builder.verify_proof(proof_target0, &vd_target, &verifier_data.common);
        builder.verify_proof(proof_target1, &vd_target, &verifier_data.common);

        let data = builder.build();
        let recursive_proof = data.prove(pw).unwrap();

        data.verify(recursive_proof.clone()).unwrap();

        (signal0.nullifier, signal1.nullifier, recursive_proof.proof)
    }

    pub fn aggregate_n_signals(
        &self,
        topics: Vec<Digest>,
        signals: Vec<Signal>,
        verifier_data: &VerifierCircuitData<F, C, 2>,
    ) -> (Vec<Digest>, ProofWithPublicInputs<F, C, 2>) {
        let config = CircuitConfig::standard_recursion_zk_config();
        let mut builder = CircuitBuilder::new(config);
        let mut pw = PartialWitness::new();
        let n1 = topics.len();
        let n2 = signals.len();
        assert_eq!(n1, n2);

        let mut proof_targets = Vec::new();
        let mut nullifiers = Vec::new();

        // todo: how to loop through 2 vectors at the same time and "with ownership"
        let mut i = 0;
        for (topic, signal) in topics.into_iter().zip(signals.into_iter()) {
            println!("{}, {:?}", i, topic);
            i += 1;
            let public_inputs: Vec<F> = self
                .0
                .cap
                .0
                .iter()
                .flat_map(|h| h.elements)
                .chain(signal.nullifier)
                .chain(topic)
                .collect();
            let proof_target = builder.add_virtual_proof_with_pis(&verifier_data.common);
            pw.set_proof_with_pis_target(
                &proof_target,
                &ProofWithPublicInputs {
                    proof: signal.proof,
                    public_inputs: public_inputs,
                },
            );

            proof_targets.push(proof_target);
            nullifiers.push(signal.nullifier);
        }

        let vd_target = VerifierCircuitTarget {
            constants_sigmas_cap: builder
                .add_virtual_cap(verifier_data.common.config.fri_config.cap_height),
        };
        pw.set_cap_target(
            &vd_target.constants_sigmas_cap,
            &verifier_data.verifier_only.constants_sigmas_cap,
        );

        for proof_target in proof_targets {
            builder.verify_proof(proof_target, &vd_target, &verifier_data.common);
        }

        let data = builder.build();
        let recursive_proof = data.prove(pw).unwrap();

        data.verify(recursive_proof.clone()).unwrap();

        (nullifiers, recursive_proof)
    }
}
