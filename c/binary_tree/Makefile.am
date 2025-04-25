AM_CFLAGS = -I$(top_srcdir)/test_helper
check_PROGRAMS = binary_tree_test \
		 binary_tree_remove_test
TESTS = $(check_PROGRAMS)
binary_tree_test_SOURCES = binary_tree.h binary_tree.c binary_tree_test.c
binary_tree_remove_test_SOURCES = binary_tree.h binary_tree.c binary_tree_remove_test.c
binary_tree_test_LDADD = ../test_helper/libtesthelper.a
binary_tree_remove_test_LDADD = ../test_helper/libtesthelper.a
