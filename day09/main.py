import numpy.polynomial as poly
import sys


def extrapolate(history: list[int]) -> tuple[int, int]:
    n = len(history)
    p = poly.Polynomial.fit(range(n), history, n-1)
    # the results are expected to be integers, but due to floating point errors, we need to round them
    return round(p(-1)), round(p(n))


assert extrapolate([0, 3, 6, 9, 12, 15]) == (-3, 18)
assert extrapolate([1, 3, 6, 10, 15, 21]) == (0, 28)
assert extrapolate([10, 13, 16, 21, 30, 45]) == (5, 68)


def process_file(path: str) -> tuple[int, int]:
    total_before, total_after = (0, 0)
    with open(path) as f:
        for line in f:
            history = [int(n) for n in line.split(' ')]
            before, after = extrapolate(history)
            total_before += before
            total_after += after
    return total_before, total_after


if __name__ == '__main__':
    assert len(sys.argv) == 2, "Missing input file path"
    print(process_file(sys.argv[1]))
