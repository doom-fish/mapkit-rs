use std::thread;

use mapkit::prelude::*;

const fn apple_park() -> MKCoordinate {
    MKCoordinate::new(37.3349, -122.0090)
}

fn map_view_round_trip() {
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    let center = apple_park();
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0).unwrap();
    let camera = MKMapCamera::looking_at_center_coordinate(center, 750.0, 35.0, 90.0);
    let zoom_range = MKMapCameraZoomRange::with_min_center_coordinate_distance(250.0);
    let configuration: MKMapConfiguration = MKStandardMapConfiguration::new()
        .with_elevation_style(MKMapElevationStyle::Realistic)
        .with_shows_traffic(true)
        .into();

    map_view.set_region(region, false).unwrap();
    map_view.set_camera(camera, false).unwrap();
    map_view
        .set_camera_zoom_range(Some(zoom_range), false)
        .unwrap();
    map_view
        .set_camera_boundary(
            Some(MKMapCameraBoundary::from_region(region).unwrap()),
            false,
        )
        .unwrap();
    map_view.set_preferred_configuration(configuration).unwrap();

    assert!((map_view.region().unwrap().center.latitude - center.latitude).abs() < 1e-6);
    assert!(
        (map_view.camera().unwrap().center_coordinate.longitude - center.longitude).abs() < 1e-6
    );
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

fn annotation_views_build() {
    let coordinate = apple_park();
    let annotation = MKPointAnnotation::new(coordinate).unwrap();
    annotation.set_title(Some("Apple Park")).unwrap();

    let view = MKAnnotationView::new(Some(&annotation), Some("reuse-id")).unwrap();
    view.set_can_show_callout(true).unwrap();
    view.set_center_offset(MKScreenPoint::new(4.0, -8.0))
        .unwrap();
    assert_eq!(
        view.reuse_identifier().unwrap().as_deref(),
        Some("reuse-id")
    );
    assert_eq!(
        view.annotation_title().unwrap().as_deref(),
        Some("Apple Park")
    );
    assert!(!MKAnnotationView::callout_info_did_change_notification()
        .unwrap()
        .is_empty());

    let marker = MKMarkerAnnotationView::new(Some(&annotation), Some("marker-id")).unwrap();
    marker.set_glyph_text(Some("A")).unwrap();
    marker.set_animates_when_added(true).unwrap();
    assert_eq!(marker.glyph_text().unwrap().as_deref(), Some("A"));

    let map_item = MKMapItem::new(MKPlacemark::new(coordinate)).with_name("Apple Park");
    let map_item_annotation = MKMapItemAnnotation::new(&map_item).unwrap();
    let pin = MKPinAnnotationView::new(Some(&map_item_annotation), Some("pin-id")).unwrap();
    pin.set_animates_drop(true).unwrap();
    pin.set_pin_color(MKPinAnnotationColor::Purple).unwrap();
    assert_eq!(pin.pin_color().unwrap(), MKPinAnnotationColor::Purple);
    assert_eq!(
        pin.annotation_title().unwrap().as_deref(),
        Some("Apple Park")
    );

    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    map_view.add_annotation(&map_item_annotation).unwrap();
    map_view.set_shows_user_location(true).unwrap();
    let user_location = map_view.user_location().unwrap();
    let user_view = MKUserLocationView::new(Some(&user_location), Some("user-id")).unwrap();
    user_view.set_can_show_callout(true).unwrap();
    assert_eq!(
        user_view.reuse_identifier().unwrap().as_deref(),
        Some("user-id")
    );
}

fn user_tracking_button_updates_map_view() {
    let map_view = MKMapView::new(MKScreenSize::new(256.0, 256.0)).unwrap();
    let button = MKUserTrackingButton::new(&map_view);
    button.set_visible(true).unwrap();
    button
        .set_tracking_mode(MKUserTrackingMode::Follow, false)
        .unwrap();
    assert!(button.is_visible().unwrap());
    assert_eq!(button.tracking_mode().unwrap(), MKUserTrackingMode::Follow);
}

fn views_refuse_other_threads() {
    let results = thread::spawn(|| {
        let annotation = MKPointAnnotation::new(apple_park()).unwrap();
        [
            MKMapView::new(MKScreenSize::new(10.0, 10.0)).map(drop),
            MKAnnotationView::new(Some(&annotation), None).map(drop),
            MKMarkerAnnotationView::new(Some(&annotation), None).map(drop),
        ]
    })
    .join()
    .unwrap();
    for result in results {
        let error = result.expect_err("views must refuse to run off the main thread");
        assert!(error.to_string().contains("main thread"), "{error}");
    }
}

fn main() {
    let checks: [(&str, fn()); 4] = [
        ("map_view_round_trip", map_view_round_trip),
        ("annotation_views_build", annotation_views_build),
        (
            "user_tracking_button_updates_map_view",
            user_tracking_button_updates_map_view,
        ),
        ("views_refuse_other_threads", views_refuse_other_threads),
    ];
    for (name, check) in checks {
        check();
        println!("test {name} ... ok");
    }
    println!("test result: ok. {} passed; 0 failed", checks.len());
}
