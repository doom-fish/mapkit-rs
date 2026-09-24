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
fn local_search_completer_refuses_other_threads() {
    let error = MKLocalSearchCompleter::new()
        .map(drop)
        .expect_err("the test harness runs tests off the main thread");
    assert!(
        matches!(error, MapKitError::MainThreadRequired(_)),
        "{error:?}"
    );
    assert!(error.to_string().contains("main thread"), "{error}");
}
