#include <stdlib.h>
int* two_sum(int* nums, int num_size, int target, int* return_size) {
	for(int i = 0; i < num_size; i+=1) {
		for(int j = 0; j < num_size; j+=1) {
			if (i != j && nums[i]+nums[j] == target)
			{
				*return_size = 2;
				int* found = calloc(2, sizeof(int));
				found[0] = i;
				found[1] = j;
				return found;
			}
		}
	}
	return NULL;
}
