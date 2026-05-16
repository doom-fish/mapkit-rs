use mapkit::prelude::*;

fn main() {
    let item = MKMapItem::from_location(
        MKCoordinate::new(37.3349, -122.0090),
        Some(MKAddress::new("1 Apple Park Way, Cupertino, CA", Some("Apple Park"))),
    )
    .with_name("Apple Park");
    println!("item={} coordinate={:?}", item.name.as_deref().unwrap_or("<unnamed>"), item.coordinate());
}
