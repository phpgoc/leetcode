pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = std::i32::MIN;
    for i in &nums {
        max_sum = max_sum.max(*i);
    }

    let mut sum = 0;
    for i in nums {
        sum += i;
        if sum <= 0 {
            sum = 0;
        } else {
            max_sum = max_sum.max(sum);
        }
    }
    max_sum
}
