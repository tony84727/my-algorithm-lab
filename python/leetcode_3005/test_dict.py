import unittest

from .dict import Solution

class SolutionTestCase(unittest.TestCase):
    def test_example_1(self):
        self.assertEqual(
                4,
                Solution().maxFrequencyElements([1,2,2,3,1,4]))

    def test_example_2(self):
        self.assertEqual(
                5,
                Solution().maxFrequencyElements([1,2,3,4,5,]))

