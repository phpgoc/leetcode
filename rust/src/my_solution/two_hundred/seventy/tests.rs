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
