pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let n = nums.len();
    let mut t = vec![];
    for mut mask in 0..(1 << n) {
        t.clear();
        for i in 0..n {
            if (mask & (1 << i)) == 0 {
                t.push(nums[i]);
            }
        }
        result.push(t.clone());
    }
    result
}
