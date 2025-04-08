#include <stdio.h>
#include <stdlib.h>

void test_assert(int result, char* message) {
	if (!result) {
		printf("failed: %s\n", message);
		exit(1);
	}
}
