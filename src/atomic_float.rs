#![allow(dead_code)]
use std::{ f32::consts::TAU, sync::atomic::{ AtomicU64, Ordering } };
pub struct AtomicF64(AtomicU64);

impl AtomicF64 {
    pub fn new(initial: f64) -> Self {
        Self(AtomicU64::new(initial.to_bits()))
    }
    pub fn store(&self, v: f64, order: Ordering) {
        self.0.store(v.to_bits(), order)
    }
    pub fn load(&self, order: Ordering) -> f64 {
        f64::from_bits(self.0.load(order))
    }
    pub fn fetch_increment_mod_of_tau(&self, num: f64, order: Ordering) -> f64 {
        let prev = self.0.update(order, order, |val|
            ((f64::from_bits(val) + num) % (TAU as f64)).to_bits()
        );

        f64::from_bits(prev)
    }
}
