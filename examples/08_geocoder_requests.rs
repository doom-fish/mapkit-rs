use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geocoder = MKGeocodingRequest::new("1 Apple Park Way, Cupertino")?;
    let reverse = MKReverseGeocodingRequest::new(MKCoordinate::new(37.3349, -122.0090))?;
    println!("forward={} reverse={}", geocoder.address_string()?, reverse.location()?.latitude);
    Ok(())
}
