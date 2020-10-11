use crate::my_solution::one_hundred_ten;

#[test]
fn test_generate() {
    use one_hundred_ten::one_hundred_eighteen;
    let result = one_hundred_eighteen::generate(5);
    assert!(result.contains(vec![1, 4, 6, 4, 1].as_ref()));
}

#[test]
fn test_get_row() {
    use one_hundred_ten::one_hundred_nineteen;
    let result = one_hundred_nineteen::get_row(5);
    assert_eq!(result, vec![1, 5, 10, 10, 5, 1]);

    let result = one_hundred_nineteen::get_row(31);
    assert_eq!(
        result,
        vec![
            1, 31, 465, 4495, 31465, 169911, 736281, 2629575, 7888725, 20160075, 44352165,
            84672315, 141120525, 206253075, 265182525, 300540195, 300540195, 265182525, 206253075,
            141120525, 84672315, 44352165, 20160075, 7888725, 2629575, 736281, 169911, 31465, 4495,
            465, 31, 1
        ]
    );
}
