use crate::my_solution::one_hundred_twenty;

#[test]
fn test_minimum_total() {
    let input = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    assert_eq!(
        one_hundred_twenty::one_hundred_twenty::minimum_total(input),
        11
    );
}

#[test]
fn test_max_profit() {
    use one_hundred_twenty::one_hundred_twenty_one;
    assert_eq!(
        one_hundred_twenty_one::max_profit(vec![7, 1, 5, 3, 6, 4]),
        5
    );
    assert_eq!(one_hundred_twenty_one::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit2() {
    use one_hundred_twenty::one_hundred_twenty_two;
    //    assert_eq!(
    //        one_hundred_twenty_two::max_profit(vec![7, 1, 5, 3, 6, 4]),
    //        7
    //    );
    //    assert_eq!(one_hundred_twenty_two::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(
        one_hundred_twenty_two::max_profit(vec![3, 2, 6, 5, 0, 3]),
        7
    );
}

#[test]
fn test_longest_consecutive() {
    use one_hundred_twenty::one_hundred_twenty_eight;
    assert_eq!(
        one_hundred_twenty_eight::longest_consecutive(vec![100, 4, 200, 1, 3, 2]),
        4
    );
}
