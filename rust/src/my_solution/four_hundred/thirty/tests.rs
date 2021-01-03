#[test]
fn min_mutation() {
    use super::three;
    assert_eq!(
        three::min_mutation(
            String::from("AACCGGTT"),
            String::from("AAACGGTA"),
            ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
        ),
        2
    );
    assert_eq!(
        three::min_mutation(
            String::from("AACCGGTT"),
            String::from("AACCGGTA"),
            ["AACCGGTA"]
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
        ),
        1
    );
}
#[test]
fn count_segments() {
    use super::four;
    assert_eq!(
        four::count_segments(String::from("Hello, my name is John")),
        5
    );
}
