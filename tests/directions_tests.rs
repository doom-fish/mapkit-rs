use mapkit::prelude::*;

#[test]
fn directions_request_builds() {
    let source = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.0, -122.0)));
    let destination = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.1, -122.1)));
    let directions = MKDirections::new(&MKDirectionsRequest::new(source, destination)).unwrap();
    assert!(!directions.is_calculating());
}
