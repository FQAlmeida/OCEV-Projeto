from pathlib import Path

import numpy as np

from ocev_projeto.framework import GAFramework
from ocev_projeto.models.config import pkl_to_config


def read_instance(name: str):
    with (Path(f"data/instances/sat-3/{name}")).open("r") as fd:
        lines = fd.readlines()
        config = lines[0]
        problem = list(map(lambda line: line.strip().split(" ")[:-1], lines[1:-2]))
        problem = list(map(lambda line: list(map(int, line)), problem))
        problem = np.array(problem).astype(np.int32)
        expected_solution = int(lines[-1])
        return config, problem, expected_solution


# TODO(Otávio): Create a Problem Class
# 001
def objective(problem: np.ndarray, individual: np.ndarray):
    def evaluate(p):
        xs_abs = np.abs(p) - 1
        xs_neg = p < 0
        xs_bool = np.apply_along_axis(lambda x: individual[x], 0, xs_abs)
        xs_bool = map(
            lambda x: (x[1] and not xs_neg[x[0]]) or (not x[1] and xs_neg[x[0]]),
            enumerate(xs_bool),
        )
        return any(xs_bool) is False

    solution = np.apply_along_axis(evaluate, 1, problem)
    return np.count_nonzero(solution)


if __name__ == "__main__":
    config_line, problem, expected_solution = read_instance("uf100-01.cnf")
    config = pkl_to_config("data/config/sat3.pkl")
    ga_framework = GAFramework(config, objective, problem)
    best_individual, result = ga_framework.run()
    print(best_individual)
    print(result)
