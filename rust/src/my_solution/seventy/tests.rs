use crate::my_solution::seventy;
use crate::my_solution::seventy::seventy_three::set_zeroes;

#[test]
fn test_climb_stairs() {
    assert_eq!(seventy::seventy::climb_stairs(3), 3);
    assert_eq!(seventy::seventy::climb_stairs(2), 2);
    assert_eq!(seventy::seventy::climb_stairs(5), 8);
}

#[test]
fn test_simplify_path() {
    use seventy::seventy_one;
    assert_eq!(
        seventy_one::simplify_path(String::from("/home//foo/")),
        String::from("/home/foo")
    );
    assert_eq!(
        seventy_one::simplify_path(String::from("/a/../../b/../c//.//")),
        String::from("/c")
    );
}

#[test]
fn test_min_distance() {
    use seventy::seventy_two;
    assert_eq!(
        seventy_two::min_distance(String::from("horse"), String::from("ros")),
        3
    );
    assert_eq!(
        seventy_two::min_distance(
            String::from("zoologicoarchaeologist"),
            String::from("zoogeologist"),
        ),
        10
    );
}

#[test]
fn test_set_zeroes() {
    use seventy::seventy_three;
    let mut input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut input);
    assert_eq!(input, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
}

#[test]
fn test_search_matrix() {
    use seventy::seventy_four;
    assert!(seventy_four::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
        3,
    ));
}
