use crate::my_solution::one_hundred_forty;

#[test]
fn test_word_break() {
    let expect = vec![String::from("cats and dog"), String::from("cat sand dog")];
    let result = one_hundred_forty::one_hundred_forty::word_break(
        String::from("catsanddog"),
        vec![
            String::from("cat"),
            String::from("cats"),
            String::from("and"),
            String::from("sand"),
            String::from("dog"),
        ],
    );
    assert_eq!(expect.len(), result.len());
    for i in result {
        assert!(expect.contains(&i));
    }
}
