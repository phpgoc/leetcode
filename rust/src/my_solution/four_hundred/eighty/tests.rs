#[test]
fn predict_the_winner() {
    use super::six;
    assert!(!six::predict_the_winner(vec![1, 5, 2]));
    assert!(six::predict_the_winner(vec![1, 5, 233, 7]));
}
