import logging

import numpy as np
from numba import jit

from ocev_projeto.framework import GAFramework
from ocev_projeto.models.config import Config, pkl_to_config
from ocev_projeto.problem import Problem

logger = logging.getLogger("PROBLEM")


class SAT3(Problem):
    def __init__(self, config: Config, instance: str) -> None:
        super().__init__("SAT-3", instance, config)
        self.config.pop_config.dim = int(self.config_line.split(" ")[2])

    def objective(self, chromossome: np.ndarray):
        @jit
        def obj(chromossome: np.ndarray, problem: np.ndarray):
            clausula_id = np.abs(problem) - 1
            clausula_neg = problem < 0

            def evaluate(p):
                return np.array([chromossome[x] for x in p])

            solution = np.apply_along_axis(evaluate, 1, clausula_id)
            solution = np.logical_or(
                np.logical_and(clausula_neg, np.logical_not(solution)),
                np.logical_and(np.logical_not(clausula_neg), solution),
            )
            solution = np.logical_not(np.apply_along_axis(np.any, 1, solution))
            return np.count_nonzero(solution)
        return obj(chromossome, self.problem)


if __name__ == "__main__":
    config = pkl_to_config("data/config/sat-3-uf100-01.pkl")
    sat3 = SAT3(config, "uf100-01.cnf")
    with GAFramework(config, sat3) as ga_framework:
        best_individual, result = ga_framework.run()
    logger.info(best_individual)
    logger.info(result)
