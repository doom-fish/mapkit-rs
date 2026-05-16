import Foundation
import MapKit

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
