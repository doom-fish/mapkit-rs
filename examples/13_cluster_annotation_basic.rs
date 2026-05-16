use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first = MKPointAnnotation::new(MKCoordinate::new(37.3349, -122.0090))?;
    let second = MKPointAnnotation::new(MKCoordinate::new(37.3355, -122.0100))?;
    let cluster = MKClusterAnnotation::new(&[first, second])?;
    println!("cluster members={} coordinate={:?}", cluster.member_count()?, cluster.coordinate()?);
    Ok(())
}
