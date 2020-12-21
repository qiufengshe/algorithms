#include <iostream>
#include <cassert>
#include <vector>
#include <set>

using namespace std;

/*
给定两个数组，编写一个函数来计算它们的交集。

示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]
示例 2：

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
*/

vector<int> intersection(vector<int>& nums1, vector<int>& nums2) {
	assert(nums1.size() > 0 && nums2.size() > 0);
	set<int> map;
	for (int i = 0; i < nums1.size(); ++i) {
		map.insert(nums1[i]);
	}

	set<int> result;
	for (int i = 0; i < nums2.size(); ++i) {
		if (map.find(nums2[i]) != map.end()) {
			result.insert(nums2[i]);
		}
	}
	vector<int> newVec;
	for (set<int>::iterator iter = result.begin(); iter != result.end(); iter++) {
		newVec.push_back(*iter);
	}

	return newVec;
}