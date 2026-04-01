// #![warn(missing_docs)]
// //!
// //!

use crate::sim_parameters::DualState;

pub type LatticeSlices = Vec<Vec<DualState>>;
pub type TrackingHistory = Vec<Vec<f64>>;

#[derive(Debug)]
pub struct LatticeHistory {
    pub lattice_history: LatticeSlices,
}

impl Default for LatticeHistory {
    fn default() -> Self {
        Self {
            lattice_history: Vec::new(),
        }
    }
}

impl LatticeHistory {
    pub fn record(&mut self, lattice: &Vec<DualState>, i: usize, sample_period: usize) {
        if sample_period > 0 && i.is_multiple_of(sample_period) {
            self.lattice_history.push(lattice.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.lattice_history.len()
    }

    pub fn take(self) -> LatticeSlices {
        self.lattice_history
    }
}
