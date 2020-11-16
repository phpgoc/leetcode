#[test]
fn max_number() {
    use super::one;
    assert_eq!(
        one::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
        vec![9, 8, 6, 5, 3]
    );
    assert_eq!(
        one::max_number(vec![6, 7], vec![6, 0, 4], 5),
        vec![6, 7, 6, 0, 4]
    );
    assert_eq!(one::max_number(vec![8, 9], vec![3, 9], 3), vec![9, 8, 9]);
}

#[test]
fn coin_change() {
    use super::two;
    assert_eq!(two::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(two::coin_change(vec![186, 419, 83, 408], 6249), 20);
}

#[test]
fn wiggle_sort() {
    use super::four;
    let mut input = vec![1, 5, 1, 1, 6, 4];
    four::wiggle_sort(&mut input);
    let mut factor = 1;
    for i in 1..input.len() {
        assert!((input[i] - input[i - 1]) * factor > 0);
        factor *= -1;
    }
}
#[test]
fn is_power_of_three() {
    use super::six;
    assert!(six::is_power_of_three(3));
    assert!(six::is_power_of_three(1162261467));
}
