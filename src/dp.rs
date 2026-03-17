// #![warn(missing_docs)]
// //!
// //!

mod cell_model_2d;
mod dp_model_2d;
mod lattice_model_2d;
mod run_simulation_2d;
mod simulation_2d;
use crate::parameters::{DPState, Parameters, Processing};

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> (usize, Vec<Vec<DPState>>, Vec<Vec<f64>>, f64) {
    params.print();
    let (t_run_time, n_lattices, lattices, tracking) = match params.processing {
        Processing::Serial => run_simulation_2d::run_simulation(&params, &Processing::Serial),
        Processing::Parallel => run_simulation_2d::run_simulation(&params, &Processing::Parallel),
    };
    match params.processing {
        Processing::Serial => println!(
            "Simulation run time (serial processing): {:4.3}s",
            t_run_time
        ),
        Processing::Parallel => println!(
            "Simulation run time (parallel processing): {:4.3}s",
            t_run_time
        ),
    };

    (n_lattices, lattices, tracking, t_run_time)
}
