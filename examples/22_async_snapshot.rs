//! Example 22 — Async map snapshotter
//!
//! Demonstrates the `async` feature: creating an async snapshot future.
//!
//! Run with:
//! ```bash
//! cargo run --example 22_async_snapshot --features async
//! ```
//!
//! Unlike `MKLocalSearch` and `MKDirections`, `MKMapSnapshotter` dispatches
//! its completion handler on a **background queue**, so this future CAN
//! resolve in a headless program.  However, capturing the actual image
//! requires the `MapKit` rendering pipeline, which may fail in headless CI.
//! This example demonstrates the API surface and exits cleanly.

#[cfg(not(feature = "async"))]
fn main() {
    println!("This example requires the 'async' feature.");
    println!("Run: cargo run --example 22_async_snapshot --features async");
}

#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use mapkit::async_api::AsyncMKMapSnapshotter;
    use mapkit::geometry::{MKCoordinate, MKCoordinateRegion, MKCoordinateSpan, MKScreenSize};
    use mapkit::snapshotter::MKMapSnapshotOptions;

    println!("=== Async Snapshot (Tier 1) ===\n");

    let size = MKScreenSize::new(256.0, 256.0);
    let options = MKMapSnapshotOptions::new(size)
        .with_region(MKCoordinateRegion::new(
            MKCoordinate::new(37.3318, -122.0312),
            MKCoordinateSpan::new(0.05, 0.05),
        ))
        .with_shows_buildings(true);
    println!("Options: size = {}×{}", size.width, size.height);

    let fut = AsyncMKMapSnapshotter::snapshot(&options)?;
    println!(
        "SnapshotterStartFuture created (size: {} bytes)",
        std::mem::size_of_val(&fut)
    );

    // In a real program you would: let snapshot = fut.await?;
    // For headless safety, we drop without awaiting here.
    drop(fut);

    println!("\nFuture dropped without awaiting (headless-safe).");
    println!("In a supported environment: let snapshot = future.await?");
    println!("\nDone — exiting 0 (headless).");
    Ok(())
}
