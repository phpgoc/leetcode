#[test]
fn lfucache() {
    use super::LFUCache;
    let mut model = LFUCache::new(2);
    model.put(2, 3);
    assert_eq!(model.get(2), 3);
    model.put(3, 4);
    model.put(4, 5);
    assert_eq!(model.get(3), -1);
}
#[test]
fn hamming_distance() {
    use super::one;
    assert_eq!(one::hamming_distance(1, 4), 2);
    assert_eq!(one::hamming_distance(1, 5), 1);
}
#[test]
fn min_moves2() {
    use super::two;
    assert_eq!(two::min_moves2(vec![1, 2, 3]), 2);
    assert_eq!(two::min_moves2(vec![1, 2, 2, 3]), 2);
    assert_eq!(two::min_moves2(vec![1, 0, 0, 8, 6]), 14);
}
#[test]
fn island_perimeter() {
    use super::three;
    let grid = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
        .iter()
        .map(|r| r.to_vec())
        .collect::<Vec<_>>();
    assert_eq!(three::island_perimeter(grid), 16);
}
#[test]
fn can_i_win() {
    use super::four;
    assert!(!four::can_i_win(10, 11));
}
#[test]
fn find_substring_in_wrapround_string() {
    use super::seven;
    assert_eq!(
        seven::find_substring_in_wrapround_string(String::from("cdefghefghijklmnopqrstuvwxmnijklmnopqrstuvbcdefghijklmnopqrstuvwabcddefghijklfghijklmabcdefghijklmnopqrstuvwxymnopqrstuvwxyz")),
        339
    );
    assert_eq!(
        seven::find_substring_in_wrapround_string(String::from("zabd")),
        7
    );
}
#[test]
fn valid_ip_address() {
    use super::eight;
    assert_eq!(
        eight::valid_ip_address(String::from("172.16.254.1")),
        String::from("IPv4")
    );
    assert_eq!(
        eight::valid_ip_address(String::from("2001:0db8:85a3:0:0:8A2E:0370:7334")),
        String::from("IPv6")
    );
    assert_eq!(
        eight::valid_ip_address(String::from("256.256.256.256")),
        String::from("Neither")
    );
    assert_eq!(
        eight::valid_ip_address(String::from("2001:0db8:85a3:0:0:8A2E:0370:7334:")),
        String::from("Neither")
    );
}
