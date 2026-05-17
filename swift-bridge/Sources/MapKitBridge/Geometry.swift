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

struct MKRMapSizePayload: Codable {
    var width: Double
    var height: Double
}

struct MKRMapRectPayload: Codable {
    var origin: MKRMapPointPayload
    var size: MKRMapSizePayload
}

struct MKRMapRectDivisionPayload: Codable {
    var slice: MKRMapRectPayload
    var remainder: MKRMapRectPayload
}

struct MKRScreenPointPayload: Codable {
    var x: Double
    var y: Double
}

struct MKRScreenSizePayload: Codable {
    var width: Double
    var height: Double
}

private enum MKRGeometryConstantKind: Int32 {
    case mapSizeWorld = 0
    case mapRectWorld = 1
    case mapRectNull = 2
}

private enum MKRMapRectPredicateKind: Int32 {
    case containsPoint = 0
    case containsRect = 1
    case intersectsRect = 2
    case spans180thMeridian = 3
}

private enum MKRMapRectTransformKind: Int32 {
    case union = 0
    case intersection = 1
    case inset = 2
    case offset = 3
    case divide = 4
    case remainder = 5
}

private enum MKRMapRectEdgeKind: Int32 {
    case minX = 0
    case minY = 1
    case maxX = 2
    case maxY = 3
}

func mkrCoordinate(from payload: MKRCoordinatePayload) -> CLLocationCoordinate2D {
    CLLocationCoordinate2D(latitude: payload.latitude, longitude: payload.longitude)
}

func mkrEncodeCoordinate(_ coordinate: CLLocationCoordinate2D) -> MKRCoordinatePayload {
    MKRCoordinatePayload(latitude: coordinate.latitude, longitude: coordinate.longitude)
}

func mkrCoordinates(from payloads: [MKRCoordinatePayload]) -> [CLLocationCoordinate2D] {
    payloads.map(mkrCoordinate)
}

