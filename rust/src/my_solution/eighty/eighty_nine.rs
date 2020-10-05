pub fn gray_code(n: i32) -> Vec<i32> {
    let mut res = vec![0];
    let mut head = 1;
    for i in 0..n {
        for j in (0..res.len()).rev() {
            res.push(head + res[j]);
        }
        head <<= 1;
    }
    res
}
