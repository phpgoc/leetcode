///给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。
//
// 整数 n 是 3 的幂次方需满足：存在整数 x 使得 n == 3x
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/power-of-three
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn is_power_of_three(n: i32) -> bool {
    [
        1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969,
        14348907, 43046721, 129140163, 387420489, 1162261467,
    ]
    .contains(&n)
}
