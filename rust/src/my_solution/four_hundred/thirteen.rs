///如果一个数列至少有三个元素，并且任意两个相邻元素之差相同，则称该数列为等差数列。
pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let mut len = 1;
    let mut diff = 0;
    let mut res = 0;
    for i in 1..a.len() {
        if a[i] - a[i - 1] == diff {
            len += 1;
        } else {
            if len >= 3 {
                res += (len - 1) * (len - 2) / 2;
            }
            diff = a[i] - a[i - 1];
            len = 2;
        }
    }
    if len >= 3 {
        res += (len - 1) * (len - 2) / 2;
    }
    res
}
