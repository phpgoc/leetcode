use std::collections::HashMap;

///给定一个表示分数的非负整数数组。 玩家 1 从数组任意一端拿取一个分数，随后玩家 2 继续从剩余数组任意一端拿取分数，然后玩家 1 拿，…… 。每次一个玩家只能拿取一个分数，分数被拿取之后不再可取。直到没有剩余分数可取时游戏结束。最终获得分数总和最多的玩家获胜。
//
// 给定一个表示分数的数组，预测玩家1是否会成为赢家。你可以假设每个玩家的玩法都会使他的分数最大化。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/predict-the-winner
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len == 1 {
        return true;
    }
    let mut memory = HashMap::new();
    max_fetch(&nums, 0, len - 1, &mut memory) >= 0
}
fn max_fetch(
    nums: &Vec<i32>,
    left: usize,
    right: usize,
    memory: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if let Some(n) = memory.get(&(left, right)) {
        return *n;
    }
    if left == right {
        return nums[left];
    }
    let max = (nums[left] - max_fetch(nums, left + 1, right, memory))
        .max(nums[right] - max_fetch(nums, left, right - 1, memory));
    memory.insert((left, right), max);
    max
}
