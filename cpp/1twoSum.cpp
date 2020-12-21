#include <iostream>
#include <cassert>
#include <vector>
#include <unordered_map>

using namespace std;

/*
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

 

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
*/

vector<int> twoSum(vector<int>& nums, int target) {
    assert(nums.size() > 0);
    unordered_map<int, int> unMap;
    for (int i = 0; i < nums.size(); ++i) {
        int val = target - nums[i];
        unordered_map<int, int>::iterator iter = unMap.find(val);
        if (iter != unMap.end() && i != iter->second) {
            return vector<int>{i, iter->second};
        }
        unMap[nums[i]] = i;
    }
    return vector<int>{0, 0};
}