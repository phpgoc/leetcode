pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in nums {
        res ^= i;
    }
    res
}
