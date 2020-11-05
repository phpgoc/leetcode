use crate::my_solution::two_hundred::twenty;
#[test]
fn contains_nearby_almost_duplicate() {
    assert!(twenty::contains_nearby_almost_duplicate(
        vec![1, 2, 3, 1],
        3,
        0
    ));
    assert!(!twenty::contains_nearby_almost_duplicate(
        vec![1, 5, 9, 1, 5, 9],
        2,
        3
    ));
}

#[test]
fn maximal_square() {
    use twenty::one;
    // assert_eq!(
    //     one::maximal_square(vec![
    //         vec!['1', '0', '1', '0', '0'],
    //         vec!['1', '0', '1', '1', '1'],
    //         vec!['1', '1', '1', '1', '1'],
    //         vec!['1', '0', '0', '1', '0']
    //     ]),
    //     4
    // );
    assert_eq!(one::maximal_square(vec![vec!['1'],]), 1);
}

#[test]
fn compute_area() {
    use twenty::three;
    assert_eq!(three::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    assert_eq!(three::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    assert_eq!(three::compute_area(0, 0, 0, 0, -1, -1, 1, 1), 4);
    assert_eq!(three::compute_area(-2, -2, 2, 2, -1, -1, 1, 1), 16);
    assert_eq!(three::compute_area(-2, -2, 2, 2, -2, 2, 2, 4), 24);
}
#[test]
fn calculate() {
    use twenty::four;
    assert_eq!(four::calculate(String::from("1 +1")), 2);
    assert_eq!(four::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
}

#[test]
fn calculate2() {
    use twenty::seven;
    assert_eq!(seven::calculate(String::from("0")), 0);
    assert_eq!(seven::calculate(String::from("1 *2")), 2);
    assert_eq!(seven::calculate(String::from("3/2")), 1);
    assert_eq!(seven::calculate(String::from(" 3+5 / 2")), 5);
    assert_eq!(seven::calculate(String::from(" 3+5 / 2*3")), 9);
}

#[test]
fn summary_ranges() {
    use twenty::eight;
    assert_eq!(
        eight::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec![
            String::from("0->2"),
            String::from("4->5"),
            String::from("7")
        ]
    );
}

#[test]
fn majority_element() {
    use rand::prelude::*;
    use twenty::nine;
    let mut rng = rand::thread_rng();
    let mut vec = vec![];
    for _ in 0..10 {
        let max = rng.gen_range(3, 6);
        for _ in 0..100 {
            vec.push(rng.gen_range(1, max));
        }
        assert_eq!(
            nine::majority_element(vec.clone()),
            nine::majority_element_use_counter(vec.clone())
        );
        vec.clear();
    }
    let vec = vec![1, 1, 2, 2, 2, 3, 3, 4];
    assert_eq!(
        nine::majority_element(vec.clone()),
        nine::majority_element_use_counter(vec.clone())
    );
    let vec = vec![4, 5, 3, 4, 4, 1, 0, -1, -2, 4, 6, 7, 8, 4];
    assert_eq!(
        nine::majority_element(vec.clone()),
        nine::majority_element_use_counter(vec.clone())
    );
}
