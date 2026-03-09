use std::time::Instant;
mod lattice_model_2d;
use lattice_model_2d::LatticeModel2D;
mod compute;
use compute::{compute_serial, compute_parallel};

#[derive(Clone)]
pub struct Parameters {
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub n_iterations: usize,
    pub slow_factor: usize,
    pub n_threads: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Dimension { D1, D2, D3, }

/// Entry point to this module.
pub fn sim_dp(p: Parameters) -> Vec<bool> {
    println!();
    println!("Dimension:   {:?}", p.dim);
    println!("Grid shape:  {:?}", (p.n_x, p.n_y, p.n_z));
    println!("Iterations:  n={}", p.n_iterations);
    println!("Slow factor: s={}", p.slow_factor);
    println!("Threads: n_threads={}\n", p.n_threads);

    let (t_serial, _,) = run_simulation(compute_serial, &p,);
    println!("Serial:   {:4.3}s", t_serial);
    
    let (t_parallel, lattice,) = run_simulation(compute_parallel, &p,);
    println!("Parallel: {:4.3}s", t_parallel);

    println!("Speedup => {:.2}x", t_serial/t_parallel);
    println!();

    lattice
}

/// Run a simulation and record how long the computation takes.
fn run_simulation(
    compute: fn(LatticeModel2D, usize) -> Vec<bool>, 
    p: &Parameters,
) -> (f64, Vec<bool>) {
    let grid = 
        LatticeModel2D::initialize(p.n_x, p.n_y).randomize();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(p.n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let lattice = 
        pool.install(|| compute(grid, p.n_iterations/p.slow_factor));
    let duration = time.elapsed().as_secs_f64() * (p.slow_factor as f64);

    (duration, lattice)
}