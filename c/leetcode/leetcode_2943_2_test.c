#include "leetcode_2943.h"
#include "test_helper.h"

int main(int argc, char** argv) {
    int actual = maximizeSquareHoleArea(2, 3, (int[]){2, 3}, 2, (int[]){2, 4}, 2);
    TEST_ASSERT(
        4 == actual,
        "expect 4 got %d", actual
    );
}
