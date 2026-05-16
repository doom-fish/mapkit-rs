use mapkit::prelude::*;

#[test]
fn geocoder_requests_expose_state() {
    let geocoder = MKGeocodingRequest::new("1 Apple Park Way, Cupertino").unwrap();
    let reverse = MKReverseGeocodingRequest::new(MKCoordinate::new(37.3349, -122.0090)).unwrap();
    assert!(geocoder.address_string().unwrap().contains("Apple Park"));
    assert!(reverse.location().unwrap().latitude > 37.0);
}
