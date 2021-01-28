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
#[test]
fn four_sum_count() {
    use super::four;
    assert_eq!(
        four::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
}
#[test]
fn find_content_children() {
    use super::five;
    assert_eq!(five::find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    assert_eq!(five::find_content_children(vec![1, 2, 3], vec![1, 2]), 2);
}
#[test]
fn find132pattern() {
    use super::six;
    assert!(six::find132pattern(vec![1, 3, 2]));
    assert!(!six::find132pattern(vec![1, 2, 3]));
    assert!(six::find132pattern(vec![3, 5, 0, 3, 4]));
}
#[test]
fn circular_array_loop() {
    use super::seven;
    assert!(seven::circular_array_loop(vec![2, -1, 1, 2, 2]));
}
#[test]
fn repeated_substring_pattern() {
    use super::nine;
    assert!(nine::repeated_substring_pattern(String::from("abab")));
    assert!(!nine::repeated_substring_pattern(String::from("aabab")));
}
