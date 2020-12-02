///给定一个正整数 num，编写一个函数，如果 num 是一个完全平方数，则返回 True，否则返回 False。
//
// 说明：不要使用任何内置的库函数，如  sqrt。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/valid-perfect-square
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn is_perfect_square(num: i32) -> bool {
    if num < 2 {
        return true;
    }
    let num = num as i64;
    let mut left = 2;
    let mut right = num / 2;
    while left <= right {
        let mid = left + (right - left) / 2;
        let square = mid * mid;
        if square == num {
            return true;
        } else if square < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}
