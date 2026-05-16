use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle = MKCircle::new(MKCoordinate::new(37.3349, -122.0090), 250.0)?;
    let polyline = MKPolyline::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3360, -122.0110),
    ])?;
    let polygon = MKPolygon::new(&[
        MKCoordinate::new(37.3340, -122.0100),
        MKCoordinate::new(37.3350, -122.0120),
        MKCoordinate::new(37.3330, -122.0130),
    ])?;
    println!("circle radius={} polyline points={} polygon points={}", circle.radius()?, polyline.point_count()?, polygon.point_count()?);
    Ok(())
}
