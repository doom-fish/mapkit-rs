use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let completer = MKLocalSearchCompleter::new()?;
    completer.set_query_fragment("coffee")?;
    completer.set_result_types(
        MKLocalSearchCompleterResultType::ADDRESS
            | MKLocalSearchCompleterResultType::POINT_OF_INTEREST,
    )?;

    println!(
        "fragment={} searching={}",
        completer.query_fragment()?,
        completer.is_searching()
    );
    Ok(())
}
