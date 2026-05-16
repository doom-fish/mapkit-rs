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
