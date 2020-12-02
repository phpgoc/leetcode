///给出一个由无重复的正整数组成的集合，找出其中最大的整除子集，子集中任意一对 (Si，Sj) 都要满足：Si % Sj = 0 或 Sj % Si = 0。
//
// 如果有多个目标子集，返回其中任何一个均可。
//
//
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/largest-divisible-subset
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }
    let mut nums = nums;
    nums.sort();
    let mut dp = nums.iter().map(|&r| vec![r]).collect::<Vec<_>>();
    let mut max_len = 0;
    let mut max_index = 0;
    for i in 1..nums.len() {
        for j in (0..i).rev() {
            if dp[i].len() <= dp[j].len() && nums[i] % nums[j] == 0 {
                dp[i] = dp[j].clone();
                dp[i].push(nums[i]);
                if dp[i].len() > max_len {
                    max_len = dp[i].len();
                    max_index = i;
                }
            }
        }
    }
    dp[max_index].to_vec()
}
