use mapkit::prelude::*;

#[test]
fn overlay_construction() {
    let circle = MKCircle::new(MKCoordinate::new(37.0, -122.0), 200.0).unwrap();
    let polyline = MKPolyline::new(&[
        MKCoordinate::new(37.0, -122.0),
        MKCoordinate::new(37.1, -122.1),
    ])
    .unwrap();
    let polygon = MKPolygon::new(&[
        MKCoordinate::new(37.0, -122.0),
        MKCoordinate::new(37.0, -122.2),
        MKCoordinate::new(37.2, -122.1),
    ])
    .unwrap();
    assert_eq!(polyline.point_count().unwrap(), 2);
    assert_eq!(polygon.point_count().unwrap(), 3);
    assert!(circle.radius().unwrap() > 0.0);
}
