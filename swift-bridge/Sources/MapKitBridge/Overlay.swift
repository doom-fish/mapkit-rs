import Foundation
import MapKit

struct MKRCirclePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var radius: Double
}

struct MKRCircleStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var radius: Double
    var boundingMapRect: MKRMapRectPayload
}

struct MKRMultiPointStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var boundingMapRect: MKRMapRectPayload
    var pointCount: Int
    var coordinates: [MKRCoordinatePayload]
    var interiorPolygonCount: Int?
}

private func mkrEncodeMultiPointState(_ multiPoint: MKMultiPoint) -> MKRMultiPointStatePayload {
    let pointCount = multiPoint.pointCount
    var coordinates = Array(
        repeating: CLLocationCoordinate2D(latitude: 0, longitude: 0),
        count: pointCount
    )
    if pointCount > 0 {
        multiPoint.getCoordinates(&coordinates, range: NSRange(location: 0, length: pointCount))
    }
    return MKRMultiPointStatePayload(
        coordinate: mkrEncodeCoordinate(multiPoint.coordinate),
        boundingMapRect: mkrEncodeMapRect((multiPoint as! MKOverlay).boundingMapRect),
        pointCount: pointCount,
        coordinates: mkrEncodeCoordinates(coordinates),
        interiorPolygonCount: (multiPoint as? MKPolygon)?.interiorPolygons?.count
    )
}

@_cdecl("mk_circle_new_json")
public func mk_circle_new_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRCirclePayload.self)
        let circle = MKCircle(
            center: mkrCoordinate(from: payload.coordinate),
            radius: payload.radius
        )
        return mkrRetain(circle)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_circle_state_json")
public func mk_circle_state_json(
    _ circle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let circle else {
        mkrSetMessageError(outError, message: "missing MKCircle")
        return nil
    }

    do {
        let overlay = mkrBorrow(circle, as: MKCircle.self)
        let payload = MKRCircleStatePayload(
            coordinate: mkrEncodeCoordinate(overlay.coordinate),
            radius: overlay.radius,
            boundingMapRect: mkrEncodeMapRect(overlay.boundingMapRect)
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_circle_release")
public func mk_circle_release(_ circle: UnsafeMutableRawPointer?) {
    guard let circle else { return }
    mkrRelease(circle)
}

@_cdecl("mk_polyline_new_json")
public func mk_polyline_new_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(payloadJSON, as: [MKRCoordinatePayload].self)
        let coordinates = mkrCoordinates(from: payload)
        guard !coordinates.isEmpty else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKPolyline requires at least one coordinate"]
            )
        }
        let polyline = coordinates.withUnsafeBufferPointer { buffer in
            MKPolyline(coordinates: buffer.baseAddress!, count: buffer.count)
        }
        return mkrRetain(polyline)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polyline_state_json")
public func mk_polyline_state_json(
    _ polyline: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let polyline else {
        mkrSetMessageError(outError, message: "missing MKPolyline")
        return nil
    }

    do {
        let overlay = mkrBorrow(polyline, as: MKPolyline.self)
        return mkrCString(try mkrEncodeJSON(mkrEncodeMultiPointState(overlay)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polyline_release")
public func mk_polyline_release(_ polyline: UnsafeMutableRawPointer?) {
    guard let polyline else { return }
    mkrRelease(polyline)
}

@_cdecl("mk_polygon_new_json")
public func mk_polygon_new_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(payloadJSON, as: [MKRCoordinatePayload].self)
        let coordinates = mkrCoordinates(from: payload)
        guard !coordinates.isEmpty else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKPolygon requires at least one coordinate"]
            )
        }
        let polygon = coordinates.withUnsafeBufferPointer { buffer in
            MKPolygon(coordinates: buffer.baseAddress!, count: buffer.count)
        }
        return mkrRetain(polygon)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polygon_state_json")
public func mk_polygon_state_json(
    _ polygon: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let polygon else {
        mkrSetMessageError(outError, message: "missing MKPolygon")
        return nil
    }

    do {
        let overlay = mkrBorrow(polygon, as: MKPolygon.self)
        return mkrCString(try mkrEncodeJSON(mkrEncodeMultiPointState(overlay)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polygon_release")
public func mk_polygon_release(_ polygon: UnsafeMutableRawPointer?) {
    guard let polygon else { return }
    mkrRelease(polygon)
}
