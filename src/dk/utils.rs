// #![warn(missing_docs)]
// //!
// //!

use crate::dk::traits::HasMean;

pub fn update_statistics<T: HasMean>(i: usize, lattice_model: &T, tracking: &mut Vec<Vec<f64>>) {
    let t = i as f64;
    tracking[0].push(t);
    let rho_mean = lattice_model.mean();
    tracking[1].push(rho_mean);
}