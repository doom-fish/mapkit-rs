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

struct MKRMapItemRequestStatePayload: Codable {
    var mapItemIdentifier: String?
    var cancelled: Bool
    var loading: Bool
}

private enum MKRMapItemStringConstantKind: Int32 {
    case launchOptionsCameraKey = 0
    case launchOptionsDirectionsModeCycling = 1
    case launchOptionsDirectionsModeDefault = 2
    case launchOptionsDirectionsModeDriving = 3
    case launchOptionsDirectionsModeKey = 4
    case launchOptionsDirectionsModeTransit = 5
    case launchOptionsDirectionsModeWalking = 6
    case launchOptionsMapCenterKey = 7
    case launchOptionsMapSpanKey = 8
    case launchOptionsMapTypeKey = 9
    case launchOptionsShowsTrafficKey = 10
    case mapItemTypeIdentifier = 11
}

@_cdecl("mk_map_item_string_constant")
public func mk_map_item_string_constant(
    _ kind: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        guard let kind = MKRMapItemStringConstantKind(rawValue: kind) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown MKMapItem string constant kind"]
            )
        }

        let value: String
        switch kind {
        case .launchOptionsCameraKey:
            value = MKLaunchOptionsCameraKey
        case .launchOptionsDirectionsModeCycling:
            value = MKLaunchOptionsDirectionsModeCycling
        case .launchOptionsDirectionsModeDefault:
            value = MKLaunchOptionsDirectionsModeDefault
        case .launchOptionsDirectionsModeDriving:
            value = MKLaunchOptionsDirectionsModeDriving
        case .launchOptionsDirectionsModeKey:
            value = MKLaunchOptionsDirectionsModeKey
        case .launchOptionsDirectionsModeTransit:
            value = MKLaunchOptionsDirectionsModeTransit
        case .launchOptionsDirectionsModeWalking:
            value = MKLaunchOptionsDirectionsModeWalking
        case .launchOptionsMapCenterKey:
            value = MKLaunchOptionsMapCenterKey
        case .launchOptionsMapSpanKey:
            value = MKLaunchOptionsMapSpanKey
        case .launchOptionsMapTypeKey:
            value = MKLaunchOptionsMapTypeKey
        case .launchOptionsShowsTrafficKey:
            value = MKLaunchOptionsShowsTrafficKey
        case .mapItemTypeIdentifier:
            value = MKMapItemTypeIdentifier
        }
        return mkrCString(value)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_item_request_new")
public func mk_map_item_request_new(
    _ mapItemIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.0, *) else {
        mkrSetMessageError(outError, message: "MKMapItemRequest requires macOS 15.0")
        return nil
    }
    guard let mapItemIdentifier else {
        mkrSetMessageError(outError, message: "missing MKMapItemIdentifier")
        return nil
    }

    let identifierString = String(cString: mapItemIdentifier)
    guard let identifier = MKMapItem.Identifier(rawValue: identifierString) else {
        mkrSetMessageError(
            outError,
            message: "invalid MKMapItemIdentifier raw value: \(identifierString)"
        )
        return nil
    }

    return mkrRetain(MKMapItemRequest(mapItemIdentifier: identifier))
}

@_cdecl("mk_map_item_request_state_json")
public func mk_map_item_request_state_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        mkrSetMessageError(outError, message: "MKMapItemRequest requires macOS 15.0")
        return nil
    }
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKMapItemRequest")
        return nil
    }

    do {
        let bridge = mkrBorrow(request, as: MKMapItemRequest.self)
        let payload = MKRMapItemRequestStatePayload(
            mapItemIdentifier: bridge.mapItemIdentifier?.rawValue,
            cancelled: bridge.isCancelled,
            loading: bridge.isLoading
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_item_request_get_map_item_json")
public func mk_map_item_request_get_map_item_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        mkrSetMessageError(outError, message: "MKMapItemRequest requires macOS 15.0")
        return nil
    }
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKMapItemRequest")
        return nil
    }

    let bridge = mkrBorrow(request, as: MKMapItemRequest.self)
    do {
        let mapItem: MKMapItem = try mkrAwaitOnMain { completion in
            bridge.getMapItem(completionHandler: { mapItem, error in
                if let mapItem {
                    completion(.success(mapItem))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKMapItemRequest completed without a map item"]
                    )))
                }
            })
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeMapItem(mapItem)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_item_request_cancel")
public func mk_map_item_request_cancel(_ request: UnsafeMutableRawPointer?) {
    guard #available(macOS 15.0, *), let request else { return }
    let bridge = mkrBorrow(request, as: MKMapItemRequest.self)
    bridge.cancel()
}

@_cdecl("mk_map_item_request_release")
public func mk_map_item_request_release(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    mkrRelease(request)
}
