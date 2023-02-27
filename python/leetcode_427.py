from typing import List
# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight

class Grid:
    def __init__(self, grid: List[List[int]]):
        self.grid = grid

    def is_unify(self) -> bool:
        last = None
        for row in self.grid:
            for cell in row:
                if last is None:
                    last = cell
                    continue
                if cell != last:
                    return False
        return True
    
    def get_dimension(self) -> int:
        return len(self.grid)

    def split_len(self) -> int:
        return int(self.get_dimension() / 2)

    def top_left(self) -> 'Grid':
        size = self.split_len()
        return list(map(lambda x: x[0:size], self.grid[0:size]))

    def top_right(self) -> 'Grid':
        size = self.split_len()
        return list(map(lambda x: x[size:], self.grid[0:size]))
    def bottom_left(self) -> 'Grid':
        size = self.split_len()
        return list(map(lambda x: x[0:size], self.grid[size:]))

    def bottom_right(self) -> 'Grid':
        size = self.split_len()
        return list(map(lambda x: x[size:], self.grid[size:]))
    
    def first(self) -> int:
        return self.grid[0][0]

class Solution:
    def construct(self, grid: List[List[int]]) -> Node:
        grid = Grid(grid)
        if grid.is_unify():
            return Node(grid.first(), True, None, None, None, None)
        return Node(
                None,
                False,
                self.construct(grid.top_left()),
                self.construct(grid.top_right()),
                self.construct(grid.bottom_left()),
                self.construct(grid.bottom_right()),
        )

