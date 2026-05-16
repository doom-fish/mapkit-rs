use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 5_000.0, 5_000.0)?;
    let request = MKLocalSearchRequest::new("Apple Park")
        .with_region(region)
        .with_result_types(
            MKLocalSearchResultType::ADDRESS | MKLocalSearchResultType::POINT_OF_INTEREST,
        );

    let response = MKLocalSearch::search(&request)?;
    let item = response
        .map_items
        .first()
        .ok_or("MKLocalSearch returned no results")?;
    let coordinate = item
        .coordinate()
        .ok_or("first MKMapItem did not include a placemark coordinate")?;

    println!(
        "first result: {} ({}, {})",
        item.name.as_deref().unwrap_or("<unnamed>"),
        coordinate.latitude,
        coordinate.longitude
    );
    println!("✅ mapkit OK");
    Ok(())
}
