import Foundation
import MapKit

enum MKRAddressFilterModePayload: String, Codable {
    case including
    case excluding
    case includingAll
    case excludingAll
}

struct MKRAddressFilterPayload: Codable {
    var mode: MKRAddressFilterModePayload
    var options: UInt64
}

struct MKRAddressPayload: Codable {
    var fullAddress: String
    var shortAddress: String?
}

struct MKRAddressRepresentationsPayload: Codable {
    var cityName: String?
    var cityWithContext: String?
    var cityWithContextShort: String?
    var cityWithContextFull: String?
    var regionName: String?
    var regionCode: String?
    var fullAddressIncludingRegionMultiline: String?
    var fullAddressIncludingRegionSingleLine: String?
    var fullAddressExcludingRegionMultiline: String?
    var fullAddressExcludingRegionSingleLine: String?
}

@available(macOS 26.0, *)
func mkrBuildAddress(_ payload: MKRAddressPayload?) throws -> MKAddress? {
    guard let payload else { return nil }
    guard let address = MKAddress(fullAddress: payload.fullAddress, shortAddress: payload.shortAddress) else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "failed to create MKAddress"]
        )
    }
    return address
}

@available(macOS 26.0, *)
func mkrEncodeAddress(_ address: MKAddress?) -> MKRAddressPayload? {
    guard let address else { return nil }
    return MKRAddressPayload(fullAddress: address.fullAddress, shortAddress: address.shortAddress)
}

@available(macOS 26.0, *)
func mkrEncodeAddressRepresentations(
    _ addressRepresentations: MKAddressRepresentations?
) -> MKRAddressRepresentationsPayload? {
    guard let addressRepresentations else { return nil }
    return MKRAddressRepresentationsPayload(
        cityName: addressRepresentations.cityName,
        cityWithContext: addressRepresentations.cityWithContext,
        cityWithContextShort: addressRepresentations.cityWithContext(.short),
        cityWithContextFull: addressRepresentations.cityWithContext(.full),
        regionName: addressRepresentations.regionName,
        regionCode: nil,
        fullAddressIncludingRegionMultiline: addressRepresentations.fullAddress(includingRegion: true, singleLine: false),
        fullAddressIncludingRegionSingleLine: addressRepresentations.fullAddress(includingRegion: true, singleLine: true),
        fullAddressExcludingRegionMultiline: addressRepresentations.fullAddress(includingRegion: false, singleLine: false),
        fullAddressExcludingRegionSingleLine: addressRepresentations.fullAddress(includingRegion: false, singleLine: true)
    )
}

@available(macOS 15.0, *)
func mkrBuildAddressFilter(_ payload: MKRAddressFilterPayload?) throws -> MKAddressFilter? {
    guard let payload else { return nil }
    let options = MKAddressFilter.Options(rawValue: UInt(payload.options))
    switch payload.mode {
    case .including:
        return MKAddressFilter(including: options)
    case .excluding:
        return MKAddressFilter(excluding: options)
    case .includingAll:
        return .includingAll
    case .excludingAll:
        return .excludingAll
    }
}


@available(macOS 15.0, *)
func mkrEncodeAddressFilter(_ filter: MKAddressFilter?) -> MKRAddressFilterPayload? {
    guard let filter else { return nil }

    let knownOptions: [(UInt64, MKAddressFilter.Options)] = [
        (1 << 0, .country),
        (1 << 1, .administrativeArea),
        (1 << 2, .subAdministrativeArea),
        (1 << 3, .locality),
        (1 << 4, .subLocality),
        (1 << 5, .postalCode)
    ]

    var included: UInt64 = 0
    var excluded: UInt64 = 0
    for (bits, option) in knownOptions {
        if filter.includes(option) {
            included |= bits
        }
        if filter.excludes(option) {
            excluded |= bits
        }
    }

    if included == 0b11_1111 {
        return MKRAddressFilterPayload(mode: .includingAll, options: included)
    }
    if excluded == 0b11_1111 {
        return MKRAddressFilterPayload(mode: .excludingAll, options: excluded)
    }
    if included != 0 {
        return MKRAddressFilterPayload(mode: .including, options: included)
    }
    if excluded != 0 {
        return MKRAddressFilterPayload(mode: .excluding, options: excluded)
    }
    return nil
}
