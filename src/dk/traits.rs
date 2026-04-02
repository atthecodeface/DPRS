use super::{CellModel1D, LatticeModel1D};
use super::{CellModel2D, LatticeModel2D};
use super::{CellModel3D, LatticeModel3D};

pub trait DramaticallySimulatable {
    fn mean(&self) -> f64;
}

// pub trait HasLattice: Sync {
//     fn lattice<T>(&self) -> &Vec<T>;
// }

impl<C: CellModel1D> DramaticallySimulatable for LatticeModel1D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }
}

impl<C: CellModel2D> DramaticallySimulatable for LatticeModel2D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }
}

impl<C: CellModel3D> DramaticallySimulatable for LatticeModel3D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }
}
