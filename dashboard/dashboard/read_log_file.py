import json
from dataclasses import dataclass
from enum import StrEnum
from pathlib import Path


class LogTypes(StrEnum):
    STATE_INDIVIDUAL = "INFO - State Individual: "
    PROBLEM = "INFO - Problem: "
    RUN = "INFO - Run: "
    END_RUN = "INFO - End Run: "
    BEST_INDIVIDUAL = "INFO - Best Individual: "
    BEST_INDIVIDUAL_DECODED = "INFO - Best Individual Decoded: "
    BEST_INDIVIDUAL_CONSTRAINT = "INFO - Best Individual Constraint: "
    BEST_INDIVIDUAL_VALUE_DECODED = "INFO - Best Individual Value Decoded: "
    BEST_INDIVIDUAL_VALUE = "INFO - Best Individual Value: "
    CONFIG = "INFO - Config: "


@dataclass
class GenerationData:
    generation: int
    best_all: float
    best_pop: float
    mean: float
    worst: float


@dataclass
class Data:
    run: int
    generations: list[GenerationData]
    best: float
    best_normed: float
    best_individual: list[int | float]
    decoded: list[int | float]
    constraint: float


@dataclass
class ProblemData:
    name: str
    config: dict
    runs: list[Data]


def read_log_file(file_path: Path):
    lines = _read_lines(file_path)
    problem_data = ProblemData(name="", runs=[], config=dict())
    data = _create_data()
    for line in lines:
        data, problem_data = process_line(line, data, problem_data)
    return problem_data


def process_line(line: str, data: Data, problem_data: ProblemData):
    if line.startswith(LogTypes.STATE_INDIVIDUAL):
        _process_state_individual(line, data)
    elif line.startswith(LogTypes.PROBLEM):
        problem_data = _process_problem(line)
    elif line.startswith(LogTypes.CONFIG):
        _process_config(line, problem_data)
    elif line.startswith(LogTypes.RUN):
        _process_run(line, data)
    elif line.startswith(LogTypes.END_RUN):
        _process_end_run(problem_data, data)
        data = _create_data()
    elif line.startswith(LogTypes.BEST_INDIVIDUAL):
        _process_best_individual(line, data)
    elif line.startswith(LogTypes.BEST_INDIVIDUAL_DECODED):
        _process_best_individual_decoded(line, data)
    elif line.startswith(LogTypes.BEST_INDIVIDUAL_CONSTRAINT):
        _process_best_individual_constraint(line, data)
    elif line.startswith(LogTypes.BEST_INDIVIDUAL_VALUE_DECODED):
        _process_best_individual_value_decoded(line, data)
    elif line.startswith(LogTypes.BEST_INDIVIDUAL_VALUE):
        _process_best_individual_value(line, data)
    return data, problem_data


def _read_lines(file_path: Path):
    with file_path.open("r") as fd:
        lines = fd.readlines()
    return lines


def _create_data():
    return Data(
        run=1,
        generations=[],
        best=0,
        best_individual=[],
        decoded=[],
        constraint=0,
        best_normed=0,
    )


def _process_state_individual(line: str, data: Data):
    content = line.removeprefix(LogTypes.STATE_INDIVIDUAL).strip()
    generation, best_all, best_pop, average, worst = map(float, content.split(" "))
    data.generations.append(
        GenerationData(int(generation), best_all, best_pop, average, worst)
    )


def _process_problem(line: str) -> ProblemData:
    content = line.removeprefix(LogTypes.PROBLEM).strip()
    (problem_name,) = content.split(" ")
    return ProblemData(name=problem_name, runs=[], config=dict())


def _process_config(line: str, problem: ProblemData):
    content = line.removeprefix(LogTypes.CONFIG).strip()
    config = json.loads(content)
    problem.config = config


def _process_run(line: str, data: Data):
    content = line.removeprefix(LogTypes.RUN).strip()
    (run,) = map(int, content.split(" "))
    data.run = run


def _process_end_run(problem_data: ProblemData, data: Data):
    problem_data.runs.append(data)


def _process_best_individual(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL).strip()
    data.best_individual = eval(
        content.replace("true", "True").replace("false", "False")
    )


def _process_best_individual_decoded(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_DECODED).strip()
    data.decoded = eval(content)


def _process_best_individual_constraint(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_CONSTRAINT).strip()
    data.constraint = float(content)


def _process_best_individual_value_decoded(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_VALUE_DECODED).strip()
    (best,) = map(float, content.split(" "))
    data.best = best


def _process_best_individual_value(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_VALUE).strip()
    (best_normed,) = map(float, content.split(" "))
    data.best_normed = best_normed
