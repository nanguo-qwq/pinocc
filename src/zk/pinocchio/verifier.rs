use crate::{
  exlib::{
    field::sparse_vec::SparseVec,
    curves::bls12_381::{
      g1_point::G1Point,
      g2_point::G2Point,
      pairing::Pairing,
    },
  },
  zk::pinocchio::{
    crs::CRS,
    proof::Proof,
  },
};

pub struct Verifier {
  pairing: Pairing,
}

impl Verifier {
  pub fn new() -> Self {
    let pairing = Pairing::new();

    Verifier {
      pairing,
    }
  }

  pub fn verify(
    &self,
    proof: &Proof,
    crs: &CRS,
    witness_io: &SparseVec,
  ) -> bool {
    let e = |a: &G1Point, b: &G2Point| self.pairing.tate(a, b);

    let (p, vk) = (&proof, &crs.vk); 

    //v * w * y
    {
      let vwy_mid_s = &p.v_mid_s + &p.g1_w_mid_s + &p.y_mid_s;
      let lhs = e(&p.beta_vwy_mid_s, &vk.gamma);
      let rhs = e(&vwy_mid_s, &vk.beta_gamma);
      if lhs != rhs { return false; }
      println!("successful beta check");
    }

    //v, w and y
    {
      let lhs = e(&p.alpha_v_mid_s, &vk.one_g2);
      let rhs = e(&p.v_mid_s, &vk.alpha_v); 
      if lhs != rhs { return false; }
    }
    {
      let lhs = e(&p.alpha_w_mid_s, &vk.one_g2);
      let rhs = e(&vk.alpha_w, &p.g2_w_mid_s); 
      if lhs != rhs { return false; }
    }
    {
      let lhs = e(&p.alpha_y_mid_s, &vk.one_g2);
      let rhs = e(&p.y_mid_s, &vk.alpha_y); 
      if lhs != rhs { return false; }
      println!("successful alpha check");
    }

    //QAP
    {
      let mut v_s = p.v_mid_s.clone();
      let mut w_s = p.g2_w_mid_s.clone();
      let mut y_s = p.y_mid_s.clone();

      for i in 0..witness_io.size_in_usize() {
        let w = &witness_io[&i];
        v_s += &vk.vk_io[i] * w;
        w_s += &vk.wk_io[i] * w;
        y_s += &vk.yk_io[i] * w;
      }

      let lhs = e(&v_s, &w_s);
      let rhs = e(&vk.t, &p.h_s) * e(&y_s, &vk.one_g2);
      if lhs == rhs { println!("successful QAP check"); }
      lhs == rhs
    }
  }
}

