///统计所有小于非负整数 n 的质数的数量。
pub fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    if n <= 2 {
        return 0;
    }
    let sqrt = (n as f64).sqrt() as usize;
    let mut vec = vec![true; n];
    for i in 2..=sqrt {
        if vec[i] == false {
            continue;
        }
        for j in i..n {
            if i * j >= n {
                break;
            }
            vec[i * j] = false;
        }
    }
    let mut c = -2;
    for i in vec {
        if i {
            c += 1;
        }
    }
    c
}
