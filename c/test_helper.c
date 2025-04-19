#include <stdio.h>
#include <stdlib.h>

void test_assert(int result, char* message) {
	if (!result) {
		printf("failed: %s\n", message);
		exit(1);
	}
}

static int compare_int(const void *a, const void* b) {
	const int *ia = (const int*)a;
	const int *ib = (const int*)b;
	return (*ia > *ib) - (*ia < *ib);
}

void sort_ints(int* array, int array_size) {
	qsort(array, array_size, sizeof(int), compare_int);
}
