pub fn is_palindrome(x: i32) -> bool {
    let x_str = format!("{}", x);
    let x_reserve: String = x_str.chars().rev().collect();
    return x_str.eq(&x_reserve);
}
