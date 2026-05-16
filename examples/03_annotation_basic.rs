use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let annotation = MKPointAnnotation::new(MKCoordinate::new(37.3349, -122.0090))?;
    annotation.set_title(Some("Apple Park"))?;
    annotation.set_subtitle(Some("Cupertino"))?;
    println!("annotation: {:?} / {:?}", annotation.title()?, annotation.subtitle()?);
    Ok(())
}
