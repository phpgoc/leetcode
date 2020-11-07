#[test]
fn number_to_words() {
    use super::three;
    assert_eq!(
        three::number_to_words(123),
        String::from("One Hundred Twenty Three")
    );
    assert_eq!(
        three::number_to_words(1234567891),
        String::from("One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One")
    );
}

#[test]
fn h_index() {
    use super::four;
    assert_eq!(four::h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(four::h_index(vec![3, 0, 4, 7, 6, 1, 5]), 4);
}

#[test]
fn h_index2() {
    use super::five;
    assert_eq!(five::h_index(vec![0, 1, 3, 5, 6]), 3);
    assert_eq!(five::h_index(vec![0, 1, 3, 4, 5, 7, 6,]), 4);
}
