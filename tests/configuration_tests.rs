use mapkit::prelude::*;

#[test]
fn configuration_models_build() {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0).unwrap();
    let camera = MKMapCamera::looking_at_center_coordinate(center, 750.0, 35.0, 90.0);
    let boundary = MKMapCameraBoundary::from_region(region).unwrap();
    let zoom_range = MKMapCameraZoomRange::with_min_center_coordinate_distance(250.0);
    let configuration: MKMapConfiguration = MKStandardMapConfiguration::new()
        .with_elevation_style(MKMapElevationStyle::Realistic)
        .with_emphasis_style(MKStandardMapEmphasisStyle::Muted)
        .with_shows_traffic(true)
        .into();

    assert_eq!(camera.center_coordinate, center);
    assert_eq!(boundary.region.center, center);
    assert!(zoom_range.min_center_coordinate_distance.is_some());
    assert!(MKMapCameraZoomRange::default_distance().is_finite());
    assert_eq!(configuration.kind, MKMapConfigurationKind::Standard);
}
