import CoreLocation
import Foundation
import MapKit

struct MKRCoordinatePayload: Codable {
    var latitude: Double
    var longitude: Double
}

struct MKRCoordinateSpanPayload: Codable {
    var latitudeDelta: Double
    var longitudeDelta: Double
}

struct MKRCoordinateRegionPayload: Codable {
    var center: MKRCoordinatePayload
    var span: MKRCoordinateSpanPayload
}

struct MKRMapPointPayload: Codable {
    var x: Double
    var y: Double
}

struct MKRPlacemarkPayload: Codable {
    var coordinate: MKRCoordinatePayload
    var countryCode: String?
    var title: String?
}

struct MKRMapItemPayload: Codable {
    var identifier: String?
    var name: String?
    var phoneNumber: String?
    var url: String?
    var timeZoneIdentifier: String?
    var pointOfInterestCategory: String?
    var isCurrentLocation: Bool
    var placemark: MKRPlacemarkPayload?
}

struct MKRLocalSearchRequestPayload: Codable {
    var naturalLanguageQuery: String
    var region: MKRCoordinateRegionPayload?
    var resultTypes: UInt
}

struct MKRLocalSearchResponsePayload: Codable {
    var mapItems: [MKRMapItemPayload]
    var boundingRegion: MKRCoordinateRegionPayload
}

enum MKRDirectionsRoutePreferencePayload: String, Codable {
    case any
    case avoid
}

struct MKRDirectionsRequestPayload: Codable {
    var source: MKRMapItemPayload
    var destination: MKRMapItemPayload
    var transportType: UInt
    var requestsAlternateRoutes: Bool
    var tollPreference: MKRDirectionsRoutePreferencePayload
    var highwayPreference: MKRDirectionsRoutePreferencePayload
}

struct MKRRouteStepPayload: Codable {
    var instructions: String
    var notice: String?
    var distance: Double
    var transportType: UInt
}

struct MKRRoutePayload: Codable {
    var name: String
    var advisoryNotices: [String]
    var distance: Double
    var expectedTravelTime: Double
    var transportType: UInt
    var hasTolls: Bool
    var hasHighways: Bool
    var steps: [MKRRouteStepPayload]
}

struct MKRDirectionsResponsePayload: Codable {
    var source: MKRMapItemPayload
    var destination: MKRMapItemPayload
    var routes: [MKRRoutePayload]
}

struct MKRETAResponsePayload: Codable {
    var source: MKRMapItemPayload
    var destination: MKRMapItemPayload
    var expectedTravelTime: Double
    var distance: Double
    var expectedArrivalDate: String?
    var expectedDepartureDate: String?
    var transportType: UInt
}

func mkrCoordinate(from payload: MKRCoordinatePayload) -> CLLocationCoordinate2D {
    CLLocationCoordinate2D(latitude: payload.latitude, longitude: payload.longitude)
}

func mkrEncodeCoordinate(_ coordinate: CLLocationCoordinate2D) -> MKRCoordinatePayload {
    MKRCoordinatePayload(latitude: coordinate.latitude, longitude: coordinate.longitude)
}

func mkrSpan(from payload: MKRCoordinateSpanPayload) -> MKCoordinateSpan {
    MKCoordinateSpan(latitudeDelta: payload.latitudeDelta, longitudeDelta: payload.longitudeDelta)
}

func mkrEncodeSpan(_ span: MKCoordinateSpan) -> MKRCoordinateSpanPayload {
    MKRCoordinateSpanPayload(
        latitudeDelta: span.latitudeDelta,
        longitudeDelta: span.longitudeDelta
    )
}

func mkrRegion(from payload: MKRCoordinateRegionPayload) -> MKCoordinateRegion {
    MKCoordinateRegion(center: mkrCoordinate(from: payload.center), span: mkrSpan(from: payload.span))
}

func mkrEncodeRegion(_ region: MKCoordinateRegion) -> MKRCoordinateRegionPayload {
    MKRCoordinateRegionPayload(center: mkrEncodeCoordinate(region.center), span: mkrEncodeSpan(region.span))
}

func mkrMapPoint(from payload: MKRMapPointPayload) -> MKMapPoint {
    MKMapPoint(x: payload.x, y: payload.y)
}

func mkrEncodeMapPoint(_ mapPoint: MKMapPoint) -> MKRMapPointPayload {
    MKRMapPointPayload(x: mapPoint.x, y: mapPoint.y)
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
    if payload.isCurrentLocation && payload.placemark == nil {
        return MKMapItem.forCurrentLocation()
    }

    guard let placemark = payload.placemark else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "MKMapItem payload is missing a placemark"]
        )
    }

    let item = MKMapItem(placemark: mkrBuildPlacemark(placemark))
    item.name = payload.name
    item.phoneNumber = payload.phoneNumber
    item.url = payload.url.flatMap(URL.init(string:))
    item.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    item.pointOfInterestCategory = payload.pointOfInterestCategory.map(MKPointOfInterestCategory.init(rawValue:))
    return item
}

