use mapkit::prelude::*;

#[test]
fn look_around_request_builds() {
    let request = MKLookAroundSceneRequest::new(MKCoordinate::new(37.3349, -122.0090)).unwrap();
    let options = MKLookAroundSnapshotOptions::new(MKScreenSize::new(256.0, 256.0));
    assert!(!request.has_map_item().unwrap());
    assert!((options.size.width - 256.0).abs() < f64::EPSILON);
}
