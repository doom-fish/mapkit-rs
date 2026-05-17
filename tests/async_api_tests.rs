//! Tests for the `async_api` module (requires `--features async`).
//!
//! These tests verify:
//! - Future types are constructed without panicking.
//! - Future types have the expected `Send` bounds.
//! - The convenience constructors return the correct types.
//!
//! Tests that would require awaiting an actual `MapKit` network call (which
//! needs the main run loop in a full macOS app) are marked `#[ignore]` and
//! are documented with what the expected behaviour is.

#[cfg(feature = "async")]
mod async_tests {
    use mapkit::async_api::{
        AsyncMKDirections, AsyncMKGeocodingRequest, AsyncMKLocalSearch, AsyncMKMapSnapshotter,
        AsyncMKReverseGeocodingRequest,
    };
    use mapkit::directions::{MKDirectionsRequest, MKDirectionsTransportType};
    use mapkit::geometry::{MKCoordinate, MKCoordinateRegion, MKCoordinateSpan, MKScreenSize};
    use mapkit::local_search::{MKLocalSearchRequest, MKLocalSearchResultType};
    use mapkit::map_item::{MKMapItem, MKPlacemark};
    use mapkit::point_of_interest::MKLocalPointsOfInterestRequest;
    use mapkit::snapshotter::MKMapSnapshotOptions;

    // ------------------------------------------------------------------
    // LocalSearch
    // ------------------------------------------------------------------

    #[test]
    fn local_search_future_constructs() {
        let request = MKLocalSearchRequest::new("pizza")
            .with_result_types(MKLocalSearchResultType::POINT_OF_INTEREST);
        let fut = AsyncMKLocalSearch::search(&request).expect("should build future");
        drop(fut);
    }

    #[test]
    fn local_search_future_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<mapkit::async_api::LocalSearchStartFuture>();
    }

    #[test]
    fn local_search_poi_future_constructs() {
        let poi_req = MKLocalPointsOfInterestRequest::with_radius(
            MKCoordinate::new(37.33, -122.03),
            500.0,
        );
        let fut = AsyncMKLocalSearch::search_points_of_interest(&poi_req)
            .expect("should build POI future");
        drop(fut);
    }

    // ------------------------------------------------------------------
    // Directions
    // ------------------------------------------------------------------

    fn make_directions_request() -> MKDirectionsRequest {
        let src = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.33, -122.03)));
        let dst = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.77, -122.41)));
        MKDirectionsRequest::new(src, dst)
            .with_transport_type(MKDirectionsTransportType::AUTOMOBILE)
    }

    #[test]
    fn directions_calculate_future_constructs() {
        let req = make_directions_request();
        let fut = AsyncMKDirections::calculate_from_request(&req).expect("should build future");
        drop(fut);
    }

    #[test]
    fn directions_calculate_future_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<mapkit::async_api::DirectionsCalculateFuture>();
    }

    #[test]
    fn directions_eta_future_constructs() {
        let req = make_directions_request();
        let fut =
            AsyncMKDirections::calculate_eta_from_request(&req).expect("should build ETA future");
        drop(fut);
    }

    #[test]
    fn directions_eta_future_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<mapkit::async_api::DirectionsEtaFuture>();
    }

    // ------------------------------------------------------------------
    // Snapshotter
    // ------------------------------------------------------------------

    #[test]
    fn snapshotter_future_constructs() {
        let opts = MKMapSnapshotOptions::new(MKScreenSize::new(128.0, 128.0)).with_region(
            MKCoordinateRegion::new(
                MKCoordinate::new(37.33, -122.03),
                MKCoordinateSpan::new(0.1, 0.1),
            ),
        );
        let fut = AsyncMKMapSnapshotter::snapshot(&opts).expect("should build future");
        drop(fut);
    }

    #[test]
    fn snapshotter_future_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<mapkit::async_api::SnapshotterStartFuture>();
    }

    // ------------------------------------------------------------------
    // GeocodingRequest (macOS 26.0+ — construction tested; resolution
    // requires the main run loop and is therefore ignored in CI)
    // ------------------------------------------------------------------

    #[test]
    fn geocoder_future_constructs() {
        let fut = AsyncMKGeocodingRequest::geocode("1 Infinite Loop, Cupertino, CA")
            .expect("should build geocoder future");
        drop(fut);
    }

    #[test]
    fn geocoder_future_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<mapkit::async_api::GeocoderFuture>();
    }

    #[test]
    fn reverse_geocoder_future_constructs() {
        let fut = AsyncMKReverseGeocodingRequest::reverse_geocode(MKCoordinate::new(37.33, -122.03))
            .expect("should build reverse geocoder future");
        drop(fut);
    }

    #[test]
    fn reverse_geocoder_future_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<mapkit::async_api::ReverseGeocoderFuture>();
    }

    // ------------------------------------------------------------------
    // Happy-path integration tests (ignored in headless CI — require the
    // main run loop and a network connection)
    // ------------------------------------------------------------------

    /// Awaiting a real local search with pollster.
    /// Requires: macOS with main run loop + network.
    #[test]
    #[ignore = "requires main run loop and network access"]
    fn local_search_resolves() {
        let request = MKLocalSearchRequest::new("Cupertino")
            .with_result_types(MKLocalSearchResultType::ADDRESS);
        let fut = AsyncMKLocalSearch::search(&request).expect("build");
        let result = pollster::block_on(fut);
        let resp = result.expect("search should succeed");
        assert!(!resp.map_items.is_empty(), "expected at least one result");
    }

    /// Awaiting a real snapshot with pollster.
    /// `MKMapSnapshotter` uses a background queue, so this MAY work
    /// headlessly.  Marked ignored by default to avoid CI flakiness.
    #[test]
    #[ignore = "may require a running display / MapKit rendering pipeline"]
    fn snapshot_resolves() {
        let opts = MKMapSnapshotOptions::new(MKScreenSize::new(64.0, 64.0)).with_region(
            MKCoordinateRegion::new(
                MKCoordinate::new(37.33, -122.03),
                MKCoordinateSpan::new(0.1, 0.1),
            ),
        );
        let fut = AsyncMKMapSnapshotter::snapshot(&opts).expect("build");
        let result = pollster::block_on(fut);
        let snapshot = result.expect("snapshot should succeed");
        let bytes = snapshot.image_byte_len().expect("image_byte_len");
        assert!(bytes > 0, "expected non-zero image size");
    }
}

// When compiled without the `async` feature, emit a stub so the test
// binary still compiles.
#[cfg(not(feature = "async"))]
#[test]
fn async_feature_not_enabled() {
    // Nothing to test — the module is not compiled without `--features async`.
}
