#ifndef _H_BINARY_TREE
#define _H_BINARY_TREE

struct Tree {
	struct Tree* left;
	struct Tree* right;
	int value;
};

void tree_add(struct Tree** root, int item);

#endif
