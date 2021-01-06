pub fn arrange_coins(n: i32) -> i32 {
    let n = n as i64 * 2;
    let mut res = 1;
    loop {
        if res * (res + 1) < n {
            res += 1;
        } else if res * (res + 1) == n {
            return res as i32;
        } else {
            return res as i32 - 1;
        }
    }
}
