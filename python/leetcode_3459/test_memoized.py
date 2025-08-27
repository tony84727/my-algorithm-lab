import unittest
from .memoized import Solution


class SolutionTestCase(unittest.TestCase):
    def test_example_1(self):
        self.assertEqual(
            5,
            Solution().lenOfVDiagonal(
                [
                    [2, 2, 1, 2, 2],
                    [2, 0, 2, 2, 0],
                    [2, 0, 1, 1, 0],
                    [1, 0, 2, 2, 2],
                    [2, 0, 0, 2, 2],
                ]
            ),
        )

    def test_example_2(self):
        self.assertEqual(
            4,
            Solution().lenOfVDiagonal(
                [
                    [2, 2, 2, 2, 2],
                    [2, 0, 2, 2, 0],
                    [2, 0, 1, 1, 0],
                    [1, 0, 2, 2, 2],
                    [2, 0, 0, 2, 2],
                ]
            ),
        )

    def test_example_3(self):
        self.assertEqual(
            3,
            Solution().lenOfVDiagonal(
                [
                    [0, 0, 1, 0],
                    [0, 2, 2, 0],
                ]
            ),
        )

    def test_case_1(self):
        self.assertEqual(
            2,
            Solution().lenOfVDiagonal(
                [
                    [2, 2, 0, 2, 0, 2, 0],
                    [1, 2, 2, 1, 0, 2, 0],
                ]
            ),
        )
