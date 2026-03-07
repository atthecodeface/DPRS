use crate::sim_dp::Domain;

/// Run a simulation for n_iterations using serial processing.
pub fn compute_serial(grid: Domain<2, 2, 2, 3>, n_iterations: usize) {
    let mut grid = grid;
    for _ in 0..n_iterations {
        grid = grid.next_iteration_serial();
    }
}

/// Run a simulation for n_iterations using parallel processing.
pub fn compute_parallel(grid: Domain<2, 2, 2, 3>, n_iterations: usize) {
    let mut grid = grid;
    for _ in 0..n_iterations {
        grid = grid.next_iteration_parallel();
    }
}