use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = MKLocalSearchRequest::new("coffee")
        .with_result_types(MKLocalSearchResultType::POINT_OF_INTEREST)
        .with_region_priority(MKLocalSearchRegionPriority::Required);
    let search = MKLocalSearch::new(&request)?;
    println!("searching={} query={}", search.is_searching(), request.natural_language_query);
    Ok(())
}
