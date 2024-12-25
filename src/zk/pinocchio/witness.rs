use crate::exlib::field::{
  prime_field_elem::PrimeFieldElem,
  sparse_vec::SparseVec,
};

pub struct Witness {
  sv: SparseVec,  // includes witness value for `1`
  pub mid_beg: PrimeFieldElem,
  pub end: PrimeFieldElem,
}

impl Witness {
  pub fn new(sv: &SparseVec, mid_beg: &PrimeFieldElem) -> Self {
    Witness {
      sv: sv.clone(),
      mid_beg: mid_beg.clone(),
      end: &sv.size - sv.f.elem(&1u8),
    }
  }

  pub fn io(&self) -> SparseVec {
    let f = &self.mid_beg.f;
    self.sv.slice(&f.elem(&0u8), &self.mid_beg)
  }

  pub fn mid(&self) -> SparseVec {
    self.sv.slice(&self.mid_beg, &self.sv.size)
  }
}














