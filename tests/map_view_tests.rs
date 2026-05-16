use mapkit::prelude::*;

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn map_view_round_trip() {
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0).unwrap();
    map_view.set_region(region, false).unwrap();
    assert_eq!(map_view.region().unwrap().center, center);
    assert!(map_view.convert_coordinate_to_point(center).unwrap().x.is_finite());
}
