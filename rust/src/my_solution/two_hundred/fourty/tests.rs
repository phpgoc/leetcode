#[test]
fn search_matrix() {
    use super::search_matrix;
    let vec = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    assert!(search_matrix(vec.clone(), 5));
    assert!(search_matrix(vec.clone(), 6));
    assert!(!search_matrix(vec.clone(), 25));
    assert!(search_matrix(
        vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25]
        ],
        15
    ));
}
