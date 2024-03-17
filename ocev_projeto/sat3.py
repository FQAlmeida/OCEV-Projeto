import numpy as np

from ocev_projeto.framework import GAFramework
from ocev_projeto.models.config import Config, pkl_to_config
from ocev_projeto.problem import Problem


class SAT3(Problem):
    def __init__(self, config: Config) -> None:
        super().__init__("SAT-3", "uf100-01.cnf", config)

    def objective(self, individual: np.ndarray):
        def evaluate(p):
            xs_abs = np.abs(p) - 1
            xs_neg = p < 0
            xs_bool = np.apply_along_axis(lambda x: individual[x], 0, xs_abs)
            xs_bool = map(
                lambda x: (x[1] and not xs_neg[x[0]]) or (not x[1] and xs_neg[x[0]]),
                enumerate(xs_bool),
            )
            return any(xs_bool) is False

        solution = np.apply_along_axis(evaluate, 1, self.problem)
        return np.count_nonzero(solution)


if __name__ == "__main__":
    config = pkl_to_config("data/config/sat-3.pkl")
    sat3 = SAT3(config)
    ga_framework = GAFramework(config, sat3)
    best_individual, result = ga_framework.run()
    print(best_individual)
    print(result)
