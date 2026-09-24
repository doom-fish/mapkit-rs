pub fn live_tests_enabled() -> bool {
    let enabled = std::env::var("MAPKIT_LIVE_TESTS").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skip: set MAPKIT_LIVE_TESTS=1 to run tests that reach Apple's servers");
    }
    enabled
}
