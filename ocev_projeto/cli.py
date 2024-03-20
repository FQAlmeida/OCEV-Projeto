import logging
from pathlib import Path
from typing import Literal

from tap import Tap

from ocev_projeto.framework import GAFramework
from ocev_projeto.problem_factory import problem_factory
from ocev_projeto.util.set_numpy_threads import set_numpy_threads

if __name__ == "__main__":
    logging.basicConfig(level="INFO")
    logger = logging.getLogger("PROBLEM")

    class Cli(Tap):
        problem: Literal["SAT-3"]
        instance: str
        config_path: Path
        numpy_parallel: bool = False

    args = Cli(underscores_to_dashes=True).parse_args()
    if args.numpy_parallel:
        set_numpy_threads()
    problem, config = problem_factory(
        args.problem, args.instance, str(args.config_path.absolute())
    )
    with GAFramework(config, problem) as ga_framework:
        best, result = ga_framework.run()
    logger.info(best)
    logger.info(result)
