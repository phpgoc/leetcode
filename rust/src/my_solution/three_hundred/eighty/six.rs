use std::cmp::Ordering;
///给定一个整数 n, 返回从 1 到 n 的字典顺序。
//
// 例如，
//
// 给定 n =1 3，返回 [1,10,11,12,13,2,3,4,5,6,7,8,9] 。
//
// 请尽可能的优化算法的时间复杂度和空间复杂度。 输入的数据 n 小于等于 5,000,000。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/lexicographical-numbers
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut origin = (1..=n).collect::<Vec<_>>();
    origin.sort_by(|o_a, o_b| {
        let mut a = *o_a;
        let mut b = *o_b;
        let mut base = 1;
        let mut a_base = 1;
        let mut b_base = 1;
        let mut b_a = 0;
        let mut b_b = 0;
        while b_a == 0 || b_b == 0 {
            if b_a == 0 && 10 * base > a {
                a_base = base;
                b_a = a / a_base;
                a -= b_a * a_base;
            }
            if b_b == 0 && 10 * base > b {
                b_base = base;
                b_b = b / b_base;
                b -= b_b * b_base;
            }
            base *= 10;
        }
        loop {
            match b_a.cmp(&b_b) {
                Ordering::Equal => {
                    if a == 0 && b == 0 {
                        return a_base.cmp(&a_base);
                    }
                    if a == 0 {
                        return Ordering::Less;
                    }
                    if b == 0 {
                        return Ordering::Greater;
                    }
                    a_base /= 10;
                    b_base /= 10;
                    b_a = a / a_base;
                    a -= b_a * a_base;
                    b_b = b / b_base;
                    b -= b_b * b_base;
                }
                _a => return _a,
            }
        }
    });
    origin
}
