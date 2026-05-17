use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle = MKCircle::new(MKCoordinate::new(37.3349, -122.0090), 250.0)?;
    let renderer = MKCircleRenderer::new(&circle)?;
    let polyline_a = MKPolyline::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3355, -122.0100),
    ])?;
    let polyline_b = MKPolyline::new(&[
        MKCoordinate::new(37.3360, -122.0110),
        MKCoordinate::new(37.3370, -122.0120),
    ])?;
    let multi_polyline = MKMultiPolyline::new(&[&polyline_a, &polyline_b])?;
    let multi_polyline_renderer = MKMultiPolylineRenderer::new(&multi_polyline)?;
    let tile_overlay = MKTileOverlay::new(Some("https://tiles.example.com/{z}/{x}/{y}.png"))?;
    let tile_url = tile_overlay.url_for_tile_path(MKTileOverlayPath {
        x: 1,
        y: 2,
        z: 3,
        content_scale_factor: 2.0,
    })?;

    println!(
        "alpha={} road_width={} multi_count={} tile_url={:?}",
        renderer.alpha()?,
        mk_road_width_at_zoom_scale(1.0),
        multi_polyline.polyline_count()?,
        tile_url
    );
    println!("multi_line_width={}", multi_polyline_renderer.line_width()?);
    Ok(())
}
