//! Example 21 — Async directions (calculate + ETA)
//!
//! Demonstrates the `async` feature: creating async directions futures.
//!
//! Run with:
//! ```bash
//! cargo run --example 21_async_directions --features async
//! ```
//!
//! **Note:** `MKDirections` dispatches its completion handler on the *main
//! queue*.  This example demonstrates the API surface and exits cleanly
//! without awaiting the futures.

#[cfg(not(feature = "async"))]
fn main() {
    println!("This example requires the 'async' feature.");
    println!("Run: cargo run --example 21_async_directions --features async");
}

#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use mapkit::async_api::AsyncMKDirections;
    use mapkit::directions::{MKDirectionsRequest, MKDirectionsTransportType};
    use mapkit::geometry::MKCoordinate;
    use mapkit::map_item::{MKMapItem, MKPlacemark};

    println!("=== Async Directions (Tier 1) ===\n");

    let source = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.3318, -122.0312)));
    let destination = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.7749, -122.4194)));
    let request = MKDirectionsRequest::new(source, destination)
        .with_transport_type(MKDirectionsTransportType::AUTOMOBILE);

    println!(
        "Request created (source/destination coordinates set)"
    );

    // -- calculate() future --
    let calc_fut = AsyncMKDirections::calculate_from_request(&request)?;
    println!(
        "DirectionsCalculateFuture created (size: {} bytes)",
        std::mem::size_of_val(&calc_fut)
    );
    drop(calc_fut);

    // -- calculate_eta() future --
    let eta_fut = AsyncMKDirections::calculate_eta_from_request(&request)?;
    println!(
        "DirectionsEtaFuture created (size: {} bytes)",
        std::mem::size_of_val(&eta_fut)
    );
    drop(eta_fut);

    println!("\nFutures dropped without awaiting (headless-safe).");
    println!("In a real app with a main run loop, use: response = future.await?");
    println!("\nDone — exiting 0 (headless).");
    Ok(())
}
