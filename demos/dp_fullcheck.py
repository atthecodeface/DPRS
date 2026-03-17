import numpy as np
from scipy.stats import linregress
from numpy.typing import NDArray
from dprs import sim

print(f"\n{sim}")

class Parameters:
    p: float = 0.163140
    p0: float = 0.99
    seed: int = 1
    n_iterations: int = 5_000
    sample_rate: int  = 5_000
    dim = sim.Dimension.D2
    n_x: int = 5_000
    n_y: int = 5_000
    n_z: int = 1
    edge_topology_x = sim.Topology.Periodic
    edge_topology_y = sim.Topology.Periodic
    edge_topology_z = sim.Topology.Unspecified
    edge_bc_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    edge_bc_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    edge_bc_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    edge_values_x = (True, True)
    edge_values_y = (True, True)
    edge_values_z = (False, False)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16
parameters = Parameters()
# Just in case we forget to update sample_rate to match n_iterations
if parameters.sample_rate > parameters.n_iterations:
    parameters.sample_rate = parameters.n_iterations

n_lattices: int
raw_lattices: list[list[bool]] 
raw_tracking: list[list, list]
t_run_time: float
(n_lattices, raw_lattices, raw_tracking, t_run_time)= sim.dp(parameters)
lattices: NDArray = np.array(raw_lattices, dtype=np.bool,).reshape(
    n_lattices, parameters.n_y, parameters.n_x,
).T
tracking: NDArray = np.array(raw_tracking, dtype=np.float64,) 

i_offset: int = parameters.n_iterations//2
t: NDArray = tracking[0][i_offset:]
ρ_mean: NDArray = tracking[1][i_offset:]
(slope, intercept, r_value, p_value, std_err) \
    = linregress(np.log(t), np.log(ρ_mean))

print(rf"Estimated t-decay exponent:  δ = {slope:0.4f}")