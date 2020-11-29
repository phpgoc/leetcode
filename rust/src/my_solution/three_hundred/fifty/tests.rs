#[test]
fn intersect() {
    let mut res = super::intersect(vec![1, 2, 2, 1], vec![2, 2]);
    res.sort();
    assert_eq!(res, [2, 2]);
}

#[test]
fn summary_ranges() {
    use super::two::SummaryRanges;
    let mut m = SummaryRanges::new();
    m.add_num(1);
    println!("{:?}", m.get_intervals());
    m.add_num(3);
    println!("{:?}", m.get_intervals());
    m.add_num(7);
    println!("{:?}", m.get_intervals());
    m.add_num(2);
    println!("{:?}", m.get_intervals());
    m.add_num(6);
    println!("{:?}", m.get_intervals());
}
