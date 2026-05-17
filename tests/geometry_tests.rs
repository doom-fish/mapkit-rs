use mapkit::prelude::*;

fn assert_f64_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON, "left={left}, right={right}");
}

#[test]
fn geometry_gap_helpers_round_trip() {
    let point = MKMapPoint::new(10.0, 20.0);
    let size = MKMapSize::new(30.0, 40.0);
    let rect = MKMapRect::new(point, size);
    let other = MKMapRect::new(MKMapPoint::new(20.0, 30.0), MKMapSize::new(15.0, 10.0));

    assert!(point.equal_to(MKMapPoint::new(10.0, 20.0)));
    assert_eq!(mk_string_from_map_point(point), "{10.0, 20.0}");

    assert!(size.equal_to(MKMapSize::new(30.0, 40.0)));
    assert_eq!(mk_string_from_map_size(size), "{30.0, 40.0}");
    assert!(MKMapSize::world().unwrap().width.is_sign_positive());

    assert_f64_eq(rect.min_x(), 10.0);
    assert_f64_eq(rect.min_y(), 20.0);
    assert_f64_eq(rect.mid_x(), 25.0);
    assert_f64_eq(rect.mid_y(), 40.0);
    assert_f64_eq(rect.max_x(), 40.0);
    assert_f64_eq(rect.max_y(), 60.0);
    assert_f64_eq(rect.width(), 30.0);
    assert_f64_eq(rect.height(), 40.0);
    assert!(rect.equal_to(MKMapRect::new(point, size)));
    assert!(rect.contains_point(MKMapPoint::new(15.0, 25.0)).unwrap());
    assert!(rect.contains_rect(MKMapRect::new(
        MKMapPoint::new(12.0, 22.0),
        MKMapSize::new(5.0, 5.0),
    ))
    .unwrap());
    assert!(rect.intersects_rect(other).unwrap());
    assert!(!rect.is_empty());
    assert!(!rect.is_null());

    let union = rect.union(other).unwrap();
    assert_f64_eq(union.min_x(), 10.0);
    assert_f64_eq(union.max_x(), 40.0);

    let intersection = rect.intersection(other).unwrap();
    assert_f64_eq(intersection.min_x(), 20.0);
    assert_f64_eq(intersection.max_x(), 35.0);

    let inset = rect.inset(5.0, 5.0).unwrap();
    assert_f64_eq(inset.width(), 20.0);
    let offset = rect.offset(3.0, -2.0).unwrap();
    assert_f64_eq(offset.min_x(), 13.0);
    assert_f64_eq(offset.min_y(), 18.0);

    let division = rect.divide(10.0, MKMapRectEdge::MinX).unwrap();
    assert_f64_eq(division.slice.width(), 10.0);
    assert_f64_eq(division.remainder.width(), 20.0);

    assert_eq!(mk_string_from_map_rect(rect), "{{10.0, 20.0}, {30.0, 40.0}}");

    let world = MKMapRect::world().unwrap();
    let spanning = MKMapRect::new(
        MKMapPoint::new(world.max_x() - 5.0, world.min_y()),
        MKMapSize::new(10.0, 10.0),
    );
    assert!(spanning.spans_180th_meridian().unwrap());
    assert!(spanning.remainder().unwrap().width() > 0.0);
    assert!(MKMapRect::null().is_null());

    let region = MKCoordinateRegion::from_map_rect(rect).unwrap();
    assert!(region.span.latitude_delta.is_finite());
    assert!(region.span.longitude_delta.is_finite());

    assert!(mk_map_points_per_meter_at_latitude(37.3349).unwrap() > 0.0);
    assert!(mk_meters_per_map_point_at_latitude(37.3349).unwrap() > 0.0);
}
