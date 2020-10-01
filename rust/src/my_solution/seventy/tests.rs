use crate::my_solution::seventy;

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
