pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut result = vec![vec![0; n]; n];

    let mut v = 1;

    for i in 0..=n / 2 {
        for j in i..n - i {
            result[i][j] = v;
            v += 1;
        }
        for j in i + 1..n - i {
            result[j][n - i - 1] = v;
            v += 1;
        }
        for j in (i..n - i - 1).rev() {
            result[n - i - 1][j] = v;
            v += 1;
        }
        for j in (i + 1..n - i - 1).rev() {
            result[j][i] = v;
            v += 1;
        }
    }
    //    println!("{:?}", result);
    result
}
