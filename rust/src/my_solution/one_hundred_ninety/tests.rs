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
