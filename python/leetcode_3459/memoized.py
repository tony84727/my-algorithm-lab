from typing import List
from functools import cache

class Solution:
    def lenOfVDiagonal(self, grid: List[List[int]]) -> int:
        @cache
        def find_longest_v(
            direction: int,
            r: int,
            c: int,
            expected: int,
            turned: bool = False,
        ) -> int:
            m = len(grid)
            n = len(grid[0])
            directions = [(-1, 1), (1, 1), (1, -1), (-1, -1)]
            offset = directions[direction]
            turnedDirection = (direction + 1) % 4
            nr = r + offset[0]
            nc = c + offset[1]
            nextExpected = 0 if expected == 2 else 2
            if not (0 <= nr < m and 0 <= nc < n) or grid[nr][nc] != expected:
                return 0
            if turned:
                return 1 + find_longest_v(direction, nr, nc, nextExpected, True)
            else:
                return 1 + max(
                    find_longest_v(direction, nr, nc, nextExpected, False),
                    find_longest_v(turnedDirection, nr, nc, nextExpected, True),
                )
        result = 0
        for r, row in enumerate(grid):
            for c, cell in enumerate(row):
                if cell == 1:
                    result = max(
                        result,
                        *[
                            1 + find_longest_v(direction, r, c, 2, False)
                            for direction in range(4)
                        ],
                    )
        return result
