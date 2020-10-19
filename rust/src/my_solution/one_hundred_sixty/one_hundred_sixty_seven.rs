pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut right = numbers.len() - 1;
    let mut left = 0;
    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }
    panic!("不该到这")
}
