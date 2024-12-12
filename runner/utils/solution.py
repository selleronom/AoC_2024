from dataclasses import dataclass
from collections import Counter
from typing import Optional


@dataclass
class Solution:
    model: str
    code: str
    result: Optional[str] = None


class SolutionSet:
    def __init__(self):
        self.solutions = []

    def add_solution(self, model: str, code: str, result: Optional[str] = None):
        self.solutions.append(Solution(model, code, result))

    def get_most_common_result(self) -> tuple[Optional[str], int]:
        """Returns the most common result and its count"""
        if not self.solutions:
            return None, 0

        results = [s.result for s in self.solutions if s.result is not None]
        if not results:
            return None, 0

        counter = Counter(results)
        most_common = counter.most_common(1)[0]
        return most_common

    def get_solution_with_result(self, result: str) -> Optional[Solution]:
        """Returns the first solution that produced the given result"""
        for solution in self.solutions:
            if solution.result == result:
                return solution
        return None
