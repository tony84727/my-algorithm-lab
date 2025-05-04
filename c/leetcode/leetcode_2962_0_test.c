#include "leetcode_2962.h"
#include "../test_helper/test_helper.h"

int main(int argc, char** argv) {
	int nums[] = {1,3,2,3,3};
	const long long answer = count_subarrays(nums, sizeof(nums)/sizeof(int), 2);
	TEST_ASSERT(answer == 6, "expected %d got %lld", 6, answer);
}
