#include "binary_tree.h"
#include "test_helper.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv) {
	struct Tree* root = NULL;
	int items[7] = {1,2,3,4,9,56,1};
	for(int i = 0; i < sizeof(items)/sizeof(int); i++)
	{
		tree_add(&root, items[i]);
	}
	tree_remove(&root, 56);
     	char error_message[100];
	for(int i = 0; i < sizeof(items)/sizeof(int); i++)
	{
		if (i == 5) {
			test_assert(!tree_search(root, items[i]), "56 should've already being deleted");
			continue;
		}
		sprintf(error_message, "%d should exist in the tree", items[i]);
		test_assert(tree_search(root, items[i]), error_message);
	}
}
