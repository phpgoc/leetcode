#[test]
fn can_measure_water() {
    use super::five;
    assert!(five::can_measure_water(3, 4, 5));
    assert!(!five::can_measure_water(8, 2, 3));
}
