use mapkit::prelude::*;

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn map_view_round_trip() {
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0).unwrap();
    let camera = MKMapCamera::looking_at_center_coordinate(center, 750.0, 35.0, 90.0);
    let zoom_range = MKMapCameraZoomRange::with_min_center_coordinate_distance(250.0);
    let configuration: MKMapConfiguration = MKStandardMapConfiguration::new()
        .with_elevation_style(MKMapElevationStyle::Realistic)
        .with_shows_traffic(true)
        .into();

    map_view.set_region(region, false).unwrap();
    map_view.set_camera(camera, false).unwrap();
    map_view.set_camera_zoom_range(Some(zoom_range), false).unwrap();
    map_view
        .set_camera_boundary(Some(MKMapCameraBoundary::from_region(region).unwrap()), false)
        .unwrap();
    map_view.set_preferred_configuration(configuration).unwrap();

    assert_eq!(map_view.region().unwrap().center, center);
    assert_eq!(map_view.camera().unwrap().center_coordinate, center);
    assert_eq!(
        map_view.preferred_configuration().unwrap().unwrap().kind,
        MKMapConfigurationKind::Standard
    );
    assert!(map_view
        .convert_coordinate_to_point(center)
        .unwrap()
        .x
        .is_finite());
}
