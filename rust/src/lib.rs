use std::time::Instant;

#[allow(dead_code)]
#[allow(unused_parens)]
#[allow(unused_assignments)]
pub mod my_solution;
pub mod no_leetcode;

pub fn cal_runtime(f: fn() -> ()) {
    let start = Instant::now();
    f();
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));
}
#[cfg(test)]
mod tests{
    #[test]
    fn test_calruntime() {
        super::cal_runtime(|| {
            let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            v.sort();
            assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        });
    }
}
