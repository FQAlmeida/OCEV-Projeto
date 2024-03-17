from pathlib import Path

import numpy as np

from ocev_projeto.models.config import Config


class Problem:
    def __init__(self, name: str, instance: str, config: Config) -> None:
        self.instance = instance
        self.name = name
        _, problem, expected_solution = self.read_instance()
        self.problem = problem
        self.config = config
        self.expected_solution = expected_solution

    def read_instance(self):
        with (
            Path(f"data/instances/{self.name.lower()}/{self.instance.lower()}")
        ).open("r") as fd:
            lines = fd.readlines()
            config = lines[0]
            problem = list(
                map(lambda line: line.strip().split(" ")[:-1], lines[1:-2])
            )
            problem = list(map(lambda line: list(map(int, line)), problem))
            problem = np.array(problem).astype(np.int32)
            expected_solution = int(lines[-1])
            return config, problem, expected_solution

    def objective(self, individual: np.ndarray) -> int | float: ...
