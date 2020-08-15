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

#[test]
fn test_roman_to_int() {
    use twenty::thirteen;
    assert_eq!(thirteen::roman_to_int(String::from("III")), 3);
    assert_eq!(thirteen::roman_to_int(String::from("LVIII")), 58);
    assert_eq!(thirteen::roman_to_int(String::from("MCMXCIV")), 1994);
}

#[test]
fn test_longest_common_prefix() {
    use twenty::fourteen;
    assert_eq!(
        fourteen::longest_common_prefix(
            vec!["flower", "flow", "flight"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        ),
        String::from("fl")
    );
    assert_eq!(
        fourteen::longest_common_prefix(
            vec!["a"].into_iter().map(String::from).collect::<Vec<_>>()
        ),
        String::from("a")
    );
    assert_eq!(
        fourteen::longest_common_prefix(
            vec!["aa", "ab"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        ),
        String::from("a")
    );
    assert_eq!(
        fourteen::longest_common_prefix(
            vec!["aa", "a"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        ),
        String::from("a")
    );
}

#[test]
fn test_three_sum() {
    use twenty::fifteen;
    assert_eq!(
        fifteen::three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        vec![vec![0, 0, 0]]
    );
    assert_eq!(
        fifteen::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
}

#[test]
fn test_three_sum_closest() {
    use twenty::sixteen;
    assert_eq!(sixteen::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}

#[test]
fn test_letter_combinations() {
    use twenty::seventeen;
    assert_eq!(
        seventeen::letter_combinations(String::from("23")),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}

#[test]
fn test_four_sum() {
    use twenty::eighteen;
    let result = eighteen::four_sum(vec![1, 0, -1, 0, -2, 2], 0);

    for i in vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]] {
        assert!(result.contains(&i));
    }
}
