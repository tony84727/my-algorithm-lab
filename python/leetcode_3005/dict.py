from typing import List

class Solution:
    def maxFrequencyElements(self, nums: List[int]) -> int:
        count = {}
        current = 0
        for n in nums:
            count[n] = count.get(n,0) + 1
            if count[n] > current:
                current = count[n]
        return sum(filter(lambda v: v == current, count.values()))
