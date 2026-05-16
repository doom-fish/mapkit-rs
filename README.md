# mapkit

Safe Rust bindings for Apple's [MapKit](https://developer.apple.com/documentation/mapkit) framework on macOS.

> **Status:** v0.1.0 covers practical `MKLocalSearch`, `MKDirections`, `MKMapItem`, `MKPlacemark`, `MKCoordinateRegion`, `MKMapPoint`, and `MKDistanceFormatter` APIs for non-UI map workflows.

## Quick start

```rust,no_run
use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 5_000.0, 5_000.0)?;
    let request = MKLocalSearchRequest::new("Apple Park")
        .with_region(region)
        .with_result_types(
            MKLocalSearchResultType::ADDRESS | MKLocalSearchResultType::POINT_OF_INTEREST,
        );

    let response = MKLocalSearch::search(&request)?;
    if let Some(item) = response.map_items.first() {
        if let Some(coordinate) = item.coordinate() {
            println!(
                "{} => {}, {}",
                item.name.as_deref().unwrap_or("<unnamed>"),
                coordinate.latitude,
                coordinate.longitude
            );
        }
    }

    Ok(())
}
```

## Highlights

- `MKLocalSearchRequest` + `MKLocalSearch::search` for query-based place lookup
- `MKDirectionsRequest` + `MKDirections::calculate` / `calculate_eta` for route planning
- `MKDistanceFormatter` for localized distance formatting and parsing
- `MKCoordinateRegion::with_distance`, `MKMapPoint::from_coordinate`, and `MKMapPoint::distance_to`
- JSON-safe snapshots for `MKMapItem`, `MKPlacemark`, `MKRoute`, `MKRouteStep`, and ETA responses

## Smoke example

Run the framework smoke test with:

```bash
cargo run --all-features --example 01_mapkit_smoke
```

Expected success footer:

```text
✅ mapkit OK
```

## Notes

- The smoke example performs a live `MKLocalSearch` for “Apple Park”, so it requires network access.
- This crate intentionally focuses on non-UI MapKit APIs and does not wrap `MKMapView`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
