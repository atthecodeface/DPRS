// #![warn(missing_docs)]
// //!
// //!

use super::growth_model_1d::GrowthModel1D;
use crate::dk::lattice_model_1d;
use crate::parameters::{
    BoundaryCondition, Dimension, DualState, GrowthModel, SimParameters, Topology,
};
use crate::parameters::{InitialCondition, Parameters, Processing};
use lattice_model_1d::LatticeModel1D;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration.
pub fn simulation(parameters: &SimParameters) -> (usize, Vec<Vec<DualState>>, Vec<Vec<f64>>) {
    let pad: usize = match parameters.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = parameters.n_x;
    let n_x: usize = pruned_n_x + pad * 2;

    // Growth model and its parameters
    let mut growth_model =
        GrowthModel1D::new(parameters.p_1, parameters.p_2, parameters.p_initial, 0);
    // Lattice model and its parameters
    let mut lm = LatticeModel1D::new(
        growth_model,
        n_x,
        (DualState::Empty, DualState::Empty),
        parameters.axis_topology_x.clone(),
        parameters.axis_bcs_x.clone(),
        parameters.axis_bc_values_x.clone(),
        parameters.do_edge_buffering,
    );

    let mut rng = StdRng::seed_from_u64(parameters.random_seed as u64);
    match parameters.initial_condition {
        InitialCondition::Randomized => {
            lm.create_randomized_lattice(&mut rng);
        }
        InitialCondition::CentralSeed => {
            lm.create_seeded_lattice();
        }
        InitialCondition::Preserved => {}
    }
    lm.apply_edge_topology();
    lm.apply_boundary_conditions();

    // Set up a recording of lattice evolution, or suppress
    let n_iterations: usize = parameters.n_iterations;
    let sample_period: usize = parameters.sample_period;
    let n_lattices = match sample_period > 0 {
        true => n_iterations / sample_period + 1,
        false => 0,
    };
    let mut lattices = Vec::new();
    let mut tracking = Vec::new();
    let t_track = Vec::new();
    let rho_mean_track = Vec::new();
    // Record the initial lattice
    lattices.push(lm.lattice().clone());
    tracking.push(t_track);
    tracking.push(rho_mean_track);
    tracking[0].push(0.0);
    let rho_mean = lm.mean();
    tracking[1].push(rho_mean);
    // We aren't going to worry about the lattice type being Cell
    //  - instead we're going to leave it up to pyo3 to convert
    // the lattice vector into a Python list as it thinks fit.
    // This happens (magically) on exiting sim_dk() back to Python.

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_edge_topology" etc are unnecessary.
    // It's only there for now to ensure the t-sliced lattices show whether
    // boundary topology/condition step is working or not.
    match parameters.processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                // Probably should be an increment function
                growth_model.iteration += 1;
                lm.next_iteration_serial(&mut rng);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                if sample_period > 0 && i.is_multiple_of(sample_period) {
                    lattices.push(lm.lattice().clone());
                };
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
        }
        Processing::Parallel => {
            // Create a vector of RNGs of length n_y,
            // i.e., of length = number of lattice rows,
            // each seeded by parameters.random_seed + their index.
            // Each RNG element of this vec will be used,
            // one per row, to generate coin tosses for DP cell updates.
            // NB: this could be shortened by 2 (pad width) but we'll
            // keep it full length for now just in case we need buffer RNGs.
            assert!(parameters.random_seed > 0);
            let mut rngs: Vec<StdRng> = (0..parameters.n_y)
                .into_iter()
                .map(|s| StdRng::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
                .collect();
            for i in 1..(n_iterations + 1) {
                // Probably should be an increment function
                growth_model.iteration += 1;
                lm.next_iteration_parallel(&mut rngs);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                if sample_period > 0 && i.is_multiple_of(sample_period) {
                    lattices.push(lm.lattice().clone());
                };
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
        }
    };
    assert!(n_iterations == growth_model.iteration);
    assert!(n_lattices == 0 || n_lattices == lattices.len());

    (n_lattices, lattices, tracking)
}

