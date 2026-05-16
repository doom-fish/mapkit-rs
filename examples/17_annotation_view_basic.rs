use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let annotation = MKPointAnnotation::new(MKCoordinate::new(37.3349, -122.0090))?;
    annotation.set_title(Some("Apple Park"))?;

    let view = MKAnnotationView::new(Some(&annotation), Some("reuse-id"))?;
    let marker = MKMarkerAnnotationView::new(Some(&annotation), Some("marker-id"))?;
    marker.set_glyph_text(Some("A"))?;

    println!(
        "reuse={:?} title={:?} glyph={:?}",
        view.reuse_identifier()?,
        view.annotation_title()?,
        marker.glyph_text()?
    );
    Ok(())
}
