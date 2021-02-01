#[test]
fn rand10() {
    use super::*;
    let mut sum1 = 0;
    let mut sum2 = 0;
    for _ in 0..10 {
        sum1 = 0;
        sum2 = 0;
        for _ in 0..100000 {
            sum1 += rand10();
            sum2 += rand7();
        }
        dbg!(sum1 as f64 / 100000.0);
        dbg!(sum2 as f64 / 100000.0);
    }
}
