from multiprocessing import cpu_count
from os import environ


def set_numpy_threads():
    n_threads = f"{cpu_count()}"
    environ["OMP_NUM_THREADS"] = n_threads
    environ["OPENBLAS_NUM_THREADS"] = n_threads
    environ["MKL_NUM_THREADS"] = n_threads
    environ["VECLIB_MAXIMUM_THREADS"] = n_threads
    environ["NUMEXPR_NUM_THREADS"] = n_threads
