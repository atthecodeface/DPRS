#!/usr/bin/env python3
import os
import pathlib
import sys

this_dir = pathlib.Path(__file__).parent.resolve()
sys.path.append(this_dir.joinpath("target", "release").__str__())

import dprs as sim


class Parameters:
    # dim: Dimension::D1,
    # processing: Processing::ParallelChunked,
    dim: int = 0
    processing: int = 2
    n_x: int = 1
    n_y: int = 1
    n_z: int = 1
    p: float = 0.5
    n_iterations: int = 1
    sample_rate: int = 10
    n_threads: int = 1
    serial_skip: int = 1
    do_buffering: bool = True


print(f"\n{sim}\n")

_ = sim.life(Parameters())

# num_iter = 10
# thread_counts = range(1, 11)
# sizes = [1000, 2000, 3000, 4000]
# for size in sizes:
#    ns = life.serial(size, size, num_iter)[1]
#    print(f"{size}: {ns / (size * size)}ns per cell for {num_iter} iterations serially")
#    for num_threads in thread_counts:
#        ns = life.parallel(num_threads, size, size, num_iter)[1]
#        print(
#            f"{size}: {ns / (size * size)}ns per cell for {num_iter} iterations with {num_threads} threads"
#        )
#    for num_threads in thread_counts:
#        ns = life.parallel_chunked(num_threads, size, size, num_iter)[1]
#        print(
#            f"{size}: {ns / (size * size)}ns per cell for {num_iter} iterations with {num_threads} threads using chunking"
#        )
#        pass
#    pass
