from dprs import sim

print(f"\n{sim}\n")

kwargs: dict = dict(
    n_x = 3_000,
    n_y = 5_000,
    n_z = 1,
    n_iterations = 50,
    slow_factor = 10,
    n_threads = 16,
)

_ = sim.dp(**kwargs)
