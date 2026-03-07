use pyo3::prelude::*;
use std::iter::repeat;
use std::time::{Duration, Instant};
use rand::distr::StandardUniform;
use rand::{RngExt, rng};
use rayon::prelude::*;

/// Model domain (grid plus metadata container) in 2d.
/// 
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=alive) stored as a linear vector; 
/// birth and surival rules as length-1 and length-2 vectors.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Domain<
    const MIN_BORN: usize,
    const MAX_BORN: usize,
    const MIN_SURVIVE: usize,
    const MAX_SURVIVE: usize,
> {
    n_x: usize,
    n_y: usize,
    lattice: Vec<bool>,
}

/// Grid methods.
impl<
    const MIN_BORN: usize,
    const MAX_BORN: usize,
    const MIN_SURVIVE: usize,
    const MAX_SURVIVE: usize,
> Domain<MIN_BORN, MAX_BORN, MIN_SURVIVE, MAX_SURVIVE>
{
    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn initialize(n_x: usize, n_y: usize,) -> Self {
        let new_lattice = repeat(false).take(n_x * n_y).collect();

        Self {
            n_x,
            n_y,
            lattice: new_lattice,
        }
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.n_x * self.n_y
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomize(&self) -> Self {
        let new_lattice = rng()
            .sample_iter(&StandardUniform)
            .take(self.n_cells())
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .map(|i_cell| self.is_successor_cell(i_cell))
            .collect();

        self.next_grid(new_lattice)
    }

    // /// Evolve the grid by one iteration using parallel processing.
    // pub fn next_iteration_parallel(&self) -> Self {
    //     let mut new_lattice = vec![false; self.lattice.len()];
    //     new_lattice
    //         .par_chunks_mut(self.n_x)
    //         .enumerate()
    //         .for_each(|(r, l)| {
    //             for (c, lc) in l.iter_mut().enumerate() {
    //                 *lc = self.will_succeed(r, c);
    //             }
    //         });
    //     self.next_grid(new_lattice)
    // }

    /// Evolve the grid by one iteration using parallel processing.
    pub fn next_iteration_parallel(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .into_par_iter()
            .map(|i_cell| self.is_successor_cell(i_cell))
            .collect();

        self.next_grid(new_lattice)
    }

    /// Create the next grid with the assigned lattice vector and previous rules.
    fn next_grid(&self, new_lattice: Vec<bool>) -> Self {
        assert!(new_lattice.len() == self.n_cells());

        Self {
            n_x: self.n_x,
            n_y: self.n_y,
            lattice: new_lattice,
        }
    }

    /// Check that this i_th cell -> cell(x,y) is a successor cell
    fn is_successor_cell(&self, i_cell: usize) -> bool {
        self.will_succeed(i_cell % self.n_x, i_cell / self.n_x)
    }

    /// Decide if this (x,y) cell, if alive, survives or gives birth,
    /// i.e., if it will "succeed" – if so, return true.
    fn will_succeed(&self, x: usize, y: usize) -> bool {
        let n_alive_neighbors = self.n_alive_neighbors(x, y);

        if self.is_alive(x, y) {
            (MIN_SURVIVE..=MAX_SURVIVE).contains(&n_alive_neighbors)
        } else {
            (MIN_BORN..=MAX_BORN).contains(&n_alive_neighbors)
        }
    }

    /// Count how many neighboring cells are alive.
    fn n_alive_neighbors(&self, x_0: usize, y_0: usize) -> usize {
        let xp1 = x_0 + 1;
        let yp1 = y_0 + 1;
        let xm1 = x_0.wrapping_sub(1);
        let ym1 = y_0.wrapping_sub(1);
        let neighbors = [
            self.is_alive(xm1, ym1),
            self.is_alive(x_0, ym1),
            self.is_alive(xp1, ym1),
            self.is_alive(xm1, y_0),
            self.is_alive(xp1, y_0),
            self.is_alive(xm1, yp1),
            self.is_alive(x_0, yp1),
            self.is_alive(xp1, yp1),
        ];

        neighbors.iter().filter(|&x| *x).count()
    }

    /// Check if this cell is within bounds and alive
    fn is_alive(&self, x: usize, y: usize) -> bool {
        // check (x,y) coordinate is within bounds
        !(x >= self.n_x || y >= self.n_y) 
        // and if the cell is occupied
        && self.lattice[y * self.n_x + x]
    }
}

/// Run a simulation for n_iterations using serial processing.
fn compute_serial(grid: Domain<2, 2, 2, 3>, n_iterations: usize) {
    let mut grid = grid;
    for _ in 0..n_iterations {
        grid = grid.next_iteration_serial();
    }
}

/// Run a simulation for n_iterations using parallel processing.
fn compute_parallel(grid: Domain<2, 2, 2, 3>, n_iterations: usize) {
    let mut grid = grid;
    for _ in 0..n_iterations {
        grid = grid.next_iteration_parallel();
    }
}

/// Run a simulation and record how long the computation takes.
fn monitor(
    compute: fn(Domain<2, 2, 2, 3>, usize) -> (), 
    n_x: usize, n_y: usize, n_iterations: usize,
) -> Duration {
    let time = Instant::now();
    let grid = Domain::initialize(n_x, n_y).randomize();
    compute(grid, n_iterations);

    time.elapsed()
}


/// Entry point to this module.
pub fn run(n_x: usize, n_y: usize, n_iterations: usize) { //args: &[String]
    println!();
    println!("Grid width:  x={n_x}");
    println!("Grid height: y={n_y}");
    println!("Iterations:  n={n_iterations}\n");

    let t_serial_computation = monitor(
        compute_serial, n_x, n_y, n_iterations/10,
    ).as_secs_f64() * 10.0;
    println!("  Serial: {:4.3}s", t_serial_computation);
    let t_parallel_computation = monitor(
        compute_parallel, n_x, n_y, n_iterations,
    ).as_secs_f64();
    println!("Parallel: {:4.3}s", t_parallel_computation);
    println!("Speedup => {:.2}x", t_serial_computation/t_parallel_computation);
    println!();
}


// #[pyo3(name = "sim")]

/// Python module implemented in Rust.
#[pymodule]
mod sim {
    use super::*;
    #[pyfunction]
    fn dp(x: usize, y: usize, n: usize) -> PyResult<String> {
        println!("{x} {y} {n}");
        run(x, y, n);
        Ok("Done".to_string())
    }
    #[pyfunction]
    fn pcp(x: usize, y: usize, n: usize) -> PyResult<String> {
        println!("{x} {y} {n}");
        run(x, y, n);
        Ok("Done".to_string())
    }
}


/// Minimal testing.
#[test]
fn test_dp() {
    let mut grid1 = Domain::<2, 2, 2, 3>::initialize(200, 200).randomize();
    let mut grid2 = grid1.clone();

    for _ in 0..100 {
        grid1 = grid1.next_iteration_serial();
        grid2 = grid2.next_iteration_parallel();

        assert_eq!(grid1, grid2);
    }
}
