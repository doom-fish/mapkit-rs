use mapkit::prelude::*;

#[test]
fn map_item_constants_and_errors_are_exposed() {
    let mut item = MKMapItem::current_location();
    item.identifier = Some("map-item-id".to_owned());
    item.alternate_identifiers = vec!["alternate-id".to_owned()];

    assert_eq!(item.identifier_value().unwrap().raw_value(), "map-item-id");
    assert_eq!(item.alternate_identifier_values()[0].raw_value(), "alternate-id");

    let constants = [
        MKMapItem::launch_options_camera_key().unwrap(),
        MKMapItem::launch_options_directions_mode_cycling().unwrap(),
        MKMapItem::launch_options_directions_mode_default().unwrap(),
        MKMapItem::launch_options_directions_mode_driving().unwrap(),
        MKMapItem::launch_options_directions_mode_key().unwrap(),
        MKMapItem::launch_options_directions_mode_transit().unwrap(),
        MKMapItem::launch_options_directions_mode_walking().unwrap(),
        MKMapItem::launch_options_map_center_key().unwrap(),
        MKMapItem::launch_options_map_span_key().unwrap(),
        MKMapItem::launch_options_map_type_key().unwrap(),
        MKMapItem::launch_options_shows_traffic_key().unwrap(),
        MKMapItem::type_identifier().unwrap(),
    ];
    assert!(constants.iter().all(|value| !value.is_empty()));

    let identifier = MKMapItemIdentifier::new("map-item-id");
    assert_eq!(identifier.raw_value(), "map-item-id");

    let _ = MKMapItemRequest::new as fn(&MKMapItemIdentifier) -> Result<MKMapItemRequest, MapKitError>;
    let _request_size = core::mem::size_of::<Option<&MKMapItemRequest>>();

    let error_info = NSErrorInfo {
        domain: mk_error_domain().to_owned(),
        code: MKErrorCode::DirectionsNotFound.as_raw(),
        message: "missing directions".to_owned(),
    };
    assert!(error_info.is_mapkit_domain());
    assert_eq!(
        error_info.mapkit_error_code(),
        Some(MKErrorCode::DirectionsNotFound)
    );
    assert_eq!(
        MapKitError::Framework(error_info).mapkit_error_code(),
        Some(MKErrorCode::DirectionsNotFound)
    );
}
