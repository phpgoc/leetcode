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

#[test]
fn increasing_triplet() {
    use super::four;
    assert!(four::increasing_triplet(vec![1, 2, 3, 4, 5]));
}

#[test]
fn palindrome_pairs() {
    use super::six;
    let res = six::palindrome_pairs(
        ["abcd", "dcba", "lls", "s", "sssll"]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>(),
    );
    let expect = vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]];
    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
    let res = six::palindrome_pairs(
        ["bat", "tab", "cat"]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>(),
    );
    let expect = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
    let res = six::palindrome_pairs(
        ["a", ""]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>(),
    );
    let expect = vec![vec![0, 1], vec![1, 0]];

    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
    let res = six::palindrome_pairs(
        [
            "bb", "bababab", "baab", "abaabaa", "aaba", "", "bbaa", "aba", "baa", "b",
        ]
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>(),
    );
    let expect = vec![
        [0, 5],
        [0, 9],
        [9, 0],
        [5, 0],
        [1, 5],
        [5, 1],
        [2, 5],
        [8, 2],
        [5, 2],
        [4, 3],
        [7, 4],
        [4, 8],
        [6, 0],
        [7, 5],
        [5, 7],
        [8, 9],
        [9, 5],
        [5, 9],
    ]
    .iter()
    .map(|r| r.to_vec())
    .collect::<Vec<_>>();
    //use std::collections::HashSet;
    // println!(
    //     "{:?}",
    //     expect
    //         .iter()
    //         .cloned()
    //         .collect::<HashSet<_>>()
    //         .difference(&res.iter().cloned().collect::<HashSet<_>>())
    // );

    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
}
