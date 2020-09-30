pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let x = x as i64;
    let mut left = 1;
    let mut right = x;
    let mut ans = -1;
    while left <= right {
        let mid = left + (right - left) / 2;
        let mul: i64 = mid * mid;
        if mul > x {
            right = mid - 1;
        } else if mul < x {
            ans = mid;
            left = mid + 1;
        } else {
            return mid as i32;
        }
    }
    ans as i32
}
