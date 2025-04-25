#include "list_util.h"
#include <stdlib.h>

struct ListNode* list_from(int* nums, int size) {
	struct ListNode* head = calloc(size, sizeof(struct ListNode));
	for (int i = 0; i < size; i += 1) {
		head[i].val = nums[i];
		if (i > 0) {
			head[i-1].next = head + i;
		}
	}
	return head;
}

int* list_to_nums(struct ListNode* list, int* size) {
	*size = 0;
	int* nums = calloc(2, sizeof(int));
	while (list != NULL) {
		nums[*size] = list->val;
		*size += 1;
		if ((*size - 1) % 2 == 0) {
			nums = realloc(nums, *size * 2);
		}
		list = list->next;
	}
	return nums;
}
