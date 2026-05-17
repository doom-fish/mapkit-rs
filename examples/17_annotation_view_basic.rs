use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coordinate = MKCoordinate::new(37.3349, -122.0090);
    let annotation = MKPointAnnotation::new(coordinate)?;
    annotation.set_title(Some("Apple Park"))?;

    let view = MKAnnotationView::new(Some(&annotation), Some("reuse-id"))?;
    let marker = MKMarkerAnnotationView::new(Some(&annotation), Some("marker-id"))?;
    marker.set_glyph_text(Some("A"))?;

    let map_item = MKMapItem::new(MKPlacemark::new(coordinate)).with_name("Apple Park");
    let map_item_annotation = MKMapItemAnnotation::new(&map_item)?;
    let pin = MKPinAnnotationView::new(Some(&map_item_annotation), Some("pin-id"))?;
    pin.set_pin_color(MKPinAnnotationColor::Purple)?;

    println!(
        "reuse={:?} title={:?} glyph={:?} pin_color={:?}",
        view.reuse_identifier()?,
        view.annotation_title()?,
        marker.glyph_text()?,
        pin.pin_color()?
    );
    Ok(())
}
