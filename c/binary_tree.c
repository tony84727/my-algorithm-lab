#include "binary_tree.h"
#include <stdlib.h>

void tree_add(struct Tree** root, int item) {
	while (*root) {
		if (item > (*root)->value)
		{
			root = &(*root)->right;
			continue;
		}
		root = &(*root)->left;
	}
	*root = calloc(1, sizeof(struct Tree));
	(*root)->value = item;
}

int tree_search(struct Tree* root, int neddle) {
	while (root) {
		if (root->value == neddle) {
			return 1;
		}
		if (root->value > neddle) {
			root = root->left;
			continue;
		}
		root = root->right;
	}
	return 0;
}
