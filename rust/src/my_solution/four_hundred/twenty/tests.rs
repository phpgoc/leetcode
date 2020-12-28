#[test]
fn original_digits() {
    use super::three;
    assert_eq!(
        three::original_digits(String::from("owoztneoer")),
        String::from("012")
    );
}
