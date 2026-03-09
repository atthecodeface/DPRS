from dprs import sim

print(f"\n{sim}\n")

kwargs = dict(
    n_x = 6_000,
    n_y = 6_000,
    # n_z = 1,
    p = 0.5,
    n_iterations = 100,
    sample_rate = 10,
    serial_skip = 10,
    n_threads = 16,
)

_ = sim.dp(**kwargs)