func mkrEncodeCoordinates(_ coordinates: [CLLocationCoordinate2D]) -> [MKRCoordinatePayload] {
    coordinates.map(mkrEncodeCoordinate)
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

func mkrMapSize(from payload: MKRMapSizePayload) -> MKMapSize {
    MKMapSize(width: payload.width, height: payload.height)
}

func mkrEncodeMapSize(_ mapSize: MKMapSize) -> MKRMapSizePayload {
    MKRMapSizePayload(width: mapSize.width, height: mapSize.height)
}

func mkrMapRect(from payload: MKRMapRectPayload) -> MKMapRect {
    MKMapRect(origin: mkrMapPoint(from: payload.origin), size: mkrMapSize(from: payload.size))
}

func mkrEncodeMapRect(_ mapRect: MKMapRect) -> MKRMapRectPayload {
    MKRMapRectPayload(origin: mkrEncodeMapPoint(mapRect.origin), size: mkrEncodeMapSize(mapRect.size))
}

func mkrScreenPoint(from payload: MKRScreenPointPayload) -> CGPoint {
    CGPoint(x: payload.x, y: payload.y)
}

func mkrEncodeScreenPoint(_ point: CGPoint) -> MKRScreenPointPayload {
    MKRScreenPointPayload(x: point.x, y: point.y)
}

func mkrScreenSize(from payload: MKRScreenSizePayload) -> CGSize {
    CGSize(width: payload.width, height: payload.height)
}

func mkrEncodeScreenSize(_ size: CGSize) -> MKRScreenSizePayload {
    MKRScreenSizePayload(width: size.width, height: size.height)
}

private func mkrRectEdge(from edge: MKRMapRectEdgeKind) -> CGRectEdge {
    switch edge {
    case .minX: return .minXEdge
    case .minY: return .minYEdge
    case .maxX: return .maxXEdge
    case .maxY: return .maxYEdge
    }
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

@_cdecl("mk_geometry_constant_json")
public func mk_geometry_constant_json(
    _ kind: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        guard let kind = MKRGeometryConstantKind(rawValue: kind) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown geometry constant kind"]
            )
        }

        let json: String
        switch kind {
        case .mapSizeWorld:
            json = try mkrEncodeJSON(mkrEncodeMapSize(MKMapSize.world))
        case .mapRectWorld:
            json = try mkrEncodeJSON(mkrEncodeMapRect(MKMapRect.world))
        case .mapRectNull:
            json = try mkrEncodeJSON(mkrEncodeMapRect(MKMapRect.null))
        }
        return mkrCString(json)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_coordinate_region_for_map_rect_json")
public func mk_coordinate_region_for_map_rect_json(
    _ mapRectJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let mapRect = try mkrDecodeJSON(mapRectJSON, as: MKRMapRectPayload.self)
        let region = MKCoordinateRegion(mkrMapRect(from: mapRect))
        return mkrCString(try mkrEncodeJSON(mkrEncodeRegion(region)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_points_per_meter_at_latitude")
public func mk_map_points_per_meter_at_latitude(
    _ latitude: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    _ = outError
    return MKMapPointsPerMeterAtLatitude(latitude)
}

@_cdecl("mk_meters_per_map_point_at_latitude")
public func mk_meters_per_map_point_at_latitude(
    _ latitude: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    _ = outError
    return MKMetersPerMapPointAtLatitude(latitude)
}

@_cdecl("mk_map_rect_predicate_json")
public func mk_map_rect_predicate_json(
    _ rectJSON: UnsafePointer<CChar>?,
    _ auxiliaryJSON: UnsafePointer<CChar>?,
    _ kind: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    do {
        guard let kind = MKRMapRectPredicateKind(rawValue: kind) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown MKMapRect predicate kind"]
            )
        }
        let rect = try mkrMapRect(from: mkrDecodeJSON(rectJSON, as: MKRMapRectPayload.self))
        switch kind {
        case .containsPoint:
            let point = try mkrMapPoint(from: mkrDecodeJSON(auxiliaryJSON, as: MKRMapPointPayload.self))
            return rect.contains(point)
        case .containsRect:
            let other = try mkrMapRect(from: mkrDecodeJSON(auxiliaryJSON, as: MKRMapRectPayload.self))
            return rect.contains(other)
        case .intersectsRect:
            let other = try mkrMapRect(from: mkrDecodeJSON(auxiliaryJSON, as: MKRMapRectPayload.self))
            return rect.intersects(other)
        case .spans180thMeridian:
            return rect.spans180thMeridian
        }
    } catch {
        mkrSetError(outError, error)
        return false
    }
}

@_cdecl("mk_map_rect_transform_json")
public func mk_map_rect_transform_json(
    _ rectJSON: UnsafePointer<CChar>?,
    _ otherRectJSON: UnsafePointer<CChar>?,
    _ dx: Double,
    _ dy: Double,
    _ amount: Double,
    _ edge: Int32,
    _ kind: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        guard let kind = MKRMapRectTransformKind(rawValue: kind) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown MKMapRect transform kind"]
            )
        }
        let rect = try mkrMapRect(from: mkrDecodeJSON(rectJSON, as: MKRMapRectPayload.self))

        let json: String
        switch kind {
        case .union:
            let other = try mkrMapRect(from: mkrDecodeJSON(otherRectJSON, as: MKRMapRectPayload.self))
            json = try mkrEncodeJSON(mkrEncodeMapRect(rect.union(other)))
        case .intersection:
            let other = try mkrMapRect(from: mkrDecodeJSON(otherRectJSON, as: MKRMapRectPayload.self))
            json = try mkrEncodeJSON(mkrEncodeMapRect(rect.intersection(other)))
        case .inset:
            json = try mkrEncodeJSON(mkrEncodeMapRect(rect.insetBy(dx: dx, dy: dy)))
        case .offset:
            json = try mkrEncodeJSON(mkrEncodeMapRect(rect.offsetBy(dx: dx, dy: dy)))
        case .divide:
            guard let edgeKind = MKRMapRectEdgeKind(rawValue: edge) else {
                throw NSError(
                    domain: "mapkit-rs",
                    code: -1,
                    userInfo: [NSLocalizedDescriptionKey: "unknown MKMapRect edge"]
                )
            }
            var slice = MKMapRect.null
            var remainder = MKMapRect.null
            MKMapRectDivide(rect, &slice, &remainder, amount, mkrRectEdge(from: edgeKind))
            json = try mkrEncodeJSON(MKRMapRectDivisionPayload(
                slice: mkrEncodeMapRect(slice),
                remainder: mkrEncodeMapRect(remainder)
            ))
        case .remainder:
            json = try mkrEncodeJSON(mkrEncodeMapRect(rect.remainder))
        }
        return mkrCString(json)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}
