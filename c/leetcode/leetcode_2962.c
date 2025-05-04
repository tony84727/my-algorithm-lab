long long count_subarrays(int* nums, int num_size, int k) {
	int max = 0;
	for(int i = 0; i < num_size; i+=1) {
		if (nums[i] > max) {
			max = nums[i];
		}
	}
	long long answer = 0;
	int left = 0;
	int count = 0;
	for(int right = 0; right < num_size; right+=1) {
		if (nums[right] == max) {
			count += 1;
		}
		while (count >= k && left <= right) {
			if (nums[left] == max) {
				count -= 1;
			}
			left += 1;
		}
		answer += left;
	}
	return answer;
}
