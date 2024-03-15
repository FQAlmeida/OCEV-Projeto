from dataclasses import dataclass
from enum import Enum


class PopType(Enum):
    INT = "int"
    PERMINT = "permint"
    REAL = "real"
    BINARY = "binary"


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
