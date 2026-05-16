use mapkit::prelude::*;

#[test]
fn map_item_coordinate_prefers_location() {
    let item = MKMapItem::from_location(
        MKCoordinate::new(37.3349, -122.0090),
        Some(MKAddress::new("1 Apple Park Way, Cupertino, CA", Some("Apple Park"))),
    );
    assert!((item.coordinate().unwrap().latitude - 37.3349).abs() < f64::EPSILON);
    assert!(MKMapItem::current_location().coordinate().is_none());
}
