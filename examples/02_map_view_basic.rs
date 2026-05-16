use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_view = MKMapView::new(MKScreenSize::new(256.0, 256.0))?;
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 2_000.0, 2_000.0)?;
    map_view.set_region(region, false)?;
    map_view.set_shows_compass(true)?;
    let point = map_view.convert_coordinate_to_point(center)?;
    println!("region center: {} {}", map_view.region()?.center.latitude, map_view.region()?.center.longitude);
    println!("point: {}, {}", point.x, point.y);
    Ok(())
}
