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

struct MKRScreenPointPayload: Codable {
    var x: Double
    var y: Double
}

struct MKRScreenSizePayload: Codable {
    var width: Double
    var height: Double
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
