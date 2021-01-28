#[test]
fn frequency_sort() {
    use super::one;
    assert_eq!(
        one::frequency_sort(String::from("Abb")),
        String::from("bbA")
    );
}
#[test]
fn min_moves() {
    use super::three;
    assert_eq!(three::min_moves(vec![1, 2, 3]), 3);
    assert_eq!(
        three::min_moves(vec![83, 86, 77, 15, 93, 35, 86, 92, 49, 21]),
        487
    );
}
