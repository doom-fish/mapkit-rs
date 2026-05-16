use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle = MKCircle::new(MKCoordinate::new(37.3349, -122.0090), 250.0)?;
    let renderer = MKCircleRenderer::new(&circle)?;
    let tile_overlay = MKTileOverlay::new(Some("https://tiles.example.com/{z}/{x}/{y}.png"))?;
    let tile_url = tile_overlay.url_for_tile_path(MKTileOverlayPath {
        x: 1,
        y: 2,
        z: 3,
        content_scale_factor: 2.0,
    })?;

    println!(
        "alpha={} road_width={} tile_url={:?}",
        renderer.alpha()?,
        mk_road_width_at_zoom_scale(1.0),
        tile_url
    );
    Ok(())
}
