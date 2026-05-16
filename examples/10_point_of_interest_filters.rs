use mapkit::prelude::*;

fn main() {
    let filter = MKPointOfInterestFilter::including(vec![
        MKPointOfInterestCategory::cafe(),
        MKPointOfInterestCategory::restaurant(),
    ]);
    let request = MKLocalPointsOfInterestRequest::with_radius(
        MKCoordinate::new(37.3349, -122.0090),
        500.0,
    )
    .with_point_of_interest_filter(filter.clone());
    println!("includes cafe={} radius={:?}", filter.includes_category(&MKPointOfInterestCategory::cafe()), request.radius);
}
