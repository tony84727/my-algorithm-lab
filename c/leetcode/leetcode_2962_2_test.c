#include "leetcode_2962.h"
#include "../test_helper/test_helper.h"

int main(int argc, char** argv) {
	int nums[] = {28,5,58,91,24,91,53,9,48,85,16,70,91,91,47,91,61,4,54,61,49};
	const long long answer = count_subarrays(nums, sizeof(nums)/sizeof(int), 1);
	TEST_ASSERT(answer == 187, "expected %d got %lld", 187, answer);
}
