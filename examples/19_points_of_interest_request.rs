use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = MKPointOfInterestFilter::including(vec![
        MKPointOfInterestCategory::cafe(),
        MKPointOfInterestCategory::ev_charger(),
        MKPointOfInterestCategory::rv_park(),
    ]);
    let request = MKLocalPointsOfInterestRequest::with_radius(
        MKCoordinate::new(37.3349, -122.0090),
        500.0,
    )
    .with_point_of_interest_filter(filter);

    request.validate()?;
    println!("max_radius={} radius={:?}", MKLocalPointsOfInterestRequest::max_radius(), request.radius);
    Ok(())
}
