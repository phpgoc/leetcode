#[test]
fn is_power_of_four() {
    use super::two;
    assert!(two::is_power_of_four(4));
}
#[test]
fn integer_break() {
    use super::three;
    assert_eq!(three::integer_break(10), 36);
}

#[test]
fn reverse_string() {
    use super::four;
    let mut input = ['h', 'e', 'l', 'l', 'o'].to_vec();
    four::reverse_string(&mut input);
    assert_eq!(input, ['o', 'l', 'l', 'e', 'h'].to_vec());
}
#[test]
fn reverse_vowels() {
    use super::five;
    assert_eq!(
        five::reverse_vowels(String::from("hello")),
        String::from("holle")
    );
}

#[test]
fn top_k_frequent() {
    use super::seven;
    let res = seven::top_k_frequent([1, 1, 1, 2, 2, 3].to_vec(), 2);
    let expect = vec![1, 2];
    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
}
