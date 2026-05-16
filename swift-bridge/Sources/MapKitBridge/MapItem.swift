import CoreLocation
import Foundation
import MapKit

struct MKRPlacemarkPayload: Codable {
    var coordinate: MKRCoordinatePayload
    var countryCode: String?
    var title: String?
}

struct MKRMapItemPayload: Codable {
    var identifier: String?
    var alternateIdentifiers: [String]
    var name: String?
    var phoneNumber: String?
    var url: String?
    var timeZoneIdentifier: String?
    var pointOfInterestCategory: String?
    var isCurrentLocation: Bool
    var placemark: MKRPlacemarkPayload?
    var location: MKRCoordinatePayload?
    var address: MKRAddressPayload?
    var addressRepresentations: MKRAddressRepresentationsPayload?
}

func mkrBuildPlacemark(_ payload: MKRPlacemarkPayload) -> MKPlacemark {
    MKPlacemark(coordinate: mkrCoordinate(from: payload.coordinate))
}

func mkrEncodePlacemark(_ placemark: MKPlacemark) -> MKRPlacemarkPayload {
    MKRPlacemarkPayload(
        coordinate: mkrEncodeCoordinate(placemark.coordinate),
        countryCode: placemark.countryCode,
        title: placemark.title
    )
}

func mkrBuildMapItem(_ payload: MKRMapItemPayload) throws -> MKMapItem {
    if payload.isCurrentLocation && payload.placemark == nil && payload.location == nil {
        return MKMapItem.forCurrentLocation()
    }

    let item: MKMapItem
    if #available(macOS 26.0, *), let location = payload.location {
        let address = try mkrBuildAddress(payload.address)
        item = MKMapItem(
            location: CLLocation(
                latitude: location.latitude,
                longitude: location.longitude
            ),
            address: address
        )
    } else if let placemark = payload.placemark {
        item = MKMapItem(placemark: mkrBuildPlacemark(placemark))
    } else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "MKMapItem payload is missing a location or placemark"]
        )
    }

    item.name = payload.name
    item.phoneNumber = payload.phoneNumber
    item.url = payload.url.flatMap(URL.init(string:))
    item.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    item.pointOfInterestCategory = payload.pointOfInterestCategory.map(mkrPointOfInterestCategory(from:))
    return item
}

func mkrEncodeMapItem(_ item: MKMapItem) -> MKRMapItemPayload {
    let identifier: String?
    let alternateIdentifiers: [String]
    if #available(macOS 15.0, *) {
        identifier = item.identifier?.rawValue
        alternateIdentifiers = item.alternateIdentifiers.map(\.rawValue)
    } else {
        identifier = nil
        alternateIdentifiers = []
    }

    let location: MKRCoordinatePayload?
    if #available(macOS 26.0, *) {
        location = mkrEncodeCoordinate(item.location.coordinate)
    } else if item.isCurrentLocation {
        location = nil
    } else {
        location = mkrEncodeCoordinate(item.placemark.coordinate)
    }

    let placemark = item.isCurrentLocation ? nil : mkrEncodePlacemark(item.placemark)
    let address: MKRAddressPayload?
    let addressRepresentations: MKRAddressRepresentationsPayload?
    if #available(macOS 26.0, *) {
        address = mkrEncodeAddress(item.address)
        addressRepresentations = mkrEncodeAddressRepresentations(item.addressRepresentations)
    } else {
        address = nil
        addressRepresentations = nil
    }

    return MKRMapItemPayload(
        identifier: identifier,
        alternateIdentifiers: alternateIdentifiers,
        name: item.name,
        phoneNumber: item.phoneNumber,
        url: item.url?.absoluteString,
        timeZoneIdentifier: item.timeZone?.identifier,
        pointOfInterestCategory: item.pointOfInterestCategory?.rawValue,
        isCurrentLocation: item.isCurrentLocation,
        placemark: placemark,
        location: location,
        address: address,
        addressRepresentations: addressRepresentations
    )
}
