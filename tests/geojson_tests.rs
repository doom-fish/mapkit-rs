use mapkit::prelude::*;

const fn assert_geo_object<T: MKGeoJSONObject>(_value: &T) {}

#[test]
fn geojson_decoder_builds_objects() {
    let geojson = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "id": "coffee",
                "properties": {"name": "Coffee"},
                "geometry": {
                    "type": "Point",
                    "coordinates": [-122.0090, 37.3349]
                }
            },
            {
                "type": "Feature",
                "properties": {},
                "geometry": {
                    "type": "LineString",
                    "coordinates": [
                        [-122.0, 37.0],
                        [-122.1, 37.1]
                    ]
                }
            }
        ]
    }"#;

    let decoder = MKGeoJSONDecoder::new();
    let objects = decoder.decode_str(geojson).unwrap();
    assert_eq!(objects.len(), 2);

    match &objects[0] {
        MKGeoJSONObjectValue::Feature(feature) => {
            assert_geo_object(feature);
            assert_eq!(feature.identifier.as_deref(), Some("coffee"));
            assert!(feature
                .properties
                .as_deref()
                .unwrap()
                .contains("\"Coffee\""));
            assert_eq!(feature.geometry.len(), 1);
            match &feature.geometry[0] {
                MKGeoJSONObjectValue::PointAnnotation(point) => {
                    assert_geo_object(point);
                    assert!((point.coordinate.latitude - 37.3349).abs() < f64::EPSILON);
                }
                other => panic!("unexpected nested object: {other:?}"),
            }
        }
        other => panic!("unexpected top-level object: {other:?}"),
    }

    match &objects[1] {
        MKGeoJSONObjectValue::Feature(feature) => match &feature.geometry[0] {
            MKGeoJSONObjectValue::Polyline(polyline) => {
                assert_geo_object(polyline);
                assert_eq!(polyline.coordinates.len(), 2);
            }
            other => panic!("unexpected geometry payload: {other:?}"),
        },
        other => panic!("unexpected second object: {other:?}"),
    }
}
