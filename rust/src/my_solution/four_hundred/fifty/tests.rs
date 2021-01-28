#[test]
fn frequency_sort() {
    use super::one;
    assert_eq!(
        one::frequency_sort(String::from("Abb")),
        String::from("bbA")
    );
}
