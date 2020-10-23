pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    for i in 0..len / 2 {
        nums.swap(i, len - 1 - i);
    }
    for i in 0..k / 2 {
        nums.swap(i, k - 1 - i);
    }
    for i in 0..(len - k) / 2 {
        nums.swap(k + i, len - 1 - i);
    }
}
