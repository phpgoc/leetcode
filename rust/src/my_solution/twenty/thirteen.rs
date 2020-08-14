pub fn roman_to_int(s: String) -> i32 {
    let mut num = 0;
    let mut str_src = s.clone();
    let mut stack = vec![
        (1, "I"),
        (4, "IV"),
        (5, "V"),
        (9, "IX"),
        (10, "X"),
        (40, "XL"),
        (50, "L"),
        (90, "XC"),
        (100, "C"),
        (400, "CD"),
        (500, "D"),
        (900, "CM"),
        (1000, "M"),
    ];
    while let Some(&(n, str)) = stack.last() {
        if str_src.starts_with(str) {
            str_src = str_src.replacen(str, "", 1);
            num += n;
        } else {
            stack.pop();
        }
    }
    num
}