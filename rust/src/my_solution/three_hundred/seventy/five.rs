///我们正在玩一个猜数游戏，游戏规则如下：
//
// 我从 1 到 n 之间选择一个数字，你来猜我选了哪个数字。
//
// 每次你猜错了，我都会告诉你，我选的数字比你的大了或者小了。
//
// 然而，当你猜了数字 x 并且猜错了的时候，你需要支付金额为 x 的现金。直到你猜到我选的数字，你才算赢得了这个游戏。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/guess-number-higher-or-lower-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn get_money_amount(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 1..n {
        for s in 1..=n - i {
            let mut min = std::usize::MAX;
            for m in s + i / 2..s + i {
                min = min.min(m + dp[s][m - 1].max(dp[m + 1][s + i]));
            }
            dp[s][s + i] = min;
        }
    }
    // for i in &dp {
    //     println!("{:?}", i);
    // }
    dp[1][n] as i32
}
