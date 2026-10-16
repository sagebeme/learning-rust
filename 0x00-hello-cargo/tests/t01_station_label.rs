use hello_cargo::t01_station_label::station_label;

#[test]
fn labels_each_station() {
    assert_eq!(station_label("NBO", "Nairobi"), "NBO (Nairobi)");
    assert_eq!(station_label("MBA", "Mombasa"), "MBA (Mombasa)");
    assert_eq!(station_label("KIS", "Kisumu"), "KIS (Kisumu)");
}
