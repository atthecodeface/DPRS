//! Documentation of Domany Kinzel models
//!
//!
mod traits;
pub use traits::{Cell1D, Cell2D, Cell3D, CellModel};

mod nbrhood_2d;
mod nbrhood_3d;
mod growth_1d;
mod growth_2d;
mod growth_3d;
mod lattice_1d;
mod lattice_2d;
mod lattice_3d;

#[cfg(test)]
mod tests;

pub use nbrhood_2d::{CellNbrhood2D, RowIterator2D};
pub use nbrhood_3d::{CellNbrhood3D, RowIterator3D};
pub use growth_1d::{DKSimplified1D, DKStaggered1D};
pub use growth_2d::{DKSimplified2D, DKStaggered2D};
pub use growth_3d::DKSimplified3D;

pub use lattice_1d::LatticeModel1D;
pub use lattice_2d::LatticeModel2D;
pub use lattice_3d::LatticeModel3D;
