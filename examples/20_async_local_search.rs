//! Example 20 — Async local search
//!
//! Demonstrates the `async` feature: creating an async local-search future.
//!
//! Run with:
//! ```bash
//! cargo run --example 20_async_local_search --features async
//! ```
//!
//! **Note:** `MKLocalSearch` dispatches its completion handler on the *main
//! queue*.  In headless binaries (no `NSApplicationMain` / no run loop) the
//! future will not resolve.  This example demonstrates the API surface and
//! exits cleanly without awaiting the future.

#[cfg(not(feature = "async"))]
fn main() {
    println!("This example requires the 'async' feature.");
    println!("Run: cargo run --example 20_async_local_search --features async");
}

#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use mapkit::async_api::AsyncMKLocalSearch;
    use mapkit::geometry::MKCoordinate;
    use mapkit::local_search::{MKLocalSearchRequest, MKLocalSearchResultType};
    use mapkit::point_of_interest::MKLocalPointsOfInterestRequest;

    println!("=== Async Local Search (Tier 1) ===\n");

    // Build a request
    let request = MKLocalSearchRequest::new("coffee shop")
        .with_result_types(MKLocalSearchResultType::POINT_OF_INTEREST);
    println!("Request built: query = {:?}", request.natural_language_query);

    // Create the async future (does NOT start the search yet — that happens
    // when the future is polled / awaited)
    let fut = AsyncMKLocalSearch::search(&request)?;
    println!("LocalSearchStartFuture created (type check ok)");
    println!("  Future size: {} bytes", std::mem::size_of_val(&fut));

    // Drop without awaiting — clean shutdown in headless mode
    drop(fut);
    println!("\nFuture dropped without awaiting (headless-safe).");
    println!("In a real app with a main run loop, use: response = future.await?");

    // --- POI request variant ---
    let poi_req = MKLocalPointsOfInterestRequest::with_radius(
        MKCoordinate::new(37.3318, -122.0312),
        1000.0,
    );
    let poi_fut = AsyncMKLocalSearch::search_points_of_interest(&poi_req)?;
    println!("\nPOI LocalSearchStartFuture created (type check ok)");
    drop(poi_fut);

    println!("\nDone — exiting 0 (headless).");
    Ok(())
}
