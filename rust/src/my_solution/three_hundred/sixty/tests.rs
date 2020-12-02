#[test]
fn can_measure_water() {
    use super::five;
    assert!(five::can_measure_water(3, 4, 5));
    assert!(!five::can_measure_water(8, 2, 3));
}

#[test]
fn is_perfect_square() {
    use super::seven;
    assert!(seven::is_perfect_square(16));
}

#[test]
fn largest_divisible_subset() {
    use super::eight;
    assert_eq!(
        eight::largest_divisible_subset(vec![1, 2, 4, 8]),
        vec![1, 2, 4, 8]
    );
}
