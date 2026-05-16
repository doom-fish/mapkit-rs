use mapkit::prelude::*;

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn annotation_views_build() {
    let annotation = MKPointAnnotation::new(MKCoordinate::new(37.3349, -122.0090)).unwrap();
    annotation.set_title(Some("Apple Park")).unwrap();

    let view = MKAnnotationView::new(Some(&annotation), Some("reuse-id")).unwrap();
    view.set_can_show_callout(true).unwrap();
    view.set_center_offset(MKScreenPoint::new(4.0, -8.0)).unwrap();
    assert_eq!(view.reuse_identifier().unwrap().as_deref(), Some("reuse-id"));
    assert_eq!(view.annotation_title().unwrap().as_deref(), Some("Apple Park"));
    assert!(!MKAnnotationView::callout_info_did_change_notification().unwrap().is_empty());

    let marker = MKMarkerAnnotationView::new(Some(&annotation), Some("marker-id")).unwrap();
    marker.set_glyph_text(Some("A")).unwrap();
    marker.set_animates_when_added(true).unwrap();
    assert_eq!(marker.glyph_text().unwrap().as_deref(), Some("A"));
}