// /// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
// #[derive(Clone, Debug)]
// pub struct Simulation1D {
//     pub growth_model: GrowthModel,
//     pub dim: Dimension,
//     pub n_x: usize,
//     pub n_y: usize,
//     pub n_z: usize,
//     pub p_1: f64,
//     pub p_2: f64,
//     pub n_iterations: usize,
//     pub sample_period: usize,
//     pub initial_condition: InitialCondition,
//     pub p_initial: f64,
//     pub random_seed: usize,
//     pub axis_topology_x: Topology,
//     pub axis_topology_y: Topology,
//     pub axis_topology_z: Topology,
//     pub axis_bcs_x: (BoundaryCondition, BoundaryCondition),
//     pub axis_bcs_y: (BoundaryCondition, BoundaryCondition),
//     pub axis_bcs_z: (BoundaryCondition, BoundaryCondition),
//     pub axis_bc_values_x: (bool, bool),
//     pub axis_bc_values_y: (bool, bool),
//     pub axis_bc_values_z: (bool, bool),
//     pub do_edge_buffering: bool,
//     pub processing: Processing,
//     pub n_threads: usize,
// }

// impl Simulation1D {
//     fn new(
//         growth_model: GrowthModel,
//         dim: Dimension,
//         n_x: usize,
//         n_y: usize,
//         n_z: usize,
//         p_1: f64,
//         p_2: f64,
//         n_iterations: usize,
//         sample_period: usize,
//         initial_condition: InitialCondition,
//         p_initial: f64,
//         random_seed: usize,
//         axis_topology_x: Topology,
//         axis_topology_y: Topology,
//         axis_topology_z: Topology,
//         axis_bcs_x: (BoundaryCondition, BoundaryCondition),
//         axis_bcs_y: (BoundaryCondition, BoundaryCondition),
//         axis_bcs_z: (BoundaryCondition, BoundaryCondition),
//         axis_bc_values_x: (bool, bool),
//         axis_bc_values_y: (bool, bool),
//         axis_bc_values_z: (bool, bool),
//         do_edge_buffering: bool,
//         processing: Processing,
//         n_threads: usize,
//     ) -> Self {
//         Self {
//             growth_model,
//             dim,
//             n_x,
//             n_y,
//             n_z,
//             p_1,
//             p_2,
//             n_iterations,
//             sample_period,
//             initial_condition,
//             p_initial,
//             random_seed,
//             axis_topology_x,
//             axis_topology_y,
//             axis_topology_z,
//             axis_bcs_x,
//             axis_bcs_y,
//             axis_bcs_z,
//             axis_bc_values_x,
//             axis_bc_values_y,
//             axis_bc_values_z,
//             do_edge_buffering,
//             processing,
//             n_threads,
//         }
//     }

//     /// Returns the number of lattices sampled, the sampled lattices, and tracking
//     /// which is a Vec with first entry a vec of iteration numbers and the second
//     /// entry a vec of mean density for the respective iteration.
//     pub fn execute(parameters: &Parameters) -> (usize, Vec<Vec<DualState>>, Vec<Vec<f64>>) {
//         let pad: usize = match parameters.do_edge_buffering {
//             true => 1,
//             false => 0,
//         };
//         let pruned_n_x = parameters.n_x;
//         let n_x: usize = pruned_n_x + pad * 2;

//         // Growth model and its parameters
//         let mut growth_model =
//             GrowthModel1D::new(parameters.p_1, parameters.p_2, parameters.p_initial, 0);
//         // Lattice model and its parameters
//         let mut lm = LatticeModel1D::new(
//             growth_model,
//             n_x,
//             (DualState::Empty, DualState::Empty),
//             parameters.axis_topology_x.clone(),
//             parameters.axis_bcs_x.clone(),
//             parameters.axis_bc_values_x.clone(),
//             parameters.do_edge_buffering,
//         );

