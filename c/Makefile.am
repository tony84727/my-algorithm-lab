check_PROGRAMS = binary_tree_test \
		 binary_tree_remove_test \
		 two_sum_brute_0_test \
		 two_sum_brute_1_test \
		 two_sum_brute_2_test \
		 two_sum_brute_3_test \
		 two_sum_hashmap_0_test \
		 two_sum_hashmap_1_test \
		 two_sum_hashmap_2_test \
		 two_sum_hashmap_3_test
TESTS = $(check_PROGRAMS)
TEST_HELPER = test_helper.h test_helper.c
binary_tree_test_SOURCES = $(TEST_HELPER) binary_tree_test.c
binary_tree_test_LDADD = libbinarytree.a
binary_tree_remove_test_SOURCES = $(TEST_HELPER) binary_tree_remove_test.c
binary_tree_remove_test_LDADD = libbinarytree.a
noinst_LIBRARIES = libbinarytree.a
libbinarytree_a_SOURCES = binary_tree.h \
			  binary_tree.c
two_sum_brute_0_test_SOURCES = $(TEST_HELPER) two_sum_brute.c two_sum_brute_0_test.c
two_sum_brute_1_test_SOURCES = $(TEST_HELPER) two_sum_brute.c two_sum_brute_1_test.c
two_sum_brute_2_test_SOURCES = $(TEST_HELPER) two_sum_brute.c two_sum_brute_2_test.c
two_sum_brute_3_test_SOURCES = $(TEST_HELPER) two_sum_brute.c two_sum_brute_3_test.c
two_sum_hashmap_0_test_SOURCES = $(TEST_HELPER) two_sum_hashmap.c two_sum_brute_0_test.c
two_sum_hashmap_1_test_SOURCES = $(TEST_HELPER) two_sum_hashmap.c two_sum_brute_1_test.c
two_sum_hashmap_2_test_SOURCES = $(TEST_HELPER) two_sum_hashmap.c two_sum_brute_2_test.c
two_sum_hashmap_3_test_SOURCES = $(TEST_HELPER) two_sum_hashmap.c two_sum_brute_3_test.c
