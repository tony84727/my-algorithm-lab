#ifndef _H_BINARY_TREE
#define _H_BINARY_TREE

struct Tree {
	struct Tree* left;
	struct Tree* right;
	int value;
};

void tree_add(struct Tree** root, int item);
int tree_search(struct Tree* root, int neddle);
int tree_remove(struct Tree** root, int neddle);

#endif
