use structs_enums_match::t00_station::Station;

const ALL: [Station; 4] = [
    Station::Nairobi,
    Station::Mombasa,
    Station::Kisumu,
    Station::Eldoret,
];

#[test]
fn every_station_has_a_code_and_a_city() {
    let codes: Vec<&str> = ALL.iter().map(|s| s.code()).collect();
    let cities: Vec<&str> = ALL.iter().map(|s| s.city()).collect();
    assert_eq!(codes, ["NBO", "MBA", "KIS", "EDL"]);
    assert_eq!(cities, ["Nairobi", "Mombasa", "Kisumu", "Eldoret"]);
}

#[test]
fn reads_a_code_back() {
    for station in ALL {
        assert_eq!(Station::from_code(station.code()), Some(station));
    }
}

#[test]
fn reads_a_code_in_any_case() {
    assert_eq!(Station::from_code("nbo"), Some(Station::Nairobi));
    assert_eq!(Station::from_code("Mba"), Some(Station::Mombasa));
}

#[test]
fn rejects_unknown_codes() {
    assert_eq!(Station::from_code("LHR"), None);
    assert_eq!(Station::from_code(""), None);
    assert_eq!(Station::from_code("NBO "), None);
}
