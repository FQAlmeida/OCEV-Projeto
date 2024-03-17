from multiprocessing import cpu_count
from os import environ
from pathlib import Path

import inquirer as cli_inquirer

from ocev_projeto.framework import GAFramework
from ocev_projeto.problem_factory import problem_factory

N_THREADS = f"{cpu_count()}"
environ["OMP_NUM_THREADS"] = N_THREADS
environ["OPENBLAS_NUM_THREADS"] = N_THREADS
environ["MKL_NUM_THREADS"] = N_THREADS
environ["VECLIB_MAXIMUM_THREADS"] = N_THREADS
environ["NUMEXPR_NUM_THREADS"] = N_THREADS

if __name__ == "__main__":
    questions = [
        cli_inquirer.List(
            "problem",
            message="Qual o problema?",
            choices=["SAT-3"],
        ),
        cli_inquirer.List(
            "instance",
            message="Qual a instância problema?",
            choices=lambda awnsers: map(
                lambda file: file.name,
                Path(f"data/instances/{awnsers['problem'].lower()}").glob("*"),
            ),
        ),
    ]

    class EmptyAnswersError(Exception):
        def __init__(self) -> None:
            super().__init__("Answers is empty")

    awnsers = cli_inquirer.prompt(questions)
    if not awnsers:
        raise EmptyAnswersError()

    problem, config = problem_factory(**awnsers)
    with GAFramework(config, problem) as ga_framework:
        best, result = ga_framework.run()
    print(best)
    print(result)
