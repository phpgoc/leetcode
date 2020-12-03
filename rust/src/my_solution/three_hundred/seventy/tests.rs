#[test]
fn super_pow() {
    use super::two;
    assert_eq!(two::super_pow(1, vec![2, 3, 4, 5, 6]), 1);
    assert_eq!(two::super_pow(2, vec![1, 0]), 1024);
    assert_eq!(two::super_pow(2147483647, vec![2, 0, 0]), 1198);
}

#[test]
fn k_smallest_pairs() {
    use super::three;
    assert_eq!(
        three::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec![vec![1, 2], vec![1, 4], vec![1, 6]]
    );
    assert_eq!(
        three::k_smallest_pairs(vec![1, 2], vec![3], 3),
        vec![vec![1, 3], vec![2, 3]]
    );
    assert_eq!(
        three::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 10),
        [
            [1, 1],
            [1, 1],
            [1, 2],
            [2, 1],
            [1, 2],
            [1, 3],
            [2, 2],
            [1, 3],
            [2, 3]
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect::<Vec<_>>()
    );
}
#[test]
fn get_money_amount() {
    use super::five;
    assert_eq!(five::get_money_amount(10), 16);
}
#[test]
fn wiggle_max_length() {
    use super::six;
    assert_eq!(
        six::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
        7
    );
    assert_eq!(six::wiggle_max_length(vec![3, 3, 3, 2, 5]), 3);
}
#[test]
fn combination_sum4() {
    use super::seven;
    assert_eq!(seven::combination_sum4(vec![1, 2, 3], 4), 7);
}
