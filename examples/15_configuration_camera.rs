use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0)?;
    let camera = MKMapCamera::looking_at_center_coordinate(center, 750.0, 35.0, 90.0);
    let boundary = MKMapCameraBoundary::from_region(region)?;
    let configuration: MKMapConfiguration = MKHybridMapConfiguration::new()
        .with_elevation_style(MKMapElevationStyle::Realistic)
        .with_shows_traffic(true)
        .into();

    println!(
        "camera_distance={} boundary_width={} config={:?}",
        camera.center_coordinate_distance,
        boundary.map_rect.size.width,
        configuration.kind
    );
    Ok(())
}
