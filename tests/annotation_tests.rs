use mapkit::prelude::*;

#[test]
fn point_annotation_updates() {
    let annotation = MKPointAnnotation::new(MKCoordinate::new(37.0, -122.0)).unwrap();
    annotation.set_title(Some("Title")).unwrap();
    annotation.set_subtitle(Some("Subtitle")).unwrap();
    assert_eq!(annotation.title().unwrap().as_deref(), Some("Title"));
    assert_eq!(annotation.subtitle().unwrap().as_deref(), Some("Subtitle"));
}

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn map_item_annotation_wraps_map_item() {
    let coordinate = MKCoordinate::new(37.3349, -122.0090);
    let map_item = MKMapItem::new(MKPlacemark::new(coordinate)).with_name("Apple Park");
    let annotation = MKMapItemAnnotation::new(&map_item).unwrap();

    assert_eq!(annotation.coordinate().unwrap(), coordinate);
    assert_eq!(annotation.title().unwrap().as_deref(), Some("Apple Park"));
    assert_eq!(
        annotation.map_item().unwrap().name.as_deref(),
        Some("Apple Park")
    );
}
