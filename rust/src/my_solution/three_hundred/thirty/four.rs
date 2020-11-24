///给定一个未排序的数组，判断这个数组中是否存在长度为 3 的递增子序列。
//
// 数学表达式如下:
//
// 如果存在这样的 i, j, k,  且满足 0 ≤ i < j < k ≤ n-1，
// 使得 arr[i] < arr[j] < arr[k] ，返回 true ; 否则返回 false 。
// 说明: 要求算法的时间复杂度为 O(n)，空间复杂度为 O(1) 。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/increasing-triplet-subsequence
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut dp = [std::i32::MAX; 2];
    for i in nums {
        if i > dp[1] {
            return true;
        }
        if i <= dp[0] {
            dp[0] = i;
            continue;
        }
        if i < dp[1] {
            dp[1] = i;
        }
    }
    false
}
