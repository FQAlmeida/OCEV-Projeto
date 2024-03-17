from collections.abc import Callable
from multiprocessing import Pool, cpu_count

import numpy as np

from ocev_projeto.ga import GA
from ocev_projeto.models.config import Config


class PoolNotInitiatedError(Exception):
    def __init__(self) -> None:
        super().__init__(
            "Pool is not initiated, "
            "please use with structure, "
            "so it call __enter__"
        )


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
        self.pool = None

    def __enter__(self):
        self.pool = Pool(cpu_count())
        return self

    def run(self):
        if self.pool is None:
            raise PoolNotInitiatedError()
        best_individual, result = (None, None)
        for run in range(self.config.qtd_runs):
            print(f"Run {run + 1}")
            ga = GA(self.config, self.objective, self.problem, self.pool)
            new_indiv, new_result = ga.run()
            if not result or new_result < result:
                best_individual, result = (new_indiv, new_result)
        return best_individual, result

    def __exit__(self, exc_type, exc_value, traceback):
        if not self.pool:
            return
        self.pool.close()
