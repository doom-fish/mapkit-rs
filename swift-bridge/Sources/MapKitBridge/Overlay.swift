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
    var canReplaceMapContent: Bool
}

struct MKRMultiPointStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var boundingMapRect: MKRMapRectPayload
    var canReplaceMapContent: Bool
    var pointCount: Int
    var coordinates: [MKRCoordinatePayload]
    var interiorPolygonCount: Int?
}

struct MKRTileOverlayPathPayload: Codable {
    var x: Int
    var y: Int
    var z: Int
    var contentScaleFactor: Double
}

struct MKRTileOverlayStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var boundingMapRect: MKRMapRectPayload
    var urlTemplate: String?
    var tileSize: MKRScreenSizePayload
    var geometryFlipped: Bool
    var minimumZ: Int
    var maximumZ: Int
    var canReplaceMapContent: Bool
}

struct MKRTileOverlayOptionsPayload: Codable {
    var tileSize: MKRScreenSizePayload?
    var geometryFlipped: Bool?
    var minimumZ: Int?
    var maximumZ: Int?
    var canReplaceMapContent: Bool?
}

struct MKRMultiPolylineStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var boundingMapRect: MKRMapRectPayload
    var canReplaceMapContent: Bool
    var polylineCount: Int
    var polylines: [[MKRCoordinatePayload]]
}

struct MKRMultiPolygonStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var boundingMapRect: MKRMapRectPayload
    var canReplaceMapContent: Bool
    var polygonCount: Int
    var polygons: [[MKRCoordinatePayload]]
}

func mkrBorrowOverlay(_ ptr: UnsafeMutableRawPointer?) throws -> any MKOverlay {
    guard let ptr else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "missing MKOverlay"]
        )
    }
    let object = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue()
    guard let overlay = object as? any MKOverlay else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "object does not conform to MKOverlay"]
        )
    }
    return overlay
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
    let overlay = multiPoint as! MKOverlay
    return MKRMultiPointStatePayload(
        coordinate: mkrEncodeCoordinate(multiPoint.coordinate),
        boundingMapRect: mkrEncodeMapRect(overlay.boundingMapRect),
        canReplaceMapContent: overlay.canReplaceMapContent?() ?? false,
        pointCount: pointCount,
        coordinates: mkrEncodeCoordinates(coordinates),
        interiorPolygonCount: (multiPoint as? MKPolygon)?.interiorPolygons?.count
    )
}

private func mkrEncodeTileOverlayState(_ overlay: MKTileOverlay) -> MKRTileOverlayStatePayload {
    MKRTileOverlayStatePayload(
        coordinate: mkrEncodeCoordinate(overlay.coordinate),
        boundingMapRect: mkrEncodeMapRect(overlay.boundingMapRect),
        urlTemplate: overlay.urlTemplate,
        tileSize: mkrEncodeScreenSize(overlay.tileSize),
        geometryFlipped: overlay.isGeometryFlipped,
        minimumZ: overlay.minimumZ,
        maximumZ: overlay.maximumZ,
        canReplaceMapContent: overlay.canReplaceMapContent
    )
}

private func mkrEncodePolylineCoordinates(_ polyline: MKPolyline) -> [MKRCoordinatePayload] {
    let pointCount = polyline.pointCount
    var coordinates = Array(
        repeating: CLLocationCoordinate2D(latitude: 0, longitude: 0),
        count: pointCount
    )
    if pointCount > 0 {
        polyline.getCoordinates(&coordinates, range: NSRange(location: 0, length: pointCount))
    }
    return mkrEncodeCoordinates(coordinates)
}

private func mkrEncodePolygonCoordinates(_ polygon: MKPolygon) -> [MKRCoordinatePayload] {
    let pointCount = polygon.pointCount
    var coordinates = Array(
        repeating: CLLocationCoordinate2D(latitude: 0, longitude: 0),
        count: pointCount
    )
    if pointCount > 0 {
        polygon.getCoordinates(&coordinates, range: NSRange(location: 0, length: pointCount))
    }
    return mkrEncodeCoordinates(coordinates)
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
            boundingMapRect: mkrEncodeMapRect(overlay.boundingMapRect),
            canReplaceMapContent: (overlay as MKOverlay).canReplaceMapContent?() ?? false
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

@_cdecl("mk_geodesic_polyline_new_json")
public func mk_geodesic_polyline_new_json(
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
                userInfo: [NSLocalizedDescriptionKey: "MKGeodesicPolyline requires at least one coordinate"]
            )
        }
        let polyline = coordinates.withUnsafeBufferPointer { buffer in
            MKGeodesicPolyline(coordinates: buffer.baseAddress!, count: buffer.count)
        }
        return mkrRetain(polyline)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_geodesic_polyline_state_json")
