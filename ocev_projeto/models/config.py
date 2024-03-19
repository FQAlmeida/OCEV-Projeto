import errno
import os
from dataclasses import dataclass
from enum import Enum
from pathlib import Path

import pkll


class InvalidPopTypeError(Exception):
    def __init__(self, name: str):
        super().__init__(f"Invalid PopType {name}")


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
        qtd_gen=pkl_config.config.qtd_gen,
        qtd_runs=pkl_config.config.qtd_runs,
        elitism=pkl_config.config.elitism,
    )
    return config
