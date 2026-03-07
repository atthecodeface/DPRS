use std::time::{Duration, Instant};
use crate::sim_dp::LatticeModel;

/// Run a simulation and record how long the computation takes.
pub fn monitor(
    compute: fn(LatticeModel<2, 2, 2, 3>, usize) -> (), 
    n_x: usize, n_y: usize, n_iterations: usize,
) -> Duration {
    let time = Instant::now();
    let grid = LatticeModel::initialize(n_x, n_y).randomize();
    compute(grid, n_iterations);

    time.elapsed()
}