#ifndef _H_TEST_HELPER_
#define _H_TEST_HELPER_
#include <stdio.h>
void test_assert(int result, char* message);
void sort_ints(int* array, int array_size);

#define TEST_ASSERT(cmp, format, ...) \
do {\
    char message[1024];\
    snprintf(message, 1024, format, __VA_ARGS__);\
    test_assert(cmp, message);\
} while (0)

char* format_array(char* buf, int buf_size, int* const nums, int const size);
#endif
