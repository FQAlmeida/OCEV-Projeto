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
    cmds = {
        LogTypes.STATE_INDIVIDUAL: _prcs_state_individual,
        LogTypes.RUN: _prcs_run,
        LogTypes.BEST_INDIVIDUAL: _prcs_best_individual,
        LogTypes.BEST_INDIVIDUAL_DECODED: _prcs_best_individual_decoded,
        LogTypes.BEST_INDIVIDUAL_CONSTRAINT: _prcs_best_individual_constraint,
        LogTypes.BEST_INDIVIDUAL_VALUE_DECODED: _prcs_best_individual_value_decoded,
        LogTypes.BEST_INDIVIDUAL_VALUE: _prcs_best_individual_value,
    }
    if line.startswith(LogTypes.PROBLEM):
        problem_data = _prcs_problem(line)
    elif line.startswith(LogTypes.END_RUN):
        data = _prcs_end_run(problem_data, data)
    elif line.startswith(LogTypes.CONFIG):
        _prcs_config(line, problem_data)
    else:
        for prefix, cmd in cmds.items():
            if line.startswith(prefix):
                cmd(line, data)
                break
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


def _prcs_state_individual(line: str, data: Data):
    content = line.removeprefix(LogTypes.STATE_INDIVIDUAL).strip()
    generation, best_all, best_pop, average, worst = map(float, content.split(" "))
    data.generations.append(
        GenerationData(int(generation), best_all, best_pop, average, worst)
    )


def _prcs_problem(line: str) -> ProblemData:
    content = line.removeprefix(LogTypes.PROBLEM).strip()
    (problem_name,) = content.split(" ")
    return ProblemData(name=problem_name, runs=[], config=dict())


def _prcs_config(line: str, problem: ProblemData):
    content = line.removeprefix(LogTypes.CONFIG).strip()
    config = json.loads(content)
    problem.config = config


def _prcs_run(line: str, data: Data):
    content = line.removeprefix(LogTypes.RUN).strip()
    (run,) = map(int, content.split(" "))
    data.run = run


def _prcs_end_run(problem_data: ProblemData, data: Data):
    problem_data.runs.append(data)
    return _create_data()


def _prcs_best_individual(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL).strip()
    data.best_individual = eval(
        content.replace("true", "True").replace("false", "False")
    )


def _prcs_best_individual_decoded(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_DECODED).strip()
    data.decoded = eval(content)


def _prcs_best_individual_constraint(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_CONSTRAINT).strip()
    data.constraint = float(content)


def _prcs_best_individual_value_decoded(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_VALUE_DECODED).strip()
    (best,) = map(float, content.split(" "))
    data.best = best


def _prcs_best_individual_value(line: str, data: Data):
    content = line.removeprefix(LogTypes.BEST_INDIVIDUAL_VALUE).strip()
    (best_normed,) = map(float, content.split(" "))
    data.best_normed = best_normed
