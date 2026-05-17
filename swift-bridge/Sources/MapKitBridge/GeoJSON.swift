import Foundation
import MapKit

struct MKRGeoJSONPointAnnotationPayload: Codable {
    var coordinate: MKRCoordinatePayload
    var title: String?
    var subtitle: String?
}

struct MKRGeoJSONPolylinePayload: Codable {
    var coordinates: [MKRCoordinatePayload]
}

struct MKRGeoJSONPolygonPayload: Codable {
    var coordinates: [MKRCoordinatePayload]
}

struct MKRGeoJSONMultiPolylinePayload: Codable {
    var polylines: [[MKRCoordinatePayload]]
}

struct MKRGeoJSONMultiPolygonPayload: Codable {
    var polygons: [[MKRCoordinatePayload]]
}

struct MKRGeoJSONFeaturePayload: Codable {
    var identifier: String?
    var properties: String?
    var geometry: [MKRGeoJSONObjectPayload]
}

struct MKRGeoJSONObjectPayload: Codable {
    var kind: String
    var feature: MKRGeoJSONFeaturePayload?
    var pointAnnotation: MKRGeoJSONPointAnnotationPayload?
    var polyline: MKRGeoJSONPolylinePayload?
    var polygon: MKRGeoJSONPolygonPayload?
    var multiPolyline: MKRGeoJSONMultiPolylinePayload?
    var multiPolygon: MKRGeoJSONMultiPolygonPayload?
}

private func mkrGeoJSONCoordinates(from multiPoint: MKMultiPoint) -> [MKRCoordinatePayload] {
    let pointCount = multiPoint.pointCount
    var coordinates = Array(
        repeating: CLLocationCoordinate2D(latitude: 0, longitude: 0),
        count: pointCount
    )
    if pointCount > 0 {
        multiPoint.getCoordinates(&coordinates, range: NSRange(location: 0, length: pointCount))
    }
    return mkrEncodeCoordinates(coordinates)
}

private func mkrEncodeGeoJSONObject(_ object: any MKGeoJSONObject) throws -> MKRGeoJSONObjectPayload {
    switch object {
    case let feature as MKGeoJSONFeature:
        let properties = feature.properties.map { String(decoding: $0, as: UTF8.self) }
        return MKRGeoJSONObjectPayload(
            kind: "feature",
            feature: MKRGeoJSONFeaturePayload(
                identifier: feature.identifier,
                properties: properties,
                geometry: try feature.geometry.map(mkrEncodeGeoJSONObject)
            ),
            pointAnnotation: nil,
            polyline: nil,
            polygon: nil,
            multiPolyline: nil,
            multiPolygon: nil
        )
    case let pointAnnotation as MKPointAnnotation:
        return MKRGeoJSONObjectPayload(
            kind: "pointAnnotation",
            feature: nil,
            pointAnnotation: MKRGeoJSONPointAnnotationPayload(
                coordinate: mkrEncodeCoordinate(pointAnnotation.coordinate),
                title: pointAnnotation.title ?? nil,
                subtitle: pointAnnotation.subtitle ?? nil
            ),
            polyline: nil,
            polygon: nil,
            multiPolyline: nil,
            multiPolygon: nil
        )
    case let multiPolyline as MKMultiPolyline:
        return MKRGeoJSONObjectPayload(
            kind: "multiPolyline",
            feature: nil,
            pointAnnotation: nil,
            polyline: nil,
            polygon: nil,
            multiPolyline: MKRGeoJSONMultiPolylinePayload(
                polylines: multiPolyline.polylines.map { mkrGeoJSONCoordinates(from: $0) }
            ),
            multiPolygon: nil
        )
    case let multiPolygon as MKMultiPolygon:
        return MKRGeoJSONObjectPayload(
            kind: "multiPolygon",
            feature: nil,
            pointAnnotation: nil,
            polyline: nil,
            polygon: nil,
            multiPolyline: nil,
            multiPolygon: MKRGeoJSONMultiPolygonPayload(
                polygons: multiPolygon.polygons.map { mkrGeoJSONCoordinates(from: $0) }
            )
        )
    case let polyline as MKPolyline:
        return MKRGeoJSONObjectPayload(
            kind: "polyline",
            feature: nil,
            pointAnnotation: nil,
            polyline: MKRGeoJSONPolylinePayload(coordinates: mkrGeoJSONCoordinates(from: polyline)),
            polygon: nil,
            multiPolyline: nil,
            multiPolygon: nil
        )
    case let polygon as MKPolygon:
        return MKRGeoJSONObjectPayload(
            kind: "polygon",
            feature: nil,
            pointAnnotation: nil,
            polyline: nil,
            polygon: MKRGeoJSONPolygonPayload(coordinates: mkrGeoJSONCoordinates(from: polygon)),
            multiPolyline: nil,
            multiPolygon: nil
        )
    default:
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "unsupported MKGeoJSONObject type: \(type(of: object))"]
        )
    }
}

@_cdecl("mk_geojson_decode_json")
public func mk_geojson_decode_json(
    _ data: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payloadData: Data
        if let data {
            payloadData = Data(bytes: data, count: len)
        } else {
            payloadData = Data()
        }
        let objects = try MKGeoJSONDecoder().decode(payloadData)
        let payload = try objects.map(mkrEncodeGeoJSONObject)
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}
