use mapkit::prelude::*;

#[test]
fn local_search_completer_result_types_combine() {
    let result_types = MKLocalSearchCompleterResultType::ADDRESS
        | MKLocalSearchCompleterResultType::POINT_OF_INTEREST;

    assert!(result_types.contains(MKLocalSearchCompleterResultType::ADDRESS));
    assert!(result_types.contains(MKLocalSearchCompleterResultType::POINT_OF_INTEREST));
    assert_eq!(
        MKLocalSearchCompleterResultType::ALL.bits(),
        MKLocalSearchCompleterResultType::ADDRESS.bits()
            | MKLocalSearchCompleterResultType::POINT_OF_INTEREST.bits()
            | MKLocalSearchCompleterResultType::QUERY.bits()
            | MKLocalSearchCompleterResultType::PHYSICAL_FEATURE.bits()
    );
}

#[test]
#[ignore = "requires a dedicated main-thread process"]
fn local_search_completer_builds() {
    let completer = MKLocalSearchCompleter::new().unwrap();
    completer.set_query_fragment("coffee").unwrap();
    completer
        .set_result_types(
            MKLocalSearchCompleterResultType::ADDRESS
                | MKLocalSearchCompleterResultType::POINT_OF_INTEREST,
        )
        .unwrap();

    assert_eq!(completer.query_fragment().unwrap(), "coffee");
    assert!(
        completer
            .result_types()
            .unwrap()
            .contains(MKLocalSearchCompleterResultType::POINT_OF_INTEREST)
    );
}
