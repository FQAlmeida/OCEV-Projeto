from collections.abc import Callable
from functools import partial
from multiprocessing.pool import Pool
from time import time

import numpy as np
from matplotlib.pylab import SeedSequence

from ocev_projeto.models.config import Config
from ocev_projeto.population_initialization import PopGenerator


class GA:
    def __init__(
        self,
        config: Config,
        objective: Callable[[np.ndarray, np.ndarray], float | int],
        problem: np.ndarray,
        pool: Pool,
    ) -> None:
        seed_seq = SeedSequence(round(time()))
        rng = np.random.default_rng(seed_seq)
        self.objective = objective
        self.config = config
        self.pop_generator = PopGenerator(config.pop_config, rng)
        self.problem = problem
        self.population = self.pop_generator.generate_pop()
        self.best_individual_index = (None, None)
        self.pool = pool

    def run(self):
        for _ in range(self.config.qtd_gen):
            result = self.__fitness(self.problem, self.population)
            ranked_results = sorted(enumerate(result), key=lambda x: x[1])
            best_individual_index = ranked_results[0]
            if not self.best_individual_index[1] or best_individual_index[1] > self.best_individual_index[1]:
                self.best_individual_index = best_individual_index
        return self.best_individual, self.best_individual_value

    def __fitness(self, problem: np.ndarray, population: np.ndarray):
        result = self.pool.map(partial(self.objective, problem), population)
        return np.array(result)

    @property
    def best_individual(self):
        if self.best_individual_index == (None, None):
            return None
        return self.population[self.best_individual_index[0]]

    @property
    def best_individual_value(self):
        return self.best_individual_index[1]
