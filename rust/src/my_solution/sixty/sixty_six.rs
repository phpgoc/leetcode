pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut left = 0;
    let len = digits.len();
    digits[len - 1] += 1;
    for i in 0..len {
        let sum = digits[len - i - 1] + left;
        if sum != 10 {
            digits[len - i - 1] = sum;
            left = 0;
            break;
        } else {
            digits[len - i - 1] = 0;
            left = 1;
        }
    }
    if left == 1 {
        digits.insert(0, 1);
    }
    digits
}
