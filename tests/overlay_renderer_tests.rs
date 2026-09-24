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
    let tile_overlay =
        MKTileOverlay::new(Some("https://tiles.example.com/{z}/{x}/{y}.png")).unwrap();

    let overlay_renderer = MKOverlayRenderer::new(&circle).unwrap();
    let path_renderer = MKOverlayPathRenderer::new(&polyline).unwrap();
    let circle_renderer = MKCircleRenderer::new(&circle).unwrap();
    let polyline_renderer = MKPolylineRenderer::new(&polyline).unwrap();
    let gradient_renderer = MKGradientPolylineRenderer::new(&polyline).unwrap();
    let second_polyline = MKPolyline::new(&[
        MKCoordinate::new(37.3360, -122.0110),
        MKCoordinate::new(37.3370, -122.0120),
    ])
    .unwrap();
    let multi_polyline = MKMultiPolyline::new(&[&polyline, &second_polyline]).unwrap();
    let polygon = MKPolygon::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3349, -122.0000),
        MKCoordinate::new(37.3400, -122.0050),
    ])
    .unwrap();
    let second_polygon = MKPolygon::new(&[
        MKCoordinate::new(37.3410, -122.0080),
        MKCoordinate::new(37.3410, -122.0020),
        MKCoordinate::new(37.3450, -122.0050),
    ])
    .unwrap();
    let multi_polygon = MKMultiPolygon::new(&[&polygon, &second_polygon]).unwrap();
    let polygon_renderer = MKPolygonRenderer::new(&polygon).unwrap();
    let multi_polyline_renderer = MKMultiPolylineRenderer::new(&multi_polyline).unwrap();
    let multi_polygon_renderer = MKMultiPolygonRenderer::new(&multi_polygon).unwrap();
    let tile_renderer = MKTileOverlayRenderer::new(&tile_overlay).unwrap();

    path_renderer.set_line_width(3.0).unwrap();
    circle_renderer.set_stroke_start(0.1).unwrap();
    polyline_renderer.set_stroke_end(0.9).unwrap();
    gradient_renderer.set_stroke_end(0.8).unwrap();
    polygon_renderer.set_stroke_start(0.2).unwrap();
    multi_polyline_renderer.set_line_width(4.0).unwrap();
    multi_polygon_renderer.set_line_dash_phase(1.0).unwrap();
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
    assert!(geodesic.point_count().unwrap() > 2);
    assert_eq!(multi_polyline.polyline_count().unwrap(), 2);
    assert_eq!(multi_polygon.polygon_count().unwrap(), 2);
    assert!(tile_url.unwrap().contains("/3/1/2"));
}

fn polyline() -> MKPolyline {
    MKPolyline::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3355, -122.0100),
        MKCoordinate::new(37.3370, -122.0120),
    ])
    .unwrap()
}

fn polygon() -> MKPolygon {
    MKPolygon::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3349, -122.0000),
        MKCoordinate::new(37.3400, -122.0050),
    ])
    .unwrap()
}

fn assert_color_eq(actual: Option<MKColor>, expected: MKColor) {
    let actual = actual.expect("renderer should report the color it was given");
    for (got, want) in [
        (actual.red, expected.red),
        (actual.green, expected.green),
        (actual.blue, expected.blue),
        (actual.alpha, expected.alpha),
    ] {
        assert!((got - want).abs() < 1e-6, "{actual:?} != {expected:?}");
    }
}

