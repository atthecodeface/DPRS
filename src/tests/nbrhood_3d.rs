use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::{
    dp::{CellModel3D, LatticeModel3D, Nbrhood3D, RowIterator3D},
    parameters::{BoundaryCondition, Dimension, Parameters, Processing, Topology},
};

#[derive(Debug)]
struct Model3D();

impl CellModel3D for Model3D {
    type State = isize;
    fn from_bool_to_state(b: &bool) -> Self::State {
        *b as isize
    }
    fn from_state_to_bool(state: &Self::State) -> bool {
        *state != 0
    }
    fn randomize_state<R: rand::Rng>(&self, _rng: &mut R, _p: f64) -> Self::State {
        0
    }
    fn update_state<R: rand::Rng>(
        &self,
        _rng: &mut R,
        _p: f64,
        nbrhood: &Nbrhood3D<Self>,
    ) -> Self::State {
        *nbrhood.iter().max().unwrap()
    }
}

fn value(parameters: &Parameters, i: usize, opt_dxyz: Option<(i8, i8, i8)>) -> isize {
    let mut x = (i % parameters.n_x) as isize;
    let mut y = (i / parameters.n_x % parameters.n_y) as isize;
    let mut z = (i / parameters.n_x / parameters.n_y) as isize;
    if let Some(dxyz) = opt_dxyz {
        x += dxyz.0 as isize;
        y += dxyz.1 as isize;
        z += dxyz.2 as isize;
    };
    10 + x + y * 7 + z * 13
}

#[test]
fn test_dp() {
    let n_x = 10;
    let n_y = 15;
    let n_z = 20;

    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;

    let mut lm = LatticeModel3D::new(Model3D(), n_x, n_y, n_z, (0, 0), (1, 1), (2, 2));
    for (i, l) in lm.lattice_mut().iter_mut().enumerate() {
        *l = value(&parameters, i, None);
    }

    for (mut x, y, z) in [(1, 1, 1)] {
        //}, (1, 4, 3), (1, n_y - 2, n_z - 2), (n_x - 2, 1, 1)] {
        let mut r = RowIterator3D::<Model3D>::new(lm.lattice(), (x, y, z), n_x, n_y)
            .expect("Must be able to make a Row iterator at this x/y/z");

        let mut i = (z * n_y + y) * n_x + x;
        loop {
            let nbrhood_of_xyz = r.nbrhood();

            for dx in (-1)..2 {
                for dy in (-1)..2 {
                    for dz in (-1)..2 {
                        assert_eq!(
                            nbrhood_of_xyz[((dx + 1) as u8, (dy + 1) as u8, (dz + 1) as u8)],
                            value(&parameters, i, Some((dx, dy, dz))),
                            "Contents for {x},{y},{z} : {i} : {dx},{dy},{dz} should match"
                        );
                    }
                }
            }
            i += 1;
            x += 1;
            if !r.next() {
                break;
            }
        }
        assert_eq!(x, n_x - 1, "Should have reached the end of the row");
    }
}

#[test]
fn test_sim() {
    let mut parameters = Parameters::default();
    parameters.dim = Dimension::D3;
    parameters.n_x = 4;
    parameters.n_y = 5;
    parameters.n_z = 6;
    parameters.n_iterations = 1;
    parameters.sample_rate = 1;
    parameters.seed = 1;
    parameters.axis_topology_x = Topology::Open;
    parameters.axis_topology_y = Topology::Open;
    parameters.axis_topology_z = Topology::Open;
    parameters.axis_bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.axis_bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.axis_bcs_z = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    let mut lm = LatticeModel3D::new(
        Model3D(),
        parameters.n_x,
        parameters.n_y,
        parameters.n_z,
        (0, 0),
        (1, 1),
        (2, 2),
    );
    for (i, l) in lm.lattice_mut().iter_mut().enumerate() {
        *l = value(&parameters, i, None);
    }
    let (n_lattices, lattices, tracking) = crate::dp::simulation_3d(
        lm,
        &mut StdRng::seed_from_u64(1),
        &Processing::Serial,
        &parameters,
        parameters.n_iterations,
        parameters.sample_rate,
    );
    assert_eq!(
        &lattices[0],
        &(0..parameters.n_x * parameters.n_y * parameters.n_z)
            .map(|i| value(&parameters, i, None))
            .collect::<Vec<_>>()
    );
    for x in 1..parameters.n_x - 1 {
        for y in 1..parameters.n_y - 1 {
            for z in 1..parameters.n_z - 1 {
                let c = lattices[0][(z * parameters.n_y + y) * parameters.n_x + x];
                eprintln!("{x}, {y}, {z}, {c}");
            }
        }
    }
    dbg!(&lattices[1]);
    assert!(false, "Force failure");
}
