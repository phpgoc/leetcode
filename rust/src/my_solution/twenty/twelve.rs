pub fn int_to_roman(num: i32) -> String {
    let mut r = String::new();
    let mut num_mut = num;
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
        if (n <= num_mut) {
            r.push_str(str);
            num_mut -= n;
        } else {
            stack.pop();
        }
    }
    r
}