macro_rules! check_path_style {
    ($renderer:expr) => {{
        let renderer = $renderer;
        let stroke = MKColor::new(0.25, 0.5, 0.75, 0.8);

        renderer.set_stroke_color(Some(stroke)).unwrap();
        assert_color_eq(renderer.stroke_color().unwrap(), stroke);
        renderer.set_stroke_color(None).unwrap();
        assert_eq!(renderer.stroke_color().unwrap(), None);

        renderer.set_line_width(6.5).unwrap();
        assert!((renderer.line_width().unwrap() - 6.5).abs() < f64::EPSILON);
        renderer.set_line_cap(MKLineCap::Square).unwrap();
        assert_eq!(renderer.line_cap().unwrap(), MKLineCap::Square);
        renderer.set_line_join(MKLineJoin::Bevel).unwrap();
        assert_eq!(renderer.line_join().unwrap(), MKLineJoin::Bevel);
        renderer.set_miter_limit(4.0).unwrap();
        assert!((renderer.miter_limit().unwrap() - 4.0).abs() < f64::EPSILON);
        renderer.set_line_dash_phase(2.0).unwrap();
        assert!((renderer.line_dash_phase().unwrap() - 2.0).abs() < f64::EPSILON);
        renderer
            .set_line_dash_pattern(Some(vec![4.0, 2.0]))
            .unwrap();
        assert_eq!(renderer.line_dash_pattern().unwrap(), Some(vec![4.0, 2.0]));
        renderer.set_line_dash_pattern(None).unwrap();
        assert_eq!(renderer.line_dash_pattern().unwrap(), None);
        renderer.set_alpha(0.5).unwrap();
        assert!((renderer.alpha().unwrap() - 0.5).abs() < 1e-6);
        renderer.set_should_rasterize(true).unwrap();
        assert!(renderer.should_rasterize().unwrap());

        for width in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                renderer.set_line_width(width),
                Err(MapKitError::InvalidArgument(_))
            ));
        }
        assert!((renderer.line_width().unwrap() - 6.5).abs() < f64::EPSILON);
        for color in [
            MKColor::new(1.5, 0.0, 0.0, 1.0),
            MKColor::new(0.0, -0.1, 0.0, 1.0),
            MKColor::new(0.0, 0.0, f64::NAN, 1.0),
            MKColor::new(0.0, 0.0, 0.0, f64::INFINITY),
        ] {
            assert!(matches!(
                renderer.set_stroke_color(Some(color)),
                Err(MapKitError::InvalidArgument(_))
            ));
        }
        assert!(matches!(
            renderer.set_alpha(1.5),
            Err(MapKitError::InvalidArgument(_))
        ));
        assert!(matches!(
            renderer.set_alpha(f64::NAN),
            Err(MapKitError::InvalidArgument(_))
        ));
        assert!(matches!(
            renderer.set_miter_limit(-2.0),
            Err(MapKitError::InvalidArgument(_))
        ));
        assert!(matches!(
            renderer.set_line_dash_phase(f64::NEG_INFINITY),
            Err(MapKitError::InvalidArgument(_))
        ));
        for pattern in [vec![4.0, -1.0], vec![0.0, 0.0], vec![f64::NAN]] {
            assert!(matches!(
                renderer.set_line_dash_pattern(Some(pattern)),
                Err(MapKitError::InvalidArgument(_))
            ));
        }
        renderer
    }};
}

macro_rules! check_fill {
    ($renderer:expr) => {{
        let renderer = $renderer;
        let fill = MKColor::new(0.1, 0.2, 0.3, 0.4);
        renderer.set_fill_color(Some(fill)).unwrap();
        assert_color_eq(renderer.fill_color().unwrap(), fill);
        renderer.set_fill_color(None).unwrap();
        assert_eq!(renderer.fill_color().unwrap(), None);
        assert!(matches!(
            renderer.set_fill_color(Some(MKColor::new(0.0, 0.0, 2.0, 1.0))),
            Err(MapKitError::InvalidArgument(_))
        ));
    }};
}