public func mk_geodesic_polyline_state_json(
    _ polyline: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let polyline else {
        mkrSetMessageError(outError, message: "missing MKGeodesicPolyline")
        return nil
    }

    do {
        let overlay = mkrBorrow(polyline, as: MKGeodesicPolyline.self)
        return mkrCString(try mkrEncodeJSON(mkrEncodeMultiPointState(overlay)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_geodesic_polyline_release")
public func mk_geodesic_polyline_release(_ polyline: UnsafeMutableRawPointer?) {
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

@_cdecl("mk_multi_polyline_new")
public func mk_multi_polyline_new(
    _ polylines: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        guard let polylines, count > 0 else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKMultiPolyline requires at least one MKPolyline"]
            )
        }
        let members = try UnsafeBufferPointer(start: polylines, count: count).map { pointer in
            guard let pointer else {
                throw NSError(
                    domain: "mapkit-rs",
                    code: -1,
                    userInfo: [NSLocalizedDescriptionKey: "missing MKPolyline"]
                )
            }
            return mkrBorrow(pointer, as: MKPolyline.self)
        }
        let overlay = MKMultiPolyline(members)
        return mkrRetain(overlay)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_multi_polyline_state_json")
public func mk_multi_polyline_state_json(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKMultiPolyline")
        return nil
    }

    do {
        let multiPolyline = mkrBorrow(overlay, as: MKMultiPolyline.self)
        let payload = MKRMultiPolylineStatePayload(
            coordinate: mkrEncodeCoordinate(multiPolyline.coordinate),
            boundingMapRect: mkrEncodeMapRect(multiPolyline.boundingMapRect),
            canReplaceMapContent: (multiPolyline as MKOverlay).canReplaceMapContent?() ?? false,
            polylineCount: multiPolyline.polylines.count,
            polylines: multiPolyline.polylines.map(mkrEncodePolylineCoordinates)
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_multi_polyline_release")
public func mk_multi_polyline_release(_ overlay: UnsafeMutableRawPointer?) {
    guard let overlay else { return }
    mkrRelease(overlay)
}

@_cdecl("mk_multi_polygon_new")
public func mk_multi_polygon_new(
    _ polygons: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        guard let polygons, count > 0 else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKMultiPolygon requires at least one MKPolygon"]
            )
        }
        let members = try UnsafeBufferPointer(start: polygons, count: count).map { pointer in
            guard let pointer else {
                throw NSError(
                    domain: "mapkit-rs",
                    code: -1,
                    userInfo: [NSLocalizedDescriptionKey: "missing MKPolygon"]
                )
            }
            return mkrBorrow(pointer, as: MKPolygon.self)
        }
        let overlay = MKMultiPolygon(members)
        return mkrRetain(overlay)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_multi_polygon_state_json")
public func mk_multi_polygon_state_json(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKMultiPolygon")
        return nil
    }

    do {
        let multiPolygon = mkrBorrow(overlay, as: MKMultiPolygon.self)
        let payload = MKRMultiPolygonStatePayload(
            coordinate: mkrEncodeCoordinate(multiPolygon.coordinate),
            boundingMapRect: mkrEncodeMapRect(multiPolygon.boundingMapRect),
            canReplaceMapContent: (multiPolygon as MKOverlay).canReplaceMapContent?() ?? false,
            polygonCount: multiPolygon.polygons.count,
            polygons: multiPolygon.polygons.map(mkrEncodePolygonCoordinates)
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_multi_polygon_release")
public func mk_multi_polygon_release(_ overlay: UnsafeMutableRawPointer?) {
    guard let overlay else { return }
    mkrRelease(overlay)
}

@_cdecl("mk_tile_overlay_new")
public func mk_tile_overlay_new(
    _ urlTemplate: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let overlay = MKTileOverlay(urlTemplate: urlTemplate.map(String.init(cString:)))
    return mkrRetain(overlay)
}

@_cdecl("mk_tile_overlay_state_json")
public func mk_tile_overlay_state_json(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKTileOverlay")
        return nil
    }

    do {
        let bridge = mkrBorrow(overlay, as: MKTileOverlay.self)
        let payload = mkrEncodeTileOverlayState(bridge)
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_tile_overlay_apply_options_json")
public func mk_tile_overlay_apply_options_json(
    _ overlay: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKTileOverlay")
        return
    }

    do {
        let bridge = mkrBorrow(overlay, as: MKTileOverlay.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRTileOverlayOptionsPayload.self)
        if let tileSize = payload.tileSize {
            bridge.tileSize = mkrScreenSize(from: tileSize)
        }
        if let geometryFlipped = payload.geometryFlipped {
            bridge.isGeometryFlipped = geometryFlipped
        }
        if let minimumZ = payload.minimumZ {
            bridge.minimumZ = minimumZ
        }
        if let maximumZ = payload.maximumZ {
            bridge.maximumZ = maximumZ
        }
        if let canReplaceMapContent = payload.canReplaceMapContent {
            bridge.canReplaceMapContent = canReplaceMapContent
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_tile_overlay_url_for_tile_path_json")
public func mk_tile_overlay_url_for_tile_path_json(
    _ overlay: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKTileOverlay")
        return nil
    }

    do {
        let bridge = mkrBorrow(overlay, as: MKTileOverlay.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRTileOverlayPathPayload.self)
        let path = MKTileOverlayPath(
            x: payload.x,
            y: payload.y,
            z: payload.z,
            contentScaleFactor: payload.contentScaleFactor
        )
        return mkrCString(try mkrEncodeJSON(bridge.url(forTilePath: path).absoluteString))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_tile_overlay_release")
public func mk_tile_overlay_release(_ overlay: UnsafeMutableRawPointer?) {
    guard let overlay else { return }
    mkrRelease(overlay)
}
