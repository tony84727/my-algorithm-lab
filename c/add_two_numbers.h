#include "list_node.h"
#include <stdlib.h>

struct ListNode* add_two_numbers(struct ListNode* l1, struct ListNode* l2) {
	int carry = 0;
	struct ListNode* head = NULL;
	struct ListNode** current;
	while (l1 != NULL || l2 != NULL) {
		int a = 0;
		int b = 0;
		if (l1 != NULL) {
			a = l1->val;
			l1 = l1->next;
		}
		if (l2 != NULL) {
			b = l2->val;
			l2 = l2->next;
		}
		int sum = a + b + carry;
		carry = sum / 10;
		struct ListNode* next = calloc(1, sizeof(struct ListNode));
		next->val = sum % 10;
		if (head == NULL) {
			head = next;
		} else {
			*current = next;
		}
		current = &next->next;
	}
	if (carry != 0)
	{
		struct ListNode* next = calloc(1, sizeof(struct ListNode));
		next->val = carry;
		*current = next;
	}
	return head;
}
