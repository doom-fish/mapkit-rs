use mapkit::prelude::*;

#[test]
fn point_of_interest_filter_logic() {
    let cafe = MKPointOfInterestCategory::cafe();
    let ev_charger = MKPointOfInterestCategory::ev_charger();
    let rv_park = MKPointOfInterestCategory::rv_park();
    let filter = MKPointOfInterestFilter::including(vec![cafe.clone(), ev_charger.clone()]);
    assert!(filter.includes_category(&cafe));
    assert!(filter.includes_category(&ev_charger));
    assert!(!filter.includes_category(&rv_park));

    let request =
        MKLocalPointsOfInterestRequest::with_radius(MKCoordinate::new(37.0, -122.0), 250.0)
            .with_point_of_interest_filter(filter);
    assert_eq!(request.radius, Some(250.0));
    assert!(MKLocalPointsOfInterestRequest::max_radius().is_finite());
    assert!(request.validate().is_ok());
}
