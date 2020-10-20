use crate::my_solution::one_hundred_sixty;
#[test]
fn test_find_peak_element() {
    assert_eq!(
        one_hundred_sixty::one_hundred_sixty::find_peak_element(vec![1, 2, 3, 1]),
        2
    );
}

#[test]
fn test_maximum_gap() {
    use one_hundred_sixty::one_hundred_sixty_four;
    assert_eq!(one_hundred_sixty_four::maximum_gap(vec![3, 6, 9, 1]), 3);
}

#[test]
fn test_compare_version() {
    use one_hundred_sixty::one_hundred_sixty_five;
    assert_eq!(
        one_hundred_sixty_five::compare_version(String::from("0.1"), String::from("1.1")),
        -1
    );
    assert_eq!(
        one_hundred_sixty_five::compare_version(String::from("1"), String::from("1.0.00")),
        0
    );
}

#[test]
fn test_fraction_to_decimal() {
    use one_hundred_sixty::one_hundred_sixty_six;
    assert_eq!(
        one_hundred_sixty_six::fraction_to_decimal(1, 2),
        String::from("0.5")
    );
    assert_eq!(
        one_hundred_sixty_six::fraction_to_decimal(2, 3),
        String::from("0.(6)")
    );
    assert_eq!(
        one_hundred_sixty_six::fraction_to_decimal(1, 333),
        String::from("0.(003)")
    );
    assert_eq!(
        one_hundred_sixty_six::fraction_to_decimal(1, 17),
        String::from("0.(0588235294117647)")
    );
    assert_eq!(
        one_hundred_sixty_six::fraction_to_decimal(-1, -2147483648),
        String::from("0.0000000004656612873077392578125")
    );
}

#[test]
fn test_two_sum() {
    use one_hundred_sixty::one_hundred_sixty_seven;
    assert_eq!(
        one_hundred_sixty_seven::two_sum(vec![2, 7, 11, 15], 9),
        vec![1, 2]
    );
}

#[test]
fn test_convert_to_title() {
    use one_hundred_sixty::one_hundred_sixty_eight;
    assert_eq!(
        one_hundred_sixty_eight::convert_to_title(28),
        String::from("AB")
    );
    assert_eq!(
        one_hundred_sixty_eight::convert_to_title(701),
        String::from("ZY")
    );
}
