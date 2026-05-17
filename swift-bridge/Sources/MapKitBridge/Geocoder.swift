import CoreLocation
import Foundation
import MapKit

struct MKRGeocodingRequestStatePayload: Codable {
    var addressString: String
    var region: MKRCoordinateRegionPayload
    var preferredLocaleIdentifier: String?
    var cancelled: Bool
    var loading: Bool
}

struct MKRReverseGeocodingRequestStatePayload: Codable {
    var location: MKRCoordinatePayload
    var preferredLocaleIdentifier: String?
    var cancelled: Bool
    var loading: Bool
}

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_new")
public func mk_geocoding_request_new(
    _ addressString: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let addressString else {
        mkrSetMessageError(outError, message: "missing geocoding address string")
        return nil
    }

    do {
        guard let request = MKGeocodingRequest(addressString: String(cString: addressString)) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKGeocodingRequest"]
            )
        }
        return mkrRetain(request)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_state_json")
public func mk_geocoding_request_state_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKGeocodingRequest")
        return nil
    }

    do {
        let geocoder = mkrBorrow(request, as: MKGeocodingRequest.self)
        let payload = MKRGeocodingRequestStatePayload(
            addressString: geocoder.addressString,
            region: mkrEncodeRegion(geocoder.region),
            preferredLocaleIdentifier: geocoder.preferredLocale?.identifier,
            cancelled: geocoder.isCancelled,
            loading: geocoder.isLoading
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_set_region_json")
public func mk_geocoding_request_set_region_json(
    _ request: UnsafeMutableRawPointer?,
    _ regionJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKGeocodingRequest")
        return
    }

    do {
        let geocoder = mkrBorrow(request, as: MKGeocodingRequest.self)
        let region = try mkrDecodeJSON(regionJSON, as: MKRCoordinateRegionPayload.self)
        geocoder.region = mkrRegion(from: region)
    } catch {
        mkrSetError(outError, error)
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_set_preferred_locale")
public func mk_geocoding_request_set_preferred_locale(
    _ request: UnsafeMutableRawPointer?,
    _ localeIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKGeocodingRequest")
        return
    }

    do {
        let geocoder = mkrBorrow(request, as: MKGeocodingRequest.self)
        geocoder.preferredLocale = localeIdentifier.map { Locale(identifier: String(cString: $0)) }
    } catch {
        mkrSetError(outError, error)
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_get_map_items_json")
public func mk_geocoding_request_get_map_items_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKGeocodingRequest")
        return nil
    }

    do {
        let geocoder = mkrBorrow(request, as: MKGeocodingRequest.self)
        let mapItems = try mkrAwaitOnMain { completion in
            geocoder.getMapItems { mapItems, error in
                if let mapItems {
                    completion(.success(mapItems))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKGeocodingRequest failed without map items"]
                    )))
                }
            }
        }
        return mkrCString(try mkrEncodeJSON(mapItems.map(mkrEncodeMapItem)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_cancel")
public func mk_geocoding_request_cancel(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    let geocoder = mkrBorrow(request, as: MKGeocodingRequest.self)
    geocoder.cancel()
}

@_cdecl("mk_geocoding_request_release")
public func mk_geocoding_request_release(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    mkrRelease(request)
}

@available(macOS 26.0, *)
@_cdecl("mk_reverse_geocoding_request_new_json")
public func mk_reverse_geocoding_request_new_json(
    _ locationJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let location = try mkrDecodeJSON(locationJSON, as: MKRCoordinatePayload.self)
        guard let request = MKReverseGeocodingRequest(
            location: CLLocation(
                latitude: location.latitude,
                longitude: location.longitude
            )
        ) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKReverseGeocodingRequest"]
            )
        }
        return mkrRetain(request)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_reverse_geocoding_request_state_json")
public func mk_reverse_geocoding_request_state_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKReverseGeocodingRequest")
        return nil
    }

    do {
        let geocoder = mkrBorrow(request, as: MKReverseGeocodingRequest.self)
        let payload = MKRReverseGeocodingRequestStatePayload(
            location: mkrEncodeCoordinate(geocoder.location.coordinate),
            preferredLocaleIdentifier: geocoder.preferredLocale?.identifier,
            cancelled: geocoder.isCancelled,
            loading: geocoder.isLoading
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_reverse_geocoding_request_set_preferred_locale")
public func mk_reverse_geocoding_request_set_preferred_locale(
    _ request: UnsafeMutableRawPointer?,
    _ localeIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKReverseGeocodingRequest")
        return
    }

    do {
        let geocoder = mkrBorrow(request, as: MKReverseGeocodingRequest.self)
        geocoder.preferredLocale = localeIdentifier.map { Locale(identifier: String(cString: $0)) }
    } catch {
        mkrSetError(outError, error)
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_reverse_geocoding_request_get_map_items_json")
public func mk_reverse_geocoding_request_get_map_items_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKReverseGeocodingRequest")
        return nil
    }

    do {
        let geocoder = mkrBorrow(request, as: MKReverseGeocodingRequest.self)
        let mapItems = try mkrAwaitOnMain { completion in
            geocoder.getMapItems { mapItems, error in
                if let mapItems {
                    completion(.success(mapItems))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKReverseGeocodingRequest failed without map items"]
                    )))
                }
            }
        }
        return mkrCString(try mkrEncodeJSON(mapItems.map(mkrEncodeMapItem)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@available(macOS 26.0, *)
@_cdecl("mk_reverse_geocoding_request_cancel")
public func mk_reverse_geocoding_request_cancel(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    let geocoder = mkrBorrow(request, as: MKReverseGeocodingRequest.self)
    geocoder.cancel()
}

@_cdecl("mk_reverse_geocoding_request_release")
public func mk_reverse_geocoding_request_release(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    mkrRelease(request)
}
