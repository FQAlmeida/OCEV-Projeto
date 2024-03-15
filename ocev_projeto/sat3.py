from multiprocessing import Pool, cpu_count
from pathlib import Path

import numpy as np

from ocev_projeto.ga import GA
from ocev_projeto.models.config import Config, PopConfig, PopType


def read_instance(name: str):
    with (Path(f"data/instances/{name}")).open("r") as fd:
        lines = fd.readlines()
        config = lines[0]
        problem = list(map(lambda line: line.strip().split(" ")[:-1], lines[1:-2]))
        problem = list(map(lambda line: list(map(int, line)), problem))
        problem = np.array(problem).astype(np.int32)
        expected_solution = int(lines[-1])
        return config, problem, expected_solution


def objective(problem: np.ndarray, individual: np.ndarray):
    def evaliate(p):
        xs = list(map(int, p))
        xs_abs = map(lambda x: abs(x) - 1, xs)
        xs_neg = list(map(lambda x: x < 0, xs))
        xs_bool = map(lambda x: bool(individual[x]), xs_abs)
        xs_bool = map(lambda x: (x[1] and not xs_neg[x[0]]) or (not x[1] and xs_neg[x[0]]), enumerate(xs_bool))
        return any(xs_bool) is False
    solution = np.apply_along_axis(evaliate, 1, problem)
    return np.count_nonzero(solution)


if __name__ == "__main__":
    config_line, problem, expected_solution = read_instance("uf100-01.cnf")
    config: Config = Config(
        pop_config=PopConfig(dim=int(config_line.split(" ")[2]), pop_size=300, pop_type=PopType.BINARY),
        qtd_gen=1000,
        qtd_runs=10,
        elitism=False,
    )
    pool = Pool(cpu_count())
    indiv, result = (None, None)
    for _ in range(config.qtd_runs):
        ga = GA(config, objective, problem, pool)
        new_indiv, new_result = ga.run()
        if not result or new_result < result:
            indiv, result = (new_indiv, new_result)
    pool.close()
    print(indiv)
    print(result)
