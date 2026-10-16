use ownership_memory::t100_chunk_roster::chunk_roster;

#[test]
fn splits_into_teams() {
    assert_eq!(
        chunk_roster(&["A", "B", "C", "D", "E"], 2),
        vec![vec!["A", "B"], vec!["C", "D"], vec!["E"]]
    );
}

#[test]
fn one_team_when_the_size_is_big_enough() {
    assert_eq!(chunk_roster(&["A", "B"], 10), vec![vec!["A", "B"]]);
}

#[test]
fn a_team_size_of_zero_does_not_crash() {
    assert!(chunk_roster(&["A"], 0).is_empty());
}

#[test]
fn an_empty_roster_has_no_teams() {
    assert!(chunk_roster(&[], 3).is_empty());
}
