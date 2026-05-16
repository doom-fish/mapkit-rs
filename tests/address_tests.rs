use mapkit::prelude::*;

#[test]
fn address_models_work() {
    let address = MKAddress::new("1 Apple Park Way, Cupertino, CA", Some("Apple Park"));
    let filter = MKAddressFilter::including(MKAddressFilterOption::COUNTRY | MKAddressFilterOption::POSTAL_CODE);
    assert_eq!(address.short_address.as_deref(), Some("Apple Park"));
    assert!(filter.includes_options(MKAddressFilterOption::COUNTRY));
}
