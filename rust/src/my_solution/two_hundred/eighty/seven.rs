///给定一个包含 n + 1 个整数的数组 nums，其数字都在 1 到 n 之间（包括 1 和 n），可知至少存在一个重复的整数。假设只有一个重复的整数，找出这个重复的数。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-the-duplicate-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let nums = nums
        .iter()
        .map(|&i| (i as usize).into())
        .collect::<Vec<usize>>();
    let mut fast = nums[nums[0]];
    let mut slow = nums[0];
    let mut finder = 0;
    while fast != slow {
        fast = nums[nums[fast]];
        slow = nums[slow];
    }
    while finder != slow {
        slow = nums[slow];
        finder = nums[finder];
    }
    finder as i32
}
