use mapkit::prelude::*;

#[test]
fn overlay_construction() {
    let circle = MKCircle::new(MKCoordinate::new(37.0, -122.0), 200.0).unwrap();
    let polyline = MKPolyline::new(&[
        MKCoordinate::new(37.0, -122.0),
        MKCoordinate::new(37.1, -122.1),
    ])
    .unwrap();
    let second_polyline = MKPolyline::new(&[
        MKCoordinate::new(37.2, -122.2),
        MKCoordinate::new(37.3, -122.3),
    ])
    .unwrap();
    let polygon = MKPolygon::new(&[
        MKCoordinate::new(37.0, -122.0),
        MKCoordinate::new(37.0, -122.2),
        MKCoordinate::new(37.2, -122.1),
    ])
    .unwrap();
    let second_polygon = MKPolygon::new(&[
        MKCoordinate::new(37.3, -122.3),
        MKCoordinate::new(37.3, -122.4),
        MKCoordinate::new(37.4, -122.35),
    ])
    .unwrap();
    let multi_polyline = MKMultiPolyline::new(&[&polyline, &second_polyline]).unwrap();
    let multi_polygon = MKMultiPolygon::new(&[&polygon, &second_polygon]).unwrap();

    assert_eq!(polyline.point_count().unwrap(), 2);
    assert_eq!(polygon.point_count().unwrap(), 3);
    assert!(circle.radius().unwrap() > 0.0);
    assert_eq!(multi_polyline.polyline_count().unwrap(), 2);
    assert_eq!(multi_polygon.polygon_count().unwrap(), 2);
    assert_eq!(multi_polyline.polylines().unwrap()[0].len(), 2);
    assert_eq!(multi_polygon.polygons().unwrap()[0].len(), 3);
}
