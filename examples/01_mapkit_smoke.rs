use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0)?;
    let point = MKMapPoint::from_coordinate(center)?;
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 240.0))?;
    map_view.set_region(region, false)?;

    let mut formatter = MKDistanceFormatter::new()?;
    formatter.set_units(MKDistanceFormatterUnits::Metric);
    let distance = formatter.string_from_distance(1_500.0)?;

    println!("center point: {:.2}, {:.2}", point.x, point.y);
    println!("formatted distance: {distance}");
    println!("annotations={} overlays={}", map_view.annotation_count()?, map_view.overlay_count()?);
    println!("✅ mapkit OK");
    Ok(())
}
