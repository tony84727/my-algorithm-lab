#ifndef _H_LIST_UTIL_
#define _H_LIST_UTIL_
#include "list_node.h"
#include <stdlib.h>

struct ListNode* list_from(int* nums, int size);
int* list_to_nums(struct ListNode* list, int* size);
#endif
