///给你一个整数数组 nums ，设计算法来打乱一个没有重复元素的数组。
//
// 实现 Solution class:
//
// Solution(int[] nums) 使用整数数组 nums 初始化对象
// int[] reset() 重设数组到它的初始状态并返回
// int[] shuffle() 返回数组随机打乱后的结果
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/shuffle-an-array
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

    /** Resets the array to its original configuration and return it. */
    pub fn reset(&self) -> Vec<i32> {
        self.data.clone()
    }

    /** Returns a random shuffling of the array. */
    pub fn shuffle(&self) -> Vec<i32> {
        let mut res = self.data.clone();
        let len = self.data.len();
        for i in (1..len).rev() {
            let ran_usize = unsafe {
                // srand();
                rand()
            } as usize
                % (i + 1);
            // println!("{:?}", ran_usize);

            res.swap(i, ran_usize);
        }
        res
    }
}
