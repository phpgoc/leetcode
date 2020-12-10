///给定一个正整数 n ，你可以做如下操作：
//
// 如果 n 是偶数，则用 n / 2替换 n 。
// 如果 n 是奇数，则可以用 n + 1或n - 1替换 n 。
// n 变为 1 所需的最小替换次数是多少？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/integer-replacement
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn integer_replacement(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    if n == std::i32::MAX {
        return 32;
    }
    match n % 2 {
        0 => integer_replacement(n / 2) + 1,
        _ => 1 + integer_replacement(n + 1).min(integer_replacement(n - 1)),
    }
}
