pub fn add_binary(a: String, b: String) -> String {
    let a = u128::from_str_radix(&a, 2).unwrap();
    let b = u128::from_str_radix(&b, 2).unwrap();
    let mut c = a + b;
    let mut base = 2;
    loop {
        if base > c {
            base /= 2;
            break;
        } else {
            base *= 2;
        }
    }
    let mut result = String::new();
    while base != 0 {
        let one = c / base;
        if one == 1 {
            c -= base;
            result.push('1');
        } else {
            result.push('0');
        }
        base /= 2;
    }
    result
}
