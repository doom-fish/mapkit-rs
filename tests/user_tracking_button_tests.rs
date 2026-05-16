use mapkit::prelude::*;

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn user_tracking_button_updates_map_view() {
    let map_view = MKMapView::new(MKScreenSize::new(256.0, 256.0)).unwrap();
    let button = MKUserTrackingButton::new(&map_view);
    button.set_visible(true).unwrap();
    button.set_tracking_mode(MKUserTrackingMode::Follow, false).unwrap();
    assert!(button.is_visible().unwrap());
    assert_eq!(button.tracking_mode().unwrap(), MKUserTrackingMode::Follow);
}
