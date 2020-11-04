///给你一个长度为 n 的整数数组 nums，其中 n > 1，返回输出数组 output ，其中 output[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积。
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut left = vec![1];
    let mut right = vec![1];
    let mut res = vec![];
    let len = nums.len();
    for i in 0..len{
        left.push(nums[i]*left[i]);
    }
    for i in 0..len{
        right.push(nums[len-i-1]*right[i]);
    }
    for i in 0..len{
        res.push(left[i]*right[len-i-1]);
    }
    res
}