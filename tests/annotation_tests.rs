use mapkit::prelude::*;

#[test]
fn point_annotation_updates() {
    let annotation = MKPointAnnotation::new(MKCoordinate::new(37.0, -122.0)).unwrap();
    annotation.set_title(Some("Title")).unwrap();
    annotation.set_subtitle(Some("Subtitle")).unwrap();
    assert_eq!(annotation.title().unwrap().as_deref(), Some("Title"));
    assert_eq!(annotation.subtitle().unwrap().as_deref(), Some("Subtitle"));
}
