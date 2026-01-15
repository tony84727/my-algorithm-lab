#include "binary_tree.h"
#include "test_helper.h"
#include <stdlib.h>

int main(int argc, char** argv) {
	struct Tree* root = NULL;
	int items[7] = {1,2,3,4,9,56,1};
	for(int i = 0; i < sizeof(items)/sizeof(int); i++)
	{
		tree_add(&root, items[i]);
	}
	test_assert(tree_search(root, 1), "tree should have 1");
	test_assert(tree_search(root,56), "tree should have 56");
	test_assert(!tree_search(root, 100), "tree shouldn't have 100");
	return 0;
}
