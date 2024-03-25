import errno
import os
from dataclasses import dataclass
from enum import Enum
from pathlib import Path

import pkll


class InvalidPopTypeError(Exception):
    def __init__(self, name: str):
        super().__init__(f"Invalid PopType {name}")


class InvalidSelectionMethodError(Exception):
    def __init__(self, name: str):
        super().__init__(f"Invalid Selection Method {name}")


class InvalidCrossOverMethodError(Exception):
    def __init__(self, name: str):
        super().__init__(f"Invalid CrossOver Method {name}")


class PopType(Enum):
    INT = "int"
    PERMINT = "permint"
    REAL = "real"
    BINARY = "binary"

    @staticmethod
    def str_to_type(name: str):
        match name:
            case "int":
                return PopType.INT
            case "permint":
                return PopType.PERMINT
            case "real":
                return PopType.REAL
            case "binary":
                return PopType.BINARY
        raise InvalidPopTypeError(name)


class SelectionMethod(Enum):
    ROULETTE = "roulette"
    TOURNAMENT = "tournament"

    @staticmethod
    def str_to_type(name: str):
        match name:
            case "roulette":
                return SelectionMethod.ROULETTE
            case "tournament":
                return SelectionMethod.TOURNAMENT
        raise InvalidSelectionMethodError(name)


class CrossOverMethod(Enum):
    ONE_POINT = "one-point"

    @staticmethod
    def str_to_type(name: str):
        match name:
            case "one-point":
                return CrossOverMethod.ONE_POINT
        raise InvalidCrossOverMethodError(name)


@dataclass
class BoundConfig:
    upper: float
    lower: float


class BoundsError(Exception):
    def __init__(self, pop_type: PopType):
        super().__init__(f"PopType {pop_type} requires bounds")


@dataclass
class PopConfig:
    dim: int
    pop_size: int
    pop_type: PopType
    bounds: BoundConfig | None = None


@dataclass
class Config:
    pop_config: PopConfig
    qtd_gen: int = 1
    qtd_runs: int = 1
    elitism: bool = False
    selection_method: SelectionMethod = SelectionMethod.ROULETTE
    crossover_method: CrossOverMethod = CrossOverMethod.ONE_POINT
    crossover_chance: float = 1.0
    mutation_chance: float = 0.05


def pkl_to_config(pkl_path: str) -> Config:
    config_path = Path(pkl_path)
    if config_path.is_file() is False:
        raise FileNotFoundError(errno.ENOENT, os.strerror(errno.ENOENT), config_path)
    pkl_config = pkll.load(f"file:///{config_path.absolute()}")
    config: Config = Config(
        pop_config=PopConfig(
            dim=pkl_config.config.pop_config.dim,
            pop_size=pkl_config.config.pop_config.pop_size,
            pop_type=PopType.str_to_type(pkl_config.config.pop_config.pop_type),
        ),
        selection_method=SelectionMethod.str_to_type(
            pkl_config.config.selection_method
        ),
        crossover_method=CrossOverMethod.str_to_type(
            pkl_config.config.crossover_method
        ),
        crossover_chance=pkl_config.config.crossover_chance,
        mutation_chance=pkl_config.config.mutation_chance,
        qtd_gen=pkl_config.config.qtd_gen,
        qtd_runs=pkl_config.config.qtd_runs,
        elitism=pkl_config.config.elitism,
    )
    return config
