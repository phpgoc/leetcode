use std::collections::HashSet;
///给定一个整数数组 a，其中1 ≤ a[i] ≤ n （n为数组长度）, 其中有些元素出现两次而其他元素出现一次。
//
// 找到所有出现两次的元素。
//
// 你可以不用到任何额外空间并在O(n)时间复杂度内解决这个问题吗？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-all-duplicates-in-an-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut res = HashSet::new();
    let mut nums = nums.iter().map(|&r| r as usize).collect::<Vec<_>>();
    for i in 0..nums.len() {
        while nums[i] - 1 != i && nums[nums[i] - 1] - 1 != nums[i] - 1 {
            let swap_to = nums[i] - 1;
            nums.swap(i, swap_to);
        }
        if i != nums[i] - 1 && nums[i] == nums[nums[i] - 1] {
            res.insert(nums[i] as i32);
        }
    }
    res.into_iter().collect()
}
