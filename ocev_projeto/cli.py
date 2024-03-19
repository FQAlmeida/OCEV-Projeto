import logging
from multiprocessing import cpu_count
from os import environ
from pathlib import Path
from typing import Literal

from tap import Tap

from ocev_projeto.framework import GAFramework
from ocev_projeto.problem_factory import problem_factory

N_THREADS = f"{cpu_count()}"
environ["OMP_NUM_THREADS"] = N_THREADS
environ["OPENBLAS_NUM_THREADS"] = N_THREADS
environ["MKL_NUM_THREADS"] = N_THREADS
environ["VECLIB_MAXIMUM_THREADS"] = N_THREADS
environ["NUMEXPR_NUM_THREADS"] = N_THREADS

if __name__ == "__main__":
    logging.basicConfig(level="INFO")
    logger = logging.getLogger("PROBLEM")

    class Cli(Tap):
        problem: Literal["SAT-3"]
        instance: str
        config_path: Path

    args = Cli(underscores_to_dashes=True).parse_args()

    problem, config = problem_factory(**args.as_dict())
    with GAFramework(config, problem) as ga_framework:
        best, result = ga_framework.run()
    logger.info(best)
    logger.info(result)
