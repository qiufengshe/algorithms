/*
给定一个整数数组 nums和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那两个整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。

你可以按任意顺序返回答案。


示例 1：

输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1]
 */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let len: usize = nums.len();
    let mut i: usize = 0;
    while i < len  {
        let val: i32 = target - nums[i];
        if map.contains_key(&val) {
            return vec![map[&val], i as i32];
        }
        map.insert(nums[i], i as i32);
        i += 1;
    }

    return vec![0, 0];
}