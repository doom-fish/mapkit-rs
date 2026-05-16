use mapkit::prelude::*;

#[test]
fn local_search_request_builds() {
    let request = MKLocalSearchRequest::new("cafe")
        .with_result_types(MKLocalSearchResultType::POINT_OF_INTEREST);
    let search = MKLocalSearch::new(&request).unwrap();
    assert!(!search.is_searching());
}
