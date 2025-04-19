#include "two_sum_brute.h"
#include "test_helper.h"
#include <string.h>

int main(int argc, char** argv) {
	int nums[] = {3,2,4};
	int target = 6;
	int return_size = 0;
	int* matched = two_sum(nums, sizeof(nums)/sizeof(int), target, &return_size);
	test_assert(return_size == 2, "return size should be 2");
	sort_ints(matched, return_size);
	int expected[] = {1,2};
	test_assert(memcmp(expected, matched, 2) == 0, "answer should be [1,2]");
}
