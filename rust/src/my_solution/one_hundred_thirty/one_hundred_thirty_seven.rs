pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut seen_once = 0;
    let mut seen_twice = 0;
    for i in nums {
        seen_once = !seen_twice & (seen_once ^ i);
        seen_twice = !seen_once & (seen_twice ^ i);
    }
    seen_once
}
