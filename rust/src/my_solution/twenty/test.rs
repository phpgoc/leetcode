use crate::my_solution::twenty;

#[test]
fn test_max_area() {
    use twenty::eleven;
    assert_eq!(eleven::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(eleven::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
}

#[test]
fn test_int_to_roman() {
    use twenty::twelve;
    assert_eq!(twelve::int_to_roman(3), String::from("III"));
    assert_eq!(twelve::int_to_roman(58), String::from("LVIII"));
    assert_eq!(twelve::int_to_roman(1994), String::from("MCMXCIV"));
}
