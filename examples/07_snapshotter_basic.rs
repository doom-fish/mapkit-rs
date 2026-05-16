use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let options = MKMapSnapshotOptions::new(MKScreenSize::new(320.0, 200.0))
        .with_region(MKCoordinateRegion::with_distance(center, 800.0, 800.0)?);
    let snapshotter = MKMapSnapshotter::new(&options)?;
    let snapshot = snapshotter.start()?;
    println!("snapshot bytes={} point={:?}", snapshot.image_byte_len()?, snapshot.point_for_coordinate(center)?);
    Ok(())
}
