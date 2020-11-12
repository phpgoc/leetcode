///给定一个整数数组  nums，求出数组从索引 i 到 j（i ≤ j）范围内元素的总和，包含 i、j 两点。
#[allow(non_snake_case)]
pub struct NumArray {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut last = 0;
        let mut dp = vec![];
        dp.push(0);
        for i in nums {
            last = last + i;
            dp.push(last);
        }
        // println!("{:?}", dp);
        Self { data: dp }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.data[j as usize + 1] - self.data[i as usize]
    }
}
