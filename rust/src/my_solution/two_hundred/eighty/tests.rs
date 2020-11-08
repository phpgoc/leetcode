#[test]
fn add_operators() {
    use super::two;
    println!("{:?}", two::add_operators(String::from("123"), 6));
    println!("{:?}", two::add_operators(String::from("00"), 0));
    println!("{:?}", two::add_operators(String::from("105"), 5));
    println!("{:?}", two::add_operators(String::from("3456237490"), 9191));
    println!(
        "{:?}",
        two::add_operators(String::from("2147483648"), -2147483648)
    );
}
