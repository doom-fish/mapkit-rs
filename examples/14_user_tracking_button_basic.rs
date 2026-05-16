use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_view = MKMapView::new(MKScreenSize::new(256.0, 256.0))?;
    let button = MKUserTrackingButton::new(&map_view);
    button.set_visible(true)?;
    button.set_tracking_mode(MKUserTrackingMode::Follow, false)?;
    println!("visible={} mode={:?}", button.is_visible()?, button.tracking_mode()?);
    Ok(())
}
