#include <iostream>
#include <cassert>
#include <vector>

using namespace std;

/*
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

示例 :

输入: [0, 1, 0, 3, 12]
输出 : [1, 3, 12, 0, 0]
说明 :

必须在原数组上操作，不能拷贝额外的数组。
尽量减少操作次数。
*/


void moveZeroes(vector<int>& nums) {
	assert(nums.size() > 0);
	int newCount = 0, total = nums.size();
	vector<int> newVec;
	for (int i = 0; i < total; ++i) {
		if (nums[i] > 0) {
			newVec.push_back(nums[i]);
		}
	}
	newCount = newVec.size();
	for (int i = 0; i < newCount; ++i) {
		nums[i] = newVec[i];
	}

	for (int i = newCount; i < total; ++i) {
		nums[i] = 0;
	}
}