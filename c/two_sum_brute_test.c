#include "two_sum_brute.h"
#include "test_helper.h"
#include <string.h>

int main(int argc, char** argv) {
	int nums[4] = {2,7,11,15};
	int target = 9;
	int return_size = 0;
	int* matched = two_sum(nums, sizeof(nums)/sizeof(int), target, &return_size);
	test_assert(return_size == 2, "return size should be 2");
	int expected[2] = {0,1};
	test_assert(memcmp(expected, matched, 2) == 0, "answer should be [0,1]");
}
