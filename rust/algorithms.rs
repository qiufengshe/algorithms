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


/*
示例 1：

输入：nums = [3,2,2,3], val = 3
输出：2, nums = [2,2]
解释：函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。你不需要考虑数组中超出新长度后面的元素。例如，函数返回的新长度为 2 ，而 nums = [2,2,3,3] 或 nums = [2,2,0,0]，也会被视作正确答案。
示例 2：

输入：nums = [0,1,2,2,3,0,4,2], val = 2
输出：5, nums = [0,1,4,0,3]
解释：函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。注意这五个元素可为任意顺序。你不需要考虑数组中超出新长度后面的元素。

*/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k: usize = 0;
    let size = nums.len();
    let mut i: usize = 0;
    while i < size {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
        i += 1;
    }

    return k as i32;
}


/*
    给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
    示例 :
    输入: [0, 1, 0, 3, 12]
    输出 : [1, 3, 12, 0, 0]
    说明 :
    必须在原数组上操作，不能拷贝额外的数组。
    尽量减少操作次数。
*/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let total: usize = nums.len();
    let mut new_vec: Vec<i32> = Vec::new();
    let mut index: usize = 0;
    while index < total {
        if nums[index] != 0 {
            new_vec.push(nums[index]);
        }
        index += 1;
    }

    for (i, item) in new_vec.iter().enumerate() {
        nums[i] = *item;
    }

    for index in new_vec.len()..total {
        nums[index] = 0;
    }
}