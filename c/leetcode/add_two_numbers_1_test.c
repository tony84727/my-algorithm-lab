#include "add_two_numbers.h"
#include "list_util.h"
#include "test_helper.h"
#include <string.h>

int main(int argc, char** argv) {
	int a[] = {0};
	int b[] = {0};
	int expected[] = {0};
	int expected_size = 1;
	struct ListNode* la = list_from(a, sizeof(a)/sizeof(int));
	struct ListNode* lb = list_from(b, sizeof(b)/sizeof(int));
	struct ListNode* result = add_two_numbers(la, lb);
	int result_length = 0;
	int* nums = list_to_nums(result, &result_length);
	TEST_ASSERT(result_length == expected_size, "result length should be %d", expected_size);
	char array[1024];
	format_array(array, 1024, expected, 1);
	TEST_ASSERT(memcmp(nums, expected, sizeof(int)) == 0, "result should be %s", array);
}
