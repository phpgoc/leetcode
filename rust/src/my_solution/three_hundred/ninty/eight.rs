///给定一个可能含有重复元素的整数数组，要求随机输出给定的数字的索引。 您可以假设给定的数字一定存在于数组中。
//
// 注意：
// 数组大小可能非常大。 使用太多额外空间的解决方案将不会通过测试。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/random-pick-index
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub struct Solution {
    data: Vec<i32>,
}
extern "C" {
    fn srand() -> u32;
    fn rand() -> u32;
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        unsafe {
            srand();
        }
        Self { data: nums }
    }

    pub fn pick(&self, target: i32) -> i32 {
        let iter = self.data.iter().enumerate().filter(|&(_, x)| *x == target);
        let mut position = unsafe { rand() } as usize % iter.clone().count();
        for (k, _) in iter {
            if position == 0 {
                return k as i32;
            }
            position -= 1;
        }
        unreachable!();
    }
}
