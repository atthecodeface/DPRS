use super::CellNbrhood3D;

use super::LatticeModel1D;
use super::LatticeModel2D;
use super::LatticeModel3D;
use crate::sim_parameters::{
    DualState, GrowthModelChoice, InitialCondition, Processing, SimParameters,
};

use super::GrowthModel1D;

// #![warn(missing_docs)]
// //!
// //!

use rand::Rng;

pub trait CellDim {
    /// The number of dimensions (1, 2 or 3)
    const N: usize;

    /// The neighborhood type for cells
    type Nbrhood;
}

/// Marker type for 1d-simulation of cells with on/off state (boolean)
pub struct Cell1D();

/// Implementation of CellDim to let it be used for simulations
impl CellDim for Cell1D {
    const N: usize = 1;
    type Nbrhood = [bool; 3];
}

/// Marker type for 2d-simulation of cells with on/off state (boolean)
pub struct Cell2D();

/// Implementation of CellDim to let it be used for simulations
impl CellDim for Cell2D {
    const N: usize = 2;
    type Nbrhood = [bool; 9];
}

/// Marker type for 2d-simulation of cells with on/off state (boolean)
pub struct Cell3D();

/// Implementation of CellDim to let it be used for simulations
impl CellDim for Cell3D {
    const N: usize = 3;
    type Nbrhood = CellNbrhood3D;
}

/// The trait required for a model
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working.
pub trait CellModel<Dim: CellDim>: Sync {
    fn next_iteration(&mut self);
    fn iteration(&self) -> usize;
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> DualState;
    fn update_state<R: Rng>(&self, rng: &mut R, nbrhood: &Dim::Nbrhood) -> DualState;
}

pub trait DramaticallySimulatable<D: CellDim>: Sized {
    fn mean(&self) -> f64;
    fn create_from_parameters(_parameters: &SimParameters) -> Result<Self, ()> {
        Err(())
    }
    //pub fn lattice(&self) -> &[DualStaetVec<C::State> {
    //         &self.lattice
    //    }

    fn iteration(&self) -> usize {
        0
    }
    fn lattice(&self) -> &[DualState];
    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {}
    fn create_seeded_lattice(&mut self) {}
    fn apply_edge_topology(&mut self) {}
    fn apply_boundary_conditions(&mut self) {}
    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R) {}
    fn iterate_once_parallel<R: Rng + Send>(&mut self, rng: &mut [R]) {}
}

// pub trait HasLattice: Sync {
//     fn lattice<T>(&self) -> &Vec<T>;
// }
impl DramaticallySimulatable<Cell1D> for LatticeModel1D<GrowthModel1D> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self
            .lattice()
            .iter()
            .map(|s| {
                let u: usize = (*s).into();
                u
            })
            .sum();

        (total as f64) / (self.n_cells() as f64)
    }
    fn iteration(&self) -> usize {
        self.cell_model.iteration
    }
    fn lattice(&self) -> &[DualState] {
        self.lattice()
    }
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        // Growth model and its parameters
        let do_staggered = match parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => false,
            GrowthModelChoice::StaggeredDomanyKinzel => true,
            _ => todo!(),
        };
        let mut growth_model = GrowthModel1D::new(
            parameters.p_1,
            parameters.p_2,
            parameters.p_initial,
            0,
            do_staggered,
        );
        // Lattice model and its parameters
        let mut lm = LatticeModel1D::new(
            growth_model,
            parameters.n_x_with_pad(),
            (DualState::Empty, DualState::Empty),
            parameters.growth_model_choice,
            parameters.axis_topology_x,
            parameters.axis_bcs_x,
            parameters.axis_bc_values_x,
            parameters.do_edge_buffering,
        );
        Ok(lm)
    }
    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {}
    fn create_seeded_lattice(&mut self) {
        self.create_seeded_lattice();
    }
    fn apply_edge_topology(&mut self) {
        self.apply_edge_topology();
    }
    fn apply_boundary_conditions(&mut self) {
        self.apply_boundary_conditions();
    }
    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R) {
        self.cell_model.increment();
        self.next_iteration_serial(rng);
    }
    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.cell_model.increment();
        self.next_iteration_parallel(rngs);
    }
}

impl<C: CellModel<Cell2D>> DramaticallySimulatable<Cell2D> for LatticeModel2D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self
            .lattice()
            .iter()
            .map(|s| {
                let u: usize = (*s).into();
                u
            })
            .sum();

        (total as f64) / (self.n_cells() as f64)
    }
    fn lattice(&self) -> &[DualState] {
        self.lattice()
    }
}

impl<C: CellModel<Cell3D>> DramaticallySimulatable<Cell3D> for LatticeModel3D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self
            .lattice()
            .iter()
            .map(|s| {
                let u: usize = (*s).into();
                u
            })
            .sum();

        (total as f64) / (self.n_cells() as f64)
    }
    fn lattice(&self) -> &[DualState] {
        self.lattice()
    }
}
