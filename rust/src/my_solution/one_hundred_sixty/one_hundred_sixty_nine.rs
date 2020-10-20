pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut res = 0;
    for i in nums {
        if count == 0 {
            res = i;
        }
        count += if i == res { 1 } else { -1 };
    }
    res
}
