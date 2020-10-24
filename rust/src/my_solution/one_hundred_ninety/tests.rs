use crate::my_solution::one_hundred_ninety;

#[test]
fn test_reverse_bits() {
    assert_eq!(
        one_hundred_ninety::one_hundred_ninety::reverse_bits(4294967293),
        3221225471
    );
    assert_eq!(
        one_hundred_ninety::one_hundred_ninety::reverse_bits(43261596),
        964176192
    );
}

#[test]
fn test_hamming_weight() {
    use one_hundred_ninety::one_hundred_ninety_one;
    assert_eq!(one_hundred_ninety_one::hammingWeight(11), 3);
}

#[test]
fn test_rob() {
    use one_hundred_ninety::one_hundred_ninety_eight;
    assert_eq!(one_hundred_ninety_eight::rob(vec![1, 2, 3, 1]), 4);
}
