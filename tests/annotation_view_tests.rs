use mapkit::prelude::*;

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn annotation_views_build() {
    let coordinate = MKCoordinate::new(37.3349, -122.0090);
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
