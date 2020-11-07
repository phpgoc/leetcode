///编写一个程序，找出第 n 个丑数。
//
// 丑数就是质因数只包含 2, 3, 5 的正整数。
pub fn nth_ugly_number(n: i32) -> i32 {
    let mut v: Vec<i64> = vec![];
    let mut b2 = 1;
    let mut b3 = 1;
    let mut b5 = 1;
    let max = std::i32::MAX as i64;
    loop {
        b3 = 1;
        loop {
            b5 = 1;
            loop {
                v.push(b2 * b3 * b5);
                b5 *= 5;
                if b2 * b3 * b5 > max {
                    break;
                }
            }
            b3 *= 3;
            if b2 * b3 > max {
                break;
            }
        }
        b2 *= 2;
        if b2 > max {
            break;
        }
    }
    v.sort();
    v[n as usize - 1] as i32
}