#[test]
fn path_renderers_set_and_read_their_style() {
    let circle = MKCircle::new(MKCoordinate::new(37.3349, -122.0090), 250.0).unwrap();
    let polyline = polyline();
    let polygon = polygon();
    let multi_polyline = MKMultiPolyline::new(&[&polyline]).unwrap();
    let multi_polygon = MKMultiPolygon::new(&[&polygon]).unwrap();

    check_fill!(check_path_style!(
        MKOverlayPathRenderer::new(&polygon).unwrap()
    ));
    check_fill!(check_path_style!(MKCircleRenderer::new(&circle).unwrap()));
    check_fill!(check_path_style!(MKPolygonRenderer::new(&polygon).unwrap()));
    check_fill!(check_path_style!(MKMultiPolygonRenderer::new(
        &multi_polygon
    )
    .unwrap()));
    check_path_style!(MKPolylineRenderer::new(&polyline).unwrap());
    check_path_style!(MKGradientPolylineRenderer::new(&polyline).unwrap());
    check_path_style!(MKMultiPolylineRenderer::new(&multi_polyline).unwrap());
}

#[test]
fn stroke_range_and_alpha_are_validated() {
    let polyline = polyline();
    let renderer = MKPolylineRenderer::new(&polyline).unwrap();
    renderer.set_stroke_start(0.25).unwrap();
    renderer.set_stroke_end(0.75).unwrap();
    assert!((renderer.stroke_start().unwrap() - 0.25).abs() < 1e-6);
    assert!((renderer.stroke_end().unwrap() - 0.75).abs() < 1e-6);
    for value in [-0.1, 1.1, f64::NAN] {
        assert!(matches!(
            renderer.set_stroke_start(value),
            Err(MapKitError::InvalidArgument(_))
        ));
        assert!(matches!(
            renderer.set_stroke_end(value),
            Err(MapKitError::InvalidArgument(_))
        ));
    }
    assert!((renderer.stroke_start().unwrap() - 0.25).abs() < 1e-6);

    let circle = MKCircle::new(MKCoordinate::new(37.3349, -122.0090), 250.0).unwrap();
    let base = MKOverlayRenderer::new(&circle).unwrap();
    base.set_alpha(0.25).unwrap();
    assert!((base.alpha().unwrap() - 0.25).abs() < 1e-6);
    assert!(matches!(
        base.set_alpha(-0.5),
        Err(MapKitError::InvalidArgument(_))
    ));

    let tile_overlay =
        MKTileOverlay::new(Some("https://tiles.example.com/{z}/{x}/{y}.png")).unwrap();
    let tile = MKTileOverlayRenderer::new(&tile_overlay).unwrap();
    tile.set_alpha(0.75).unwrap();
    assert!((tile.alpha().unwrap() - 0.75).abs() < 1e-6);
    assert!(matches!(
        tile.set_alpha(f64::INFINITY),
        Err(MapKitError::InvalidArgument(_))
    ));
}

#[test]
fn gradient_polyline_colors_round_trip_and_are_validated() {
    let polyline = polyline();
    let renderer = MKGradientPolylineRenderer::new(&polyline).unwrap();
    let colors = [
        MKColor::new(1.0, 0.0, 0.0, 1.0),
        MKColor::new(0.0, 0.0, 1.0, 0.5),
    ];
    renderer.set_colors(&colors, &[0.0, 1.0]).unwrap();

    let read = renderer.colors().unwrap();
    assert_eq!(read.len(), 2);
    assert_color_eq(Some(read[0]), colors[0]);
    assert_color_eq(Some(read[1]), colors[1]);
    let locations = renderer.locations().unwrap();
    assert_eq!(locations.len(), 2);
    assert!(locations[0].abs() < 1e-6 && (locations[1] - 1.0).abs() < 1e-6);

    assert!(matches!(
        renderer.set_colors(&colors, &[0.0]),
        Err(MapKitError::InvalidArgument(_))
    ));
    assert!(matches!(
        renderer.set_colors(&colors, &[0.0, 1.5]),
        Err(MapKitError::InvalidArgument(_))
    ));
    assert!(matches!(
        renderer.set_colors(&[MKColor::new(0.0, 0.0, 0.0, 2.0)], &[0.5]),
        Err(MapKitError::InvalidArgument(_))
    ));
    assert_eq!(renderer.colors().unwrap().len(), 2);
}
