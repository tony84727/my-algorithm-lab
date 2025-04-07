#include "binary_tree.h"
#include <stdlib.h>

void tree_add(struct Tree** root, int item) {
	struct Tree** current = root;
	while (*current) {
		if (item > (*current)->value)
		{
			current = &(*current)->right;
			continue;
		}
		current = &(*current)->left;
	}
	*current = calloc(1, sizeof(struct Tree));
	(*current)->value = item;
}
