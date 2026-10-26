use mapkit::prelude::*;

#[test]
fn map_view_default_reuse_identifiers_exist() {
    assert_ne!(
        MKMapView::default_annotation_view_reuse_identifier().unwrap(),
        ""
    );
    assert_ne!(
        MKMapView::default_cluster_annotation_view_reuse_identifier().unwrap(),
        ""
    );
}
