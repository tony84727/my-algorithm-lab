check_PROGRAMS = binary_tree_test
TESTS = $(check_PROGRAMS)
TEST_HELPER = test_helper.h test_helper.c
binary_tree_test_SOURCES = $(TEST_HELPER) binary_tree_test.c
binary_tree_test_LDADD = libbinarytree.a
noinst_LIBRARIES = libbinarytree.a
libbinarytree_a_SOURCES = binary_tree.h \
			  binary_tree.c
