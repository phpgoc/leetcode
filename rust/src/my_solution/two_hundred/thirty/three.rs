///给定一个整数 n，计算所有小于等于 n 的非负整数中数字 1 出现的个数。
pub fn count_digit_one(n: i32) -> i32 {
    let mut base: i64 = 1;
    let n = n as i64;
    let mut res = 0;
    while base <= n {
        // println!("base = {}, front = {},back = {}",base,(n / base /10) * base,base.min(0.max(n % (base*10) - base + 1)));
        res += (n / base / 10) * base + base.min(0.max(n % (base * 10) - base + 1));
        base *= 10;
    }
    res as i32
}
