use crate::my_solution::one_hundred_thirty;

#[test]
fn test_solve() {
    let mut input = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let expect = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    one_hundred_thirty::one_hundred_thirty::solve(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_partition() {
    use one_hundred_thirty::one_hundred_thirty_one;
    let expect = vec![
        vec![String::from("aa"), String::from("b")],
        vec![String::from("a"), String::from("a"), String::from("b")],
    ];
    let result = one_hundred_thirty_one::partition(String::from("aab"));
    assert_eq!(result.len(), expect.len());
    for i in expect {
        assert!(result.contains(&i));
    }
}

#[test]
fn test_min_cut() {
    use one_hundred_thirty::one_hundred_thirty_two;
    assert_eq!(one_hundred_thirty_two::min_cut(String::from("aab")), 1);
    assert_eq!(one_hundred_thirty_two::min_cut(String::from("aabc")), 2);
    assert_eq!(one_hundred_thirty_two::min_cut(String::from("aabaac")), 1);
    assert_eq!(one_hundred_thirty_two::min_cut(String::from("apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp")), 452);
}

#[test]
fn test_can_complete_circuit() {
    use one_hundred_thirty::one_hundred_thirty_four;
    assert_eq!(
        one_hundred_thirty_four::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(
        one_hundred_thirty_four::can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]),
        0
    );
}

#[test]
fn test_candy() {
    use one_hundred_thirty::one_hundred_thirty_five;
    assert_eq!(one_hundred_thirty_five::candy(vec![1, 0, 2]), 5);
    assert_eq!(one_hundred_thirty_five::candy(vec![1, 2, 2]), 4);
    assert_eq!(one_hundred_thirty_five::candy(vec![1, 3, 2, 2, 1]), 7);
    assert_eq!(
        one_hundred_thirty_five::candy(vec![0, 1, 2, 5, 3, 2, 7]),
        15
    );
}

#[test]
fn test_single_number() {
    use one_hundred_thirty::one_hundred_thirty_six;
    assert_eq!(one_hundred_thirty_six::single_number(vec![1, 2, 2]), 1);
}
