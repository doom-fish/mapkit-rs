use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = MKLookAroundSceneRequest::new(MKCoordinate::new(37.3349, -122.0090))?;
    let options = MKLookAroundSnapshotOptions::new(MKScreenSize::new(256.0, 256.0));
    println!("coordinate={} has_map_item={} size={}", request.coordinate()?.latitude, request.has_map_item()?, options.size.width);
    Ok(())
}
