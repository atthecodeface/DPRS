# [**DPRS**](https://pypi.org/project/dprs/)

###  _Directed percolation-type models in Rust_

<!-- [![](https://github.com/cstarkjp/DPRS/actions/workflows/publish-pypi.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/publish-pypi.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/publish-testpypi.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/publish-testpypi.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml) -->


In this project, we implement directed percolation (DP) and similar lattice  models in Rust. The [Rust code](https://github.com/cstarkjp/DPRS/tree/main/src) is accessed via a [Python wrapper](https://github.com/cstarkjp/DPRS/tree/main/src/sim.rs) to make experimentation as convenient as possible. Jupyter notebooks are used to implement the Python-wrapped simulations. 

![t-decay of mean ρ, for p=0.163145, nx=30,000, ny=30,000, t=50,000](images/ρmean_p0p163145_s1_nx30000_ny30000.png){width=500}

We have two motivations for adopting Rust: one is to ensure maximum performance; another is to achieve this in a memory-safe and bug-free fashion (which is not easy to do in C or C++). 

Fast run times are achieved through parallelization using the [`Rayon`](https://docs.rs/rayon/latest/rayon/) crate. 
We anticipate boosting performance further with GPU-compute using [`wgpu`](https://wgpu.rs/).

See [here](HOWTO.md) for some rough "how-to" notes on wrapping Rust with Python.

## Demos

For now, only DP has been implemented.  A series of related models are in development.

### DP

Directed-percolation model simulations in 2d are demonstrated in the following Jupyter notebook:

 - [Jupyter demo](https://github.com/cstarkjp/DPRS/tree/main/notebooks/dp_2d.ipynb)

and pure Python demos can be found here:

 - [Python demos](https://github.com/cstarkjp/DPRS/tree/main/demos/)




<!-- The  [`dprs` package ](https://pypi.org/project/dprs/) provides software tools to integrate a time-dependent density field described by DPRS equations of directed-percolation type. It can be extended to solve DPRS equations of absorbing phase transition (APT) type.

!!! note "This is a work in progress"
    `dprs` is under active development as part of a research effort.
    If you are interested in using it, or even better, interested in
    collaborating in its development, please contact the maintainer cstarkjp@gmail.com.
    
[Directed percolation (DP)](references.md) is the _type example_ of a non-equilibrium, absorbing phase transition. Its DPRS equation is:
$$
    \partial_t\rho
    =
    a \rho
    -
    b \rho^2
    +
    D \nabla^2 \rho
    +
    \eta\sqrt{\rho}\,\xi
$$
where $\rho(\mathbf{x},t)$ is a fluctuating meso-scale field  evolving nonlinearly (with coefficients $a$ and $b$) subject to diffusion (with rate $D$) and multiplicative white noise $\sqrt{\rho}\,\xi(\mathbf{x},t)$ (with amplitude $\eta$).

![Plot of grid-averaged density $\overline{\rho}(t)$ versus time, for an ensemble of simulations with $a$ taking values ranging symmetrically about criticality $a_c \approx 1.8857$ by up to $\Delta{a}=\pm 0.01$.](images/ρ_t_loglog_reduced.png)
<!-- /// caption
Plot of grid-averaged density $\overline{\rho}(t)$ versus time, for an ensemble of simulations with $a$ taking values ranging symmetrically about criticality $a_c \approx 1.8857$ by up to $\Delta{a}=\pm 0.01$.
/// -->


<!-- The `dprs` integrator employs the operator-splitting method originated largely by [Dornic et al (2005)](references.md). The software tools are implemented as a [`pip`-installable Python package](https://pypi.org/project/dprs/) with a C++ core, a set of [Jupyter notebooks](https://github.com/cstarkjp/DPRS/tree/main/simulation/dp), and related [Python scripts](https://github.com/cstarkjp/DPRS/tree/main/python). -->
 -->
