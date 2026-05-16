use mapkit::prelude::*;

#[test]
fn snapshotter_produces_snapshot() {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let options = MKMapSnapshotOptions::new(MKScreenSize::new(300.0, 180.0))
        .with_region(MKCoordinateRegion::with_distance(center, 800.0, 800.0).unwrap());
    let snapshotter = MKMapSnapshotter::new(&options).unwrap();
    let snapshot = snapshotter.start().unwrap();
    assert!(snapshot.image_byte_len().unwrap() > 0);
    assert!(snapshot.point_for_coordinate(center).unwrap().x.is_finite());
}
