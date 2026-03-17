from dprs import sim

print(f"\n{sim}")

class Parameters:
    p: float = 0.163145
    p0: float = 0.99
    seed: int = 1
    n_iterations: int = 1_000
    sample_rate: int  = 1_000
    dim = sim.Dimension.D2
    n_x: int = 3_000
    n_y: int = 3_000
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

_ = sim.dp(parameters)