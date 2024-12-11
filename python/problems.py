from pathlib import Path
import dataclasses
from collections.abc import Callable, Generator
import contextlib
import time

DIR = Path(__file__).parent


@contextlib.contextmanager
def timer() -> Generator[None, None, None]:
    start = time.perf_counter()
    yield
    print(f"({(time.perf_counter() - start)*1000:.2f}ms)")


def run(name: str, solution_a: Callable[[str], int], solution_b: Callable[[str], int]):
    input = DIR.joinpath(f"../data/{name}.txt").read_text()
    with timer():
        print("Solution A:", solution_a(input), end = " ")
    with timer():
        print("Solution B:", solution_b(input), end = " ")


@dataclasses.dataclass
class test:
    name: str
    answer: int

    def __call__[T: Callable[[str], int]](self, solution: T) -> T:
        input = DIR.joinpath(f"../samples/{self.name}.txt").read_text()
        total = solution(input)
        assert total == self.answer, f"{total} not {self.answer}"
        print(f"Test {self.name}:{solution.__name__} passed")
        return solution
