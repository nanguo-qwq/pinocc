use Pinocchio::{
    building_block::{
        field::{
            prime_field::PrimeField,
            prime_field_elem::PrimeFieldElem,
        }

    },
    zk::{
        qap::{
            term::Term,
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
use std::ptr::replace;
use num_bigint::BigUint;
use num_traits::Num;
use Pinocchio::zk::qap::equation_parser::EquationParser;
use Pinocchio::zk::qap::gate::Gate;
use Pinocchio::zk::qap::term::Term::{Out, TmpVar};

fn main() {
    println!("Initializing PrimeField and CRS...");
    let p_hex = "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab";
    let p = BigUint::from_str_radix(p_hex, 16).unwrap(); // 使用BLS12-381的素数p
    let f = PrimeField::new(&p);
    // 创建Prover实例需要的参数
    let expr = "(x * x * x) + x + 5 == 35";
    /*
    let input = "3";
    let expr2 = expr.replace("x",input);
    println!("{}", expr2);
    let eq = EquationParser::parse(&f, expr).unwrap();
    let gates = &Gate::build(&f, &eq);
    */
    let eq = EquationParser::parse(&f, expr).unwrap();
    let witness_map = {
        HashMap::<Term, PrimeFieldElem>::from([
            (Term::One, f.elem(&1u8)),
            (Term::var("x"), f.elem(&3u8)),
            (TmpVar(1), f.elem(&9u8)),
            (TmpVar(2), f.elem(&27u8)),
            (TmpVar(3), f.elem(&8u8)),
            (TmpVar(4), f.elem(&35u8)),
            (Out, eq.rhs),
        ])
    };

    let p = Prover::new(&f, expr, &witness_map);
    let crs = CRS::new(&f, &p);

    println!("Creating Prover and generating proof...");
    let proof = p.prove(&crs);

    // Step 3: 使用验证者（Verifier）来验证生成的证明
    println!("Verifying proof...");
    let verifier = Verifier::new();
    let witness_io = p.witness.io();
    let is_valid = verifier.verify(&proof, &crs, &witness_io);
    assert!(is_valid, "The proof is invalid!");

    // Step 4: 验证见证（Witness）的扩展
    println!("Extending witness...");
    let extended_witness = Witness::new(&witness_io, &p.witness.mid_beg);
    println!("Witness has been extended.");

    println!("All tests passed!");
}