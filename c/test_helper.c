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

char* format_array(char* buf, int buf_size, int* nums, int size) {
	int length = 1;
	buf[0] = '[';
	for(int i = 0;i < size; i++) {
		length += snprintf(buf + length, buf_size - length, "%d", nums[i]);
		if (i + 1 != size) {
			buf[length] = ',';
			continue;
		}
		if (buf_size - length > 1) {
			buf[length] = ']';
			buf[length+1] = '\0';
		}
	}
	return buf;
}
