///在 "100 game" 这个游戏中，两名玩家轮流选择从 1 到 10 的任意整数，累计整数和，先使得累计整数和达到或超过 100 的玩家，即为胜者。
//
// 如果我们将游戏规则改为 “玩家不能重复使用整数” 呢？
//
// 例如，两个玩家可以轮流从公共整数池中抽取从 1 到 15 的整数（不放回），直到累计整数和 >= 100。
//
// 给定一个整数 maxChoosableInteger （整数池中可选择的最大数）和另一个整数 desiredTotal（累计和），判断先出手的玩家是否能稳赢（假设两位玩家游戏时都表现最佳）？
//
// 你可以假设 maxChoosableInteger 不会大于 20， desiredTotal 不会大于 300。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/can-i-win
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if max_choosable_integer * (max_choosable_integer + 1) < desired_total * 2 {
        return false;
    }
    let mut map = vec![None; 2usize.pow(max_choosable_integer as u32)];
    can_i_win_recursive(
        max_choosable_integer as usize,
        0,
        desired_total as usize,
        0,
        &mut map,
    )
}
fn can_i_win_recursive(
    max_choosable_integer: usize,
    status: usize,
    target: usize,
    total: usize,
    map: &mut Vec<Option<bool>>,
) -> bool {
    let mut first = true;
    let mut res = false;
    for i in (0..max_choosable_integer).rev() {
        let next_status = 2_usize.pow(i as u32);

        if (next_status & status) == 0 {
            if first && (i + 1 + total >= target) {
                res = true;
                break;
            }
            first = false;
            if let Some(t) = map[status + next_status] {
                if !t {
                    res = true;
                    break;
                }
            } else {
                if !can_i_win_recursive(
                    max_choosable_integer,
                    status ^ next_status,
                    target,
                    total + i + 1,
                    map,
                ) {
                    res = true;
                    break;
                }
            }
        }
    }
    map[status] = Some(res);
    res
}
