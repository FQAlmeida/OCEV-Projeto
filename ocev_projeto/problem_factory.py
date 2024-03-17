from ocev_projeto.models.config import pkl_to_config
from ocev_projeto.sat3 import SAT3


class ProblemNotFoundError(Exception):
    def __init__(self, problem) -> None:
        super().__init__(f"Problema {problem} não encontrado")


def problem_factory(problem: str, instance: str):
    match problem.upper():
        case "SAT-3":
            config = pkl_to_config("data/config/sat-3.pkl")
            sat3 = SAT3(config, instance)
            return sat3, config
    raise ProblemNotFoundError(problem)
