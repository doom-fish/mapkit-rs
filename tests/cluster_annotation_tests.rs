use mapkit::prelude::*;

#[test]
fn cluster_annotation_counts_members() {
    let first = MKPointAnnotation::new(MKCoordinate::new(37.0, -122.0)).unwrap();
    let second = MKPointAnnotation::new(MKCoordinate::new(37.1, -122.1)).unwrap();
    let cluster = MKClusterAnnotation::new(&[first, second]).unwrap();
    assert_eq!(cluster.member_count().unwrap(), 2);
}
