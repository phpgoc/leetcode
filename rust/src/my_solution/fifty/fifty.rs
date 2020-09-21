pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == -1 {
        return 1.0 / x;
    } else if n == 0 {
        return 1.0;
    } else if n == 1 || x == 1.0000 {
        return x;
    }
    let half = my_pow(x, n / 2);
    if n % 2 == 0 {
        half * half
    } else {
        if n > 0 {
            half * half * x
        } else {
            half * half / x
        }
    }
}
