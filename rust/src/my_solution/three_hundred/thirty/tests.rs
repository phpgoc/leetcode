#[test]
fn min_patches() {
    assert_eq!(super::min_patches(vec![1, 3], 6), 1);
    assert_eq!(super::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}

#[test]
fn find_itinerary() {
    use super::two;
    let res = two::find_itinerary(vec![
        vec![String::from("MUC"), String::from("LHR")],
        vec![String::from("JFK"), String::from("MUC")],
        vec![String::from("SFO"), String::from("SJC")],
        vec![String::from("LHR"), String::from("SFO")],
    ]);
    let expect = vec![
        String::from("JFK"),
        String::from("MUC"),
        String::from("LHR"),
        String::from("SFO"),
        String::from("SJC"),
    ];
    assert_eq!(res, expect);
    // [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
    let res = two::find_itinerary(vec![
        vec![String::from("JFK"), String::from("SFO")],
        vec![String::from("JFK"), String::from("ATL")],
        vec![String::from("SFO"), String::from("ATL")],
        vec![String::from("ATL"), String::from("JFK")],
        vec![String::from("ATL"), String::from("SFO")],
    ]);
    let expect = vec![
        String::from("JFK"),
        String::from("ATL"),
        String::from("JFK"),
        String::from("SFO"),
        String::from("ATL"),
        String::from("SFO"),
    ];
    assert_eq!(res, expect);
}
