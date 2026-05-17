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

struct MKRMapItemAnnotationStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var title: String?
    var subtitle: String?
    var mapItem: MKRMapItemPayload
}

struct MKRUserLocationStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var title: String?
    var subtitle: String?
    var updating: Bool
    var location: MKRCoordinatePayload?
    var heading: Double?
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

@_cdecl("mk_map_item_annotation_new_json")
public func mk_map_item_annotation_new_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        guard #available(macOS 15.0, *) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKMapItemAnnotation requires macOS 15.0+"]
            )
        }
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMapItemPayload.self)
        guard let annotation = MKMapItemAnnotation(mapItem: try mkrBuildMapItem(payload)) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKMapItemAnnotation requires a map item with a valid coordinate"]
            )
        }
        return mkrRetain(annotation)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_item_annotation_state_json")
public func mk_map_item_annotation_state_json(
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let annotation else {
        mkrSetMessageError(outError, message: "missing MKMapItemAnnotation")
        return nil
    }

    do {
        guard #available(macOS 15.0, *) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "MKMapItemAnnotation requires macOS 15.0+"]
            )
        }
        let mapItemAnnotation = mkrBorrow(annotation, as: MKMapItemAnnotation.self)
        let payload = MKRMapItemAnnotationStatePayload(
            coordinate: mkrEncodeCoordinate(mapItemAnnotation.coordinate),
            title: mapItemAnnotation.title ?? nil,
            subtitle: mapItemAnnotation.subtitle ?? nil,
            mapItem: mkrEncodeMapItem(mapItemAnnotation.mapItem)
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_item_annotation_release")
public func mk_map_item_annotation_release(_ annotation: UnsafeMutableRawPointer?) {
    guard let annotation else { return }
    mkrRelease(annotation)
}

@_cdecl("mk_user_location_state_json")
public func mk_user_location_state_json(
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let annotation else {
        mkrSetMessageError(outError, message: "missing MKUserLocation")
        return nil
    }

    do {
        let userLocation = mkrBorrow(annotation, as: MKUserLocation.self)
        let payload = MKRUserLocationStatePayload(
            coordinate: mkrEncodeCoordinate(userLocation.coordinate),
            title: userLocation.title,
            subtitle: userLocation.subtitle,
            updating: userLocation.isUpdating,
            location: userLocation.location.map(\.coordinate).map(mkrEncodeCoordinate),
            heading: userLocation.heading?.trueHeading
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_user_location_release")
public func mk_user_location_release(_ annotation: UnsafeMutableRawPointer?) {
    guard let annotation else { return }
    mkrRelease(annotation)
}
