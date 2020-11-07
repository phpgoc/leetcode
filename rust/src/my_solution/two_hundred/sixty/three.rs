///编写一个程序判断给定的数是否为丑数。
//
// 丑数就是只包含质因数 2, 3, 5 的正整数。
pub fn is_ugly(num: i32) -> bool {
    if num < 1 {
        return false;
    }
    let mut num = num;
    while num & 1 == 0 {
        num >>= 1;
    }
    while num % 3 == 0 {
        num /= 3;
    }
    while num % 5 == 0 {
        num /= 5;
    }
    num == 1
}
