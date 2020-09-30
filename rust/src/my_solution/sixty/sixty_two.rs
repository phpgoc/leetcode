pub fn unique_paths(m: i32, n: i32) -> i32 {
    // 相乘在除之前可能会溢出
    let mut result: i64 = 1;
    let min_v = (m.min(n) - 1) as i64;
    let max_v = (m.max(n) - 1) as i64;
    for i in 1..=min_v {
        result *= i + max_v;
        result /= i;
    }
    result as i32
}
