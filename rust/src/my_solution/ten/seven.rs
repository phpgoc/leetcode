pub fn reverse(x: i32) -> i32 {
    let mut r: i64 = 0;
    let mut x_mut = x;
    while x_mut != 0 {
        r = r * 10 + ((x_mut % 10) as i64);
        x_mut = x_mut / 10;
    }
    if r > (std::i32::MAX as i64) || r < (std::i32::MIN as i64) {
        return 0;
    }
    r as i32
}
