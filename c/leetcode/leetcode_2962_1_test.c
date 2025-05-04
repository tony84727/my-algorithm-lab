#include "leetcode_2962.h"
#include "../test_helper/test_helper.h"

int main(int argc, char** argv) {
	int nums[] = {61,23,38,23,56,40,82,56,82,82,82,70,8,69,8,7,19,14,58,42,82,10,82,78,15,82};
	const long long answer = count_subarrays(nums, sizeof(nums)/sizeof(int), 2);
	TEST_ASSERT(answer == 224, "expected %d got %lld", 224, answer);
}
