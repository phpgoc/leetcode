pub fn convert_to_title(n: i32) -> String {
    let mut res = String::new();
    if n < 1 {
        return res;
    }
    let chars = "ABCDEFGHIJLKMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
    let mut n = n as usize;
    while n != 0 {
        let left = (n - 1) % 26;

        res.push(chars[left]);

        n = (n - 1) / 26;
        // println!("left = {},res = {},n = {}", left, res, n);
    }
    res.chars().rev().collect()
}
