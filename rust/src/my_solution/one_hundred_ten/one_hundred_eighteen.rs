pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![1]];
    if num_rows < 1 {
        return vec![];
    }
    let num_rows = num_rows as usize;
    for i in 2..=num_rows {
        let mut c_v = vec![1];
        for j in 0..i - 2 {
            c_v.push(res[i - 2][j] + res[i - 2][j + 1]);
        }
        c_v.push(1);
        res.push(c_v);
    }
    res
}
