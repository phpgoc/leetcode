///给定一个整数数组  nums，求出数组从索引 i 到 j  (i ≤ j) 范围内元素的总和，包含 i,  j 两点。
//
// update(i, val) 函数可以通过将下标为 i 的数值更新为 val，从而对数列进行修改。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/range-sum-query-mutable
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct NumArray {
    sum: Vec<i32>,
    origin: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub(crate) fn new(nums: Vec<i32>) -> Self {
        let mut last = 0;
        let mut dp = vec![];
        dp.push(0);
        for &i in &nums {
            last = last + i;
            dp.push(last);
        }
        // println!("{:?}", dp);
        Self {
            origin: nums,
            sum: dp,
        }
    }

    pub fn update(&mut self, i: i32, val: i32) {
        let diff = val - self.origin[i as usize];
        for j in i as usize..self.origin.len() {
            self.sum[j as usize + 1] += diff;
        }

        self.origin[i as usize] = val;
    }

    pub(crate) fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum[j as usize + 1] - self.sum[i as usize]
    }
}
