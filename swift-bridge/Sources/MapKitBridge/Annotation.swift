import Foundation
import MapKit

struct MKRPointAnnotationStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var title: String?
    var subtitle: String?
}

struct MKRPointAnnotationOptionsPayload: Codable {
    var coordinate: MKRCoordinatePayload?
    var titlePresent: Bool
    var title: String?
    var subtitlePresent: Bool
    var subtitle: String?
}

@_cdecl("mk_point_annotation_new_json")
public func mk_point_annotation_new_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRPointAnnotationStatePayload.self)
        let annotation = MKPointAnnotation(
            coordinate: mkrCoordinate(from: payload.coordinate),
            title: payload.title,
            subtitle: payload.subtitle
        )
        return mkrRetain(annotation)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_point_annotation_state_json")
public func mk_point_annotation_state_json(
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let annotation else {
        mkrSetMessageError(outError, message: "missing MKPointAnnotation")
        return nil
    }

    do {
        let pointAnnotation = mkrBorrow(annotation, as: MKPointAnnotation.self)
        let payload = MKRPointAnnotationStatePayload(
            coordinate: mkrEncodeCoordinate(pointAnnotation.coordinate),
            title: pointAnnotation.title,
            subtitle: pointAnnotation.subtitle
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_point_annotation_apply_json")
public func mk_point_annotation_apply_json(
    _ annotation: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let annotation else {
        mkrSetMessageError(outError, message: "missing MKPointAnnotation")
        return
    }

    do {
        let pointAnnotation = mkrBorrow(annotation, as: MKPointAnnotation.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRPointAnnotationOptionsPayload.self)
        if let coordinate = payload.coordinate {
            pointAnnotation.coordinate = mkrCoordinate(from: coordinate)
        }
        if payload.titlePresent {
            pointAnnotation.title = payload.title
        }
        if payload.subtitlePresent {
            pointAnnotation.subtitle = payload.subtitle
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_point_annotation_release")
public func mk_point_annotation_release(_ annotation: UnsafeMutableRawPointer?) {
    guard let annotation else { return }
    mkrRelease(annotation)
}