func mkrEncodeMapItem(_ item: MKMapItem) -> MKRMapItemPayload {
    let identifier: String?
    if #available(macOS 15.0, *) {
        identifier = item.identifier?.rawValue
    } else {
        identifier = nil
    }

    let placemark = item.isCurrentLocation ? nil : mkrEncodePlacemark(item.placemark)
    return MKRMapItemPayload(
        identifier: identifier,
        name: item.name,
        phoneNumber: item.phoneNumber,
        url: item.url?.absoluteString,
        timeZoneIdentifier: item.timeZone?.identifier,
        pointOfInterestCategory: item.pointOfInterestCategory?.rawValue,
        isCurrentLocation: item.isCurrentLocation,
        placemark: placemark
    )
}

func mkrBuildLocalSearchRequest(_ payload: MKRLocalSearchRequestPayload) -> MKLocalSearch.Request {
    let request = MKLocalSearch.Request()
    request.naturalLanguageQuery = payload.naturalLanguageQuery
    if let region = payload.region {
        request.region = mkrRegion(from: region)
    }
    request.resultTypes = .init(rawValue: payload.resultTypes)
    return request
}

func mkrEncodeLocalSearchResponse(_ response: MKLocalSearch.Response) -> MKRLocalSearchResponsePayload {
    MKRLocalSearchResponsePayload(
        mapItems: response.mapItems.map(mkrEncodeMapItem),
        boundingRegion: mkrEncodeRegion(response.boundingRegion)
    )
}

func mkrBuildDirectionsRequest(_ payload: MKRDirectionsRequestPayload) throws -> MKDirections.Request {
    let request = MKDirections.Request()
    request.source = try mkrBuildMapItem(payload.source)
    request.destination = try mkrBuildMapItem(payload.destination)
    request.transportType = .init(rawValue: payload.transportType)
    request.requestsAlternateRoutes = payload.requestsAlternateRoutes
    request.tollPreference = payload.tollPreference == .avoid ? .avoid : .any
    request.highwayPreference = payload.highwayPreference == .avoid ? .avoid : .any
    return request
}

func mkrEncodeRouteStep(_ step: MKRoute.Step) -> MKRRouteStepPayload {
    MKRRouteStepPayload(
        instructions: step.instructions,
        notice: step.notice,
        distance: step.distance,
        transportType: step.transportType.rawValue
    )
}

func mkrEncodeRoute(_ route: MKRoute) -> MKRRoutePayload {
    MKRRoutePayload(
        name: route.name,
        advisoryNotices: route.advisoryNotices,
        distance: route.distance,
        expectedTravelTime: route.expectedTravelTime,
        transportType: route.transportType.rawValue,
        hasTolls: route.hasTolls,
        hasHighways: route.hasHighways,
        steps: route.steps.map(mkrEncodeRouteStep)
    )
}

func mkrEncodeDirectionsResponse(_ response: MKDirections.Response) -> MKRDirectionsResponsePayload {
    MKRDirectionsResponsePayload(
        source: mkrEncodeMapItem(response.source),
        destination: mkrEncodeMapItem(response.destination),
        routes: response.routes.map(mkrEncodeRoute)
    )
}

func mkrEncodeETAResponse(_ response: MKDirections.ETAResponse) -> MKRETAResponsePayload {
    MKRETAResponsePayload(
        source: mkrEncodeMapItem(response.source),
        destination: mkrEncodeMapItem(response.destination),
        expectedTravelTime: response.expectedTravelTime,
        distance: response.distance,
        expectedArrivalDate: mkrDateString(response.expectedArrivalDate),
        expectedDepartureDate: mkrDateString(response.expectedDepartureDate),
        transportType: response.transportType.rawValue
    )
}

