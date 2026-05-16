use mapkit::prelude::*;

#[test]
fn overlay_renderer_helpers_work() {
    let path = MKTileOverlayPath {
        x: 1,
        y: 2,
        z: 3,
        content_scale_factor: 2.0,
    };

    assert_eq!(path.x, 1);
    assert!(mk_road_width_at_zoom_scale(1.0).is_finite());
}

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn overlay_renderers_build() {
    let circle = MKCircle::new(MKCoordinate::new(37.3349, -122.0090), 250.0).unwrap();
    let polyline = MKPolyline::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3355, -122.0100),
    ])
    .unwrap();
    let geodesic = MKGeodesicPolyline::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(40.6892, -74.0445),
    ])
    .unwrap();
    let tile_overlay = MKTileOverlay::new(Some("https://tiles.example.com/{z}/{x}/{y}.png")).unwrap();

    let overlay_renderer = MKOverlayRenderer::new(&circle).unwrap();
    let path_renderer = MKOverlayPathRenderer::new(&polyline).unwrap();
    let circle_renderer = MKCircleRenderer::new(&circle).unwrap();
    let polyline_renderer = MKPolylineRenderer::new(&polyline).unwrap();
    let gradient_renderer = MKGradientPolylineRenderer::new(&polyline).unwrap();
    let polygon = MKPolygon::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3349, -122.0000),
        MKCoordinate::new(37.3400, -122.0050),
    ])
    .unwrap();
    let polygon_renderer = MKPolygonRenderer::new(&polygon).unwrap();
    let tile_renderer = MKTileOverlayRenderer::new(&tile_overlay).unwrap();

    path_renderer.set_line_width(3.0).unwrap();
    circle_renderer.set_stroke_start(0.1).unwrap();
    polyline_renderer.set_stroke_end(0.9).unwrap();
    gradient_renderer.set_stroke_end(0.8).unwrap();
    polygon_renderer.set_stroke_start(0.2).unwrap();
    tile_renderer.reload_data().unwrap();

    let tile_url = tile_overlay
        .url_for_tile_path(MKTileOverlayPath {
            x: 1,
            y: 2,
            z: 3,
            content_scale_factor: 2.0,
        })
        .unwrap();

    assert!(overlay_renderer.content_scale_factor().unwrap() >= 1.0);
    assert_eq!(geodesic.point_count().unwrap(), 2);
    assert!(tile_url.unwrap().contains("/3/1/2"));
}
