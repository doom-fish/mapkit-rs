use mapkit::prelude::*;

#[test]
fn point_of_interest_filter_logic() {
    let cafe = MKPointOfInterestCategory::cafe();
    let filter = MKPointOfInterestFilter::including(vec![cafe.clone()]);
    assert!(filter.includes_category(&cafe));
    let request = MKLocalPointsOfInterestRequest::with_radius(MKCoordinate::new(37.0, -122.0), 250.0)
        .with_point_of_interest_filter(filter);
    assert_eq!(request.radius, Some(250.0));
}
