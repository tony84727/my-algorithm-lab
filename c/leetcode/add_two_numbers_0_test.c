#include "add_two_numbers.h"
#include "list_util.h"
#include "test_helper.h"
#include <string.h>

int main(int argc, char** argv) {
	int a[] = {2,4,3};
	int b[] = {5,6,4};
	int expected[] = {7,0,8};
	const int EXPECTED_SIZE = 3;
	struct ListNode* la = list_from(a, sizeof(a)/sizeof(int));
	struct ListNode* lb = list_from(b, sizeof(b)/sizeof(int));
	struct ListNode* result = add_two_numbers(la, lb);
	int result_length = 0;
	int* nums = list_to_nums(result, &result_length);
	TEST_ASSERT(result_length == EXPECTED_SIZE, "result length should be %d", EXPECTED_SIZE);
	char array[1024];
	format_array(array, 1024, expected, sizeof(expected)/sizeof(int));
	TEST_ASSERT(memcmp(nums, expected, 3*sizeof(int)) == 0, "result should be %s", array);
}