@_cdecl("mk_coordinate_region_make_with_distance_json")
public func mk_coordinate_region_make_with_distance_json(
    _ centerJSON: UnsafePointer<CChar>?,
    _ latitudinalMeters: Double,
    _ longitudinalMeters: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let center = try mkrDecodeJSON(centerJSON, as: MKRCoordinatePayload.self)
        let region = MKCoordinateRegion(
            center: mkrCoordinate(from: center),
            latitudinalMeters: latitudinalMeters,
            longitudinalMeters: longitudinalMeters
        )
        return mkrCString(try mkrEncodeJSON(mkrEncodeRegion(region)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_point_for_coordinate_json")
public func mk_map_point_for_coordinate_json(
    _ coordinateJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let coordinate = try mkrDecodeJSON(coordinateJSON, as: MKRCoordinatePayload.self)
        let mapPoint = MKMapPoint(mkrCoordinate(from: coordinate))
        return mkrCString(try mkrEncodeJSON(mkrEncodeMapPoint(mapPoint)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_coordinate_for_map_point_json")
public func mk_coordinate_for_map_point_json(
    _ mapPointJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let mapPoint = try mkrDecodeJSON(mapPointJSON, as: MKRMapPointPayload.self)
        return mkrCString(try mkrEncodeJSON(mkrEncodeCoordinate(mkrMapPoint(from: mapPoint).coordinate)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_meters_between_map_points")
public func mk_meters_between_map_points(
    _ firstMapPointJSON: UnsafePointer<CChar>?,
    _ secondMapPointJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    do {
        let firstMapPoint = try mkrDecodeJSON(firstMapPointJSON, as: MKRMapPointPayload.self)
        let secondMapPoint = try mkrDecodeJSON(secondMapPointJSON, as: MKRMapPointPayload.self)
        return mkrMapPoint(from: firstMapPoint).distance(to: mkrMapPoint(from: secondMapPoint))
    } catch {
        mkrSetError(outError, error)
        return -1
    }
}

@_cdecl("mk_local_search_new")
public func mk_local_search_new(
    _ requestJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(requestJSON, as: MKRLocalSearchRequestPayload.self)
        let search = MKLocalSearch(request: mkrBuildLocalSearchRequest(payload))
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

@_cdecl("mk_directions_new")
public func mk_directions_new(
    _ requestJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(requestJSON, as: MKRDirectionsRequestPayload.self)
        let directions = MKDirections(request: try mkrBuildDirectionsRequest(payload))
        return mkrRetain(directions)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_directions_calculate_json")
public func mk_directions_calculate_json(
    _ directions: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let directions else {
        mkrSetMessageError(outError, message: "missing MKDirections")
        return nil
    }

    let routeService = mkrBorrow(directions, as: MKDirections.self)
    do {
        let response = try mkrAwaitOnMain { completion in
            routeService.calculate { response, error in
                if let response {
                    completion(.success(response))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKDirections failed without a response"]
                    )))
                }
            }
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeDirectionsResponse(response)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_directions_calculate_eta_json")
public func mk_directions_calculate_eta_json(
    _ directions: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let directions else {
        mkrSetMessageError(outError, message: "missing MKDirections")
        return nil
    }

    let routeService = mkrBorrow(directions, as: MKDirections.self)
    do {
        let response = try mkrAwaitOnMain { completion in
            routeService.calculateETA { response, error in
                if let response {
                    completion(.success(response))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKDirections ETA failed without a response"]
                    )))
                }
            }
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeETAResponse(response)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_directions_cancel")
public func mk_directions_cancel(_ directions: UnsafeMutableRawPointer?) {
    guard let directions else { return }
    let routeService = mkrBorrow(directions, as: MKDirections.self)
    routeService.cancel()
}

@_cdecl("mk_directions_is_calculating")
public func mk_directions_is_calculating(_ directions: UnsafeMutableRawPointer?) -> Bool {
    guard let directions else { return false }
    let routeService = mkrBorrow(directions, as: MKDirections.self)
    return routeService.isCalculating
}

@_cdecl("mk_directions_release")
public func mk_directions_release(_ directions: UnsafeMutableRawPointer?) {
    guard let directions else { return }
    mkrRelease(directions)
}

@_cdecl("mk_distance_formatter_new")
public func mk_distance_formatter_new() -> UnsafeMutableRawPointer {
    mkrRetain(MKDistanceFormatter())
}

@_cdecl("mk_distance_formatter_set_units")
public func mk_distance_formatter_set_units(_ formatter: UnsafeMutableRawPointer?, _ rawUnits: UInt64) {
    guard let formatter else { return }
    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    switch rawUnits {
    case 1:
        distanceFormatter.units = .metric
    case 2:
        distanceFormatter.units = .imperial
    case 3:
        distanceFormatter.units = .imperialWithYards
    default:
        distanceFormatter.units = .default
    }
}

@_cdecl("mk_distance_formatter_set_unit_style")
public func mk_distance_formatter_set_unit_style(_ formatter: UnsafeMutableRawPointer?, _ rawUnitStyle: UInt64) {
    guard let formatter else { return }
    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    switch rawUnitStyle {
    case 1:
        distanceFormatter.unitStyle = .abbreviated
    case 2:
        distanceFormatter.unitStyle = .full
    default:
        distanceFormatter.unitStyle = .default
    }
}

@_cdecl("mk_distance_formatter_string_from_distance")
public func mk_distance_formatter_string_from_distance(
    _ formatter: UnsafeMutableRawPointer?,
    _ distance: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let formatter else {
        mkrSetMessageError(outError, message: "missing MKDistanceFormatter")
        return nil
    }

    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    return mkrCString(distanceFormatter.string(fromDistance: distance))
}

@_cdecl("mk_distance_formatter_distance_from_string")
public func mk_distance_formatter_distance_from_string(
    _ formatter: UnsafeMutableRawPointer?,
    _ distanceString: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    guard let formatter else {
        mkrSetMessageError(outError, message: "missing MKDistanceFormatter")
        return -1
    }
    guard let distanceString else {
        mkrSetMessageError(outError, message: "missing distance string")
        return -1
    }

    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    let parsed = distanceFormatter.distance(from: String(cString: distanceString))
    if parsed < 0 {
        mkrSetMessageError(outError, message: "failed to parse distance string")
    }
    return parsed
}

@_cdecl("mk_distance_formatter_release")
public func mk_distance_formatter_release(_ formatter: UnsafeMutableRawPointer?) {
    guard let formatter else { return }
    mkrRelease(formatter)
}
