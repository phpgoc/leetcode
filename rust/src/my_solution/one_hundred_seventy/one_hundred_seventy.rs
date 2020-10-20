pub fn title_to_number(s: String) -> i32 {
    let mut res = 0;
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
    let s = s.chars().rev().collect::<Vec<_>>();
    let mut base = 1;
    for i in s {
        let p = chars.iter().position(|&r| r == i).unwrap();
        res += (p + 1) * base;
        base *= 26;
    }
    res as i32
}
