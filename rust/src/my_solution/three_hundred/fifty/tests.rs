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

#[test]
fn max_envelopes() {
    use super::four;
    assert_eq!(
        four::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
        3
    );
    assert_eq!(
        four::max_envelopes(vec![
            vec![2, 100],
            vec![3, 200],
            vec![4, 300],
            vec![5, 500],
            vec![5, 400],
            vec![5, 250],
            vec![6, 370],
            vec![6, 360],
            vec![7, 380]
        ]),
        5
    );
}

#[test]
fn twitter() {
    use super::five::Twitter;
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    println!("{:?}", twitter.get_news_feed(1));

    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    println!("{:?}", twitter.get_news_feed(1));

    twitter.unfollow(1, 2);
}

#[test]
fn count_numbers_with_unique_digits() {
    use super::seven;
    assert_eq!(seven::count_numbers_with_unique_digits(2), 91);
}
