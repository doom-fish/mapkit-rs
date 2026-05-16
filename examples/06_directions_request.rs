use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.3349, -122.0090))).with_name("Apple Park");
    let destination = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.3317, -122.0307))).with_name("Infinite Loop");
    let request = MKDirectionsRequest::new(source, destination)
        .with_transport_type(MKDirectionsTransportType::AUTOMOBILE)
        .with_highway_preference(MKDirectionsRoutePreference::Avoid);
    let directions = MKDirections::new(&request)?;
    println!("calculating={} transport={}", directions.is_calculating(), request.transport_type.bits());
    Ok(())
}
