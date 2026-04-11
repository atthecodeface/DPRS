use directed_percolation::SimParameters;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
pub struct Dims {
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
}

impl From<&SimParameters> for Dims {
    fn from(p: &SimParameters) -> Dims {
        let mut s = Dims::default();
        s.n_x = p.n_x;
        s.n_y = p.n_y;
        s.n_z = p.n_z;
        s
    }
}
impl From<&Dims> for SimParameters {
    fn from(p: &Dims) -> SimParameters {
        let mut s = SimParameters::default();
        s.n_x = p.n_x;
        s.n_y = p.n_y;
        s.n_z = p.n_z;
        s
    }
}

crate::make_default_constructor! {Dims}
