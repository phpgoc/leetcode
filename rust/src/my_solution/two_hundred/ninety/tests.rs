#[test]
fn word_pattern() {
    use super::word_pattern;
    assert!(word_pattern(
        String::from("abba"),
        String::from("dog cat cat dog")
    ));
}

#[test]
fn can_win_nim() {
    use super::one;
    assert!(!one::can_win_nim(4));
}

#[test]
#[allow(non_snake_case)]
fn MedianFinder() {
    use super::five;
    let mut m = five::MedianFinder::new();
    m.add_num(-1);
    println!("{:?}", m.find_median());
    m.add_num(-2);
    println!("{:?}", m.find_median());
    m.add_num(-3);
    println!("{:?}", m.find_median());

    m.add_num(4);
    m.add_num(1);
    m.add_num(2);
    assert_eq!(m.find_median(), 2.0);
}
