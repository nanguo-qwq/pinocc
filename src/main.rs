use Pinocchio::{
    building_block::{
        field::{
            prime_field::PrimeField,
            prime_field_elem::PrimeFieldElem,
        },
        curves::{
            bls12_381::g1_point::G1Point,
        }
    },
    zk::{
        qap::{
            term::Term,
            term::Term::{Out, TmpVar},
            equation_parser::EquationParser,
        },
        pinocchio::{
            crs::CRS,
            proof::Proof,
            prover::Prover,
            verifier::Verifier,
            witness::Witness,
        }

    }
};
use std::collections::HashMap;

fn main() {

    let f = &G1Point::curve_group();
    let expr = "x * x * x + x + 10== 40";

    let eq = EquationParser::parse(&f, expr).unwrap();
    let witness_map = {
        HashMap::<Term, PrimeFieldElem>::from([
            (Term::One, f.elem(&1u8)),
            (Term::var("x"), f.elem(&3u8)),
            (TmpVar(1), f.elem(&9u8)),
            (TmpVar(2), f.elem(&27u8)),
            (TmpVar(3), f.elem(&13u8)),
            (TmpVar(4), f.elem(&40u8)),
            (Out, eq.rhs),
        ])
    };

    let p = Prover::new(&f, expr, &witness_map);
    let crs = CRS::new(&f, &p);

    //println!("Creating Prover and generating proof");
    let proof = p.prove(&crs);

    //println!("Verifying proof");
    let verifier = Verifier::new();
    let witness_io = p.witness.io();
    let is_valid = verifier.verify(&proof, &crs, &witness_io);

    println!("\nProof is valid: {}\n", is_valid);

    assert!(is_valid, "The proof is invalid!");

}