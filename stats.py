import sys
from collections import Counter
from typing import List, Tuple


class StatisticsCalculator:
    def __init__(self, data: List[int]):
        if not data:
            raise ValueError("Data list cannot be empty.")
        self.data = data

    def mean(self) -> float:
        return sum(self.data) / len(self.data)

    def median(self) -> float:
        sorted_data = sorted(self.data)
        n = len(sorted_data)
        mid = n // 2
        if n % 2 == 1:
            return float(sorted_data[mid])
        return (sorted_data[mid - 1] + sorted_data[mid]) / 2.0

    def mode(self) -> Tuple[List[int], int]:
        counts = Counter(self.data)
        max_freq = max(counts.values())

        if max_freq == 1:
            return ([], 0)

        modes = sorted([k for k, v in counts.items() if v == max_freq])
        return (modes, max_freq)


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <int> <int> ...")
        return

    data = [int(x) for x in sys.argv[1:]]
    calc = StatisticsCalculator(data)

    modes, freq = calc.mode()

    print(f"Count: {len(data)}")
    print(f"Mean: {calc.mean():.3f}")
    print(f"Median: {calc.median():.3f}")

    if not modes:
        print("Mode: none")
    else:
        print(f"Mode(s): {', '.join(map(str, modes))} (frequency={freq})")


if __name__ == "__main__":
    main()
