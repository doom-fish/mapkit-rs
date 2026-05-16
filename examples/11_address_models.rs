use mapkit::prelude::*;

fn main() {
    let address = MKAddress::new("1 Apple Park Way, Cupertino, CA", Some("Apple Park"));
    let filter = MKAddressFilter::including(MKAddressFilterOption::COUNTRY | MKAddressFilterOption::POSTAL_CODE);
    println!("address={} includes_country={}", address.full_address, filter.includes_options(MKAddressFilterOption::COUNTRY));
}
