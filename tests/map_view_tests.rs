use mapkit::prelude::*;

#[test]
fn map_view_default_reuse_identifiers_exist() {
    assert!(!MKMapView::default_annotation_view_reuse_identifier()
        .unwrap()
        .is_empty());
    assert!(
        !MKMapView::default_cluster_annotation_view_reuse_identifier()
            .unwrap()
            .is_empty()
    );
}