//         let mut rng = StdRng::seed_from_u64(parameters.random_seed as u64);
//         match parameters.initial_condition {
//             InitialCondition::Randomized => {
//                 lm.create_randomized_lattice(&mut rng);
//             }
//             InitialCondition::CentralSeed => {
//                 lm.create_seeded_lattice();
//             }
//             InitialCondition::Preserved => {}
//         }
//         lm.apply_edge_topology();
//         lm.apply_boundary_conditions();

//         // Set up a recording of lattice evolution, or suppress
//         let n_iterations: usize = parameters.n_iterations;
//         let sample_period: usize = parameters.sample_period;
//         let n_lattices = match sample_period > 0 {
//             true => n_iterations / sample_period + 1,
//             false => 0,
//         };
//         let mut lattices = Vec::new();
//         let mut tracking = Vec::new();
//         let t_track = Vec::new();
//         let rho_mean_track = Vec::new();
//         // Record the initial lattice
//         lattices.push(lm.lattice().clone());
//         tracking.push(t_track);
//         tracking.push(rho_mean_track);
//         tracking[0].push(0.0);
//         let rho_mean = lm.mean();
//         tracking[1].push(rho_mean);
//         // We aren't going to worry about the lattice type being Cell
//         //  - instead we're going to leave it up to pyo3 to convert
//         // the lattice vector into a Python list as it thinks fit.
//         // This happens (magically) on exiting sim_dk() back to Python.

//         // Evolve the lattice for n_iterations
//         //
//         // Note: the second "apply_edge_topology" etc are unnecessary.
//         // It's only there for now to ensure the t-sliced lattices show whether
//         // boundary topology/condition step is working or not.
//         match parameters.processing {
//             Processing::Serial => {
//                 for i in 1..(n_iterations + 1) {
//                     // Probably should be an increment function
//                     growth_model.iteration += 1;
//                     lm.next_iteration_serial(&mut rng);
//                     lm.apply_edge_topology();
//                     lm.apply_boundary_conditions();
//                     if sample_period > 0 && i.is_multiple_of(sample_period) {
//                         lattices.push(lm.lattice().clone());
//                     };
//                     let t = i as f64;
//                     tracking[0].push(t);
//                     let rho_mean = lm.mean();
//                     tracking[1].push(rho_mean);
//                 }
//             }
//             Processing::Parallel => {
//                 // Create a vector of RNGs of length n_y,
//                 // i.e., of length = number of lattice rows,
//                 // each seeded by parameters.random_seed + their index.
//                 // Each RNG element of this vec will be used,
//                 // one per row, to generate coin tosses for DP cell updates.
//                 // NB: this could be shortened by 2 (pad width) but we'll
//                 // keep it full length for now just in case we need buffer RNGs.
//                 assert!(parameters.random_seed > 0);
//                 let mut rngs: Vec<StdRng> = (0..parameters.n_y)
//                     .into_iter()
//                     .map(|s| StdRng::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
//                     .collect();
//                 for i in 1..(n_iterations + 1) {
//                     // Probably should be an increment function
//                     growth_model.iteration += 1;
//                     lm.next_iteration_parallel(&mut rngs);
//                     lm.apply_edge_topology();
//                     lm.apply_boundary_conditions();
//                     if sample_period > 0 && i.is_multiple_of(sample_period) {
//                         lattices.push(lm.lattice().clone());
//                     };
//                     let t = i as f64;
//                     tracking[0].push(t);
//                     let rho_mean = lm.mean();
//                     tracking[1].push(rho_mean);
//                 }
//             }
//         };
//         assert!(n_iterations == growth_model.iteration);
//         assert!(n_lattices == 0 || n_lattices == lattices.len());

//         (n_lattices, lattices, tracking)
//     }
// }
