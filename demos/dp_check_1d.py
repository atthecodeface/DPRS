from dprs import sim
from dprs.utils import DP, dp_state as state

print(f"\n{sim}")

class Parameters:
    dim = sim.Dimension.D1
    n_x: int = 1_000
    n_y: int = 1
    n_z: int = 1
    p: float = 0.538910
    p0: float = 0.99
    seed: int = 1
    n_iterations: int = 1_000
    sample_rate: int  = 1_000
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Unspecified
    axis_topology_z = sim.Topology.Unspecified
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    axis_bcs_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    axis_bc_values_x = (state(DP.OCCUPIED), state(DP.OCCUPIED))
    axis_bc_values_y = (state(DP.EMPTY), state(DP.EMPTY))
    axis_bc_values_z = (state(DP.EMPTY), state(DP.EMPTY))
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16
parameters = Parameters()

_ = sim.dp(parameters)