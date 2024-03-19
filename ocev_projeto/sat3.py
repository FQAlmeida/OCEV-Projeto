import logging

import numpy as np

from ocev_projeto.framework import GAFramework
from ocev_projeto.models.config import Config, pkl_to_config
from ocev_projeto.problem import Problem

logger = logging.getLogger("PROBLEM")


class SAT3(Problem):
    def __init__(self, config: Config, instance: str) -> None:
        super().__init__("SAT-3", instance, config)
        self.config.pop_config.dim = int(self.config_line.split(" ")[2])

    def objective(self, chromossome: np.ndarray):
        xs_abs = np.abs(self.problem) - 1
        xs_neg = self.problem < 0

        def evaluate(p):
            print(p)
            xs_bool = [chromossome[x] for x in p]
            print(xs_bool)
            print(xs_neg)
            xs_bool = [
                (x[1] and not xs_neg[x[0]]) or (not x[1] and xs_neg[x[0]])
                for x in enumerate(xs_bool)
            ]
            print(xs_bool)
            return any(list(xs_bool)) is False

        solution = np.apply_along_axis(evaluate, 1, xs_abs)
        return np.count_nonzero(solution)


if __name__ == "__main__":
    config = pkl_to_config("data/config/sat-3-uf100-01.pkl")
    sat3 = SAT3(config, "uf100-01.cnf")
    with GAFramework(config, sat3) as ga_framework:
        best_individual, result = ga_framework.run()
    logger.info(best_individual)
    logger.info(result)
