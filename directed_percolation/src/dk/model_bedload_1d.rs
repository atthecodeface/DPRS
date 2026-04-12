use crate::{Cell1D, CellModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// The DP bedload model has the following mechanisms:
///   1) collective entrainment: a moving grain hits a static grain => two moving grains
///   2) collective detrainment: two moving grains collide => one moving + one static
///   3) detrainment: a moving grain stops moving => one static grain
///   4) entrainment: a static grain starts moving => one moving grain
/// In the context of a reaction-diffusion model in Langevin form, with order param ρ:
///   1) collective entrainment rate = + a_ce ρ
///   2) collective detrainment rate = - b_cd ρ^2
///   3) detrainment rate = - a_d ρ
///   4) entrainment rate = + c_e
/// such that the total rate δρ/δt ~ (a_ce-a_d) ρ - b_cd ρ^2  + c_e + diffusion + noise
/// where +c_e is not a standard DP term but rather an "external conjugate field" term.
///
/// ModelBedload1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedload1D {
    p_1: f64,
    p_2: f64,
}

// Implement CellModel1D trait for ModelBedload1D.
impl CellModel<Cell1D> for ModelBedload1D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        let do_survive = {
            // TODO: change to bedload rule
            //
            // Simplified Domany-Kinzel rule: this cell will become occupied if:
            // either (1) it's already occupied and a coin toss with prob p_1 succeeds
            //   or   (2) (regardless) it has neighbors and a coin toss with prob p_2 succeeds
            let n_nbrs: usize = [nbrhood[0].into(), nbrhood[2].into()].iter().sum();
            let has_nbrs = n_nbrs > 0;
            let uniform_variate: f64 = rng.random();
            let is_occupied = nbrhood[1];
            let is_activated = (is_occupied & (uniform_variate < self.p_1))
                | (has_nbrs & (uniform_variate < self.p_2));
            is_activated
        };
        do_survive.into()
    }
}
