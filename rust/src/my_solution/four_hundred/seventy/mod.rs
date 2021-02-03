use rand::{thread_rng, Rng};
///已有方法 rand7 可生成 1 到 7 范围内的均匀随机整数，试写一个方法 rand10 生成 1 到 10 范围内的均匀随机整数。
pub fn rand10() -> i32 {
    let random_num = loop {
        let mut sum = (rand7() - 1) * 7 + rand7();
        if sum <= 40 {
            break sum;
        }
        sum = (sum % 10) * 7 + rand7();
        if sum <= 60 {
            break sum;
        }
        sum = (sum % 10) * 7 + rand7();
        if sum <= 20 {
            break sum;
        }
    };
    random_num % 10 + 1
}

pub fn rand7() -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(1, 8)
}
pub mod five;
pub mod six;
#[cfg(test)]
mod tests;
pub mod three;
pub mod two;
