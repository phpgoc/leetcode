use crate::my_solution::one_hundred_eighty;

#[test]
fn test_find_repeated_dna_sequences() {
    use one_hundred_eighty::one_hundred_eighty_seven;
    let expect = vec![String::from("AAAAACCCCC"), String::from("CCCCCAAAAA")];
    let result = one_hundred_eighty_seven::find_repeated_dna_sequences(String::from(
        "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
    ));
    assert_eq!(expect.len(), result.len());
    for i in result {
        assert!(expect.contains(&i));
    }
    let expect = vec![String::from("AAAAAAAAAA")];
    let result = one_hundred_eighty_seven::find_repeated_dna_sequences(String::from("AAAAAAAAAAA"));
    assert_eq!(expect.len(), result.len());
    for i in result {
        assert!(expect.contains(&i));
    }
}
