import Foundation
import MapKit

enum MKRLocalSearchRegionPriorityPayload: String, Codable {
    case `default`
    case required
}

struct MKRLocalSearchRequestPayload: Codable {
    var naturalLanguageQuery: String
    var region: MKRCoordinateRegionPayload?
    var resultTypes: UInt
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
    var addressFilter: MKRAddressFilterPayload?
    var regionPriority: MKRLocalSearchRegionPriorityPayload
}

struct MKRLocalSearchResponsePayload: Codable {
    var mapItems: [MKRMapItemPayload]
    var boundingRegion: MKRCoordinateRegionPayload
}

func mkrBuildLocalSearchRequest(_ payload: MKRLocalSearchRequestPayload) throws -> MKLocalSearch.Request {
    let request = MKLocalSearch.Request()
    request.naturalLanguageQuery = payload.naturalLanguageQuery
    if let region = payload.region {
        request.region = mkrRegion(from: region)
    }
    request.resultTypes = .init(rawValue: payload.resultTypes)
    if #available(macOS 10.15, *) {
        request.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
    }
    if #available(macOS 15.0, *) {
        if let addressFilter = try mkrBuildAddressFilter(payload.addressFilter) {
            request.addressFilter = addressFilter
        }
    } else if payload.addressFilter != nil {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "addressFilter requires macOS 15.0+"]
        )
    }
    if #available(macOS 15.0, *) {
        request.regionPriority = payload.regionPriority == .required ? .required : .default
    }
    return request
}

func mkrBuildLocalPointsOfInterestRequest(
    _ payload: MKRLocalPointsOfInterestRequestPayload
) throws -> MKLocalPointsOfInterestRequest {
    let request: MKLocalPointsOfInterestRequest
    if let region = payload.region {
        request = MKLocalPointsOfInterestRequest(coordinateRegion: mkrRegion(from: region))
    } else if let coordinate = payload.coordinate, let radius = payload.radius {
        request = MKLocalPointsOfInterestRequest(center: mkrCoordinate(from: coordinate), radius: radius)
    } else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "MKLocalPointsOfInterestRequest payload must include region or coordinate+radius"]
        )
    }
    request.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
    return request
}

func mkrEncodeLocalSearchResponse(_ response: MKLocalSearch.Response) -> MKRLocalSearchResponsePayload {
    MKRLocalSearchResponsePayload(
        mapItems: response.mapItems.map(mkrEncodeMapItem),
        boundingRegion: mkrEncodeRegion(response.boundingRegion)
    )
}

@_cdecl("mk_local_search_new")
public func mk_local_search_new(
    _ requestJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(requestJSON, as: MKRLocalSearchRequestPayload.self)
        let search = MKLocalSearch(request: try mkrBuildLocalSearchRequest(payload))
        return mkrRetain(search)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_local_search_new_points_of_interest")
public func mk_local_search_new_points_of_interest(
    _ requestJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(requestJSON, as: MKRLocalPointsOfInterestRequestPayload.self)
        let search = MKLocalSearch(request: try mkrBuildLocalPointsOfInterestRequest(payload))
        return mkrRetain(search)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_local_search_start_json")
public func mk_local_search_start_json(
    _ search: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let search else {
        mkrSetMessageError(outError, message: "missing MKLocalSearch")
        return nil
    }

    let localSearch = mkrBorrow(search, as: MKLocalSearch.self)
    do {
        let response = try mkrAwaitOnMain { completion in
            localSearch.start { response, error in
                if let response {
                    completion(.success(response))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKLocalSearch failed without a response"]
                    )))
                }
            }
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeLocalSearchResponse(response)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_local_search_cancel")
public func mk_local_search_cancel(_ search: UnsafeMutableRawPointer?) {
    guard let search else { return }
    let localSearch = mkrBorrow(search, as: MKLocalSearch.self)
    localSearch.cancel()
}

@_cdecl("mk_local_search_is_searching")
public func mk_local_search_is_searching(_ search: UnsafeMutableRawPointer?) -> Bool {
    guard let search else { return false }
    let localSearch = mkrBorrow(search, as: MKLocalSearch.self)
    return localSearch.isSearching
}

@_cdecl("mk_local_search_release")
public func mk_local_search_release(_ search: UnsafeMutableRawPointer?) {
    guard let search else { return }
    mkrRelease(search)
}
