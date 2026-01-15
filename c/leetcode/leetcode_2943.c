#include <stdlib.h>

static inline int max(int a, int b) {
    if (a > b) {
        return a;
    }
    return b;
}

static inline int min(int a, int b) {
    if (a > b) {
        return b;
    }
    return a;
}

static int maxGap(int* bars, int s) {
    int start = *bars;
    int gap = 0;
    int length = 1;
    for(int i = 1; i < s; i++) {
        if (start + length == bars[i]) {
            length += 1;
            continue;
        }
        gap = max(gap, length);
        length = 1;
        start = bars[i];
    }
    return max(gap, length);
}
int compareIntegers(const void *a, const void *b) {
    return *((const int*)a) - *((const int*)b);
}


int maximizeSquareHoleArea(int n, int m, int* hBars, int hBarsSize, int* vBars, int vBarsSize) {
    qsort(hBars, hBarsSize, sizeof(int), compareIntegers);
    qsort(vBars, vBarsSize, sizeof(int), compareIntegers);
    int hGap = maxGap(hBars, hBarsSize) + 1;
    int vGap = maxGap(vBars, vBarsSize) + 1;
    int length = min(hGap, vGap);
    return length*length;
}
