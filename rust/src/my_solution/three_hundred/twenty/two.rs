///给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回 -1。
//
// 你可以认为每种硬币的数量是无限的。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/coin-change
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut coins = coins.iter().map(|r| *r as usize).collect::<Vec<usize>>();
    coins.sort();
    let amount = amount as usize;
    let mut dp = vec![0; amount + 1];
    for i in 1..=amount as usize {
        let mut min = std::i32::MAX;
        for &j in &coins {
            if i < j {
                break;
            }
            if dp[i - j] != -1 {
                min = min.min(dp[i - j]);
            }
        }
        if min == std::i32::MAX {
            dp[i] = -1;
        } else {
            dp[i] = min + 1;
        }
    }
    dp[amount]
}
