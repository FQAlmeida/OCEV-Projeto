from collections.abc import Callable
from multiprocessing import Pool, cpu_count

import numpy as np

from ocev_projeto.ga import GA
from ocev_projeto.models.config import Config


# TODO(Otávio): Create tests
# 002
class GAFramework:
    def __init__(
        self,
        config: Config,
        objective: Callable[[np.ndarray, np.ndarray], float | int],
        problem: np.ndarray,
    ) -> None:
        self.config = config
        self.objective = objective
        self.problem = problem

    def run(self):
        pool = Pool(cpu_count())
        best_individual, result = (None, None)
        for run in range(self.config.qtd_runs):
            print(f"Run {run + 1}")
            ga = GA(self.config, self.objective, self.problem, pool)
            new_indiv, new_result = ga.run()
            if not result or new_result < result:
                best_individual, result = (new_indiv, new_result)
        pool.close()
        return best_individual, result
