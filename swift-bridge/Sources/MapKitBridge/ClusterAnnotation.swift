import Foundation
import MapKit

struct MKRClusterAnnotationStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var title: String?
    var subtitle: String?
    var memberCount: Int
}

struct MKRClusterAnnotationOptionsPayload: Codable {
    var titlePresent: Bool
    var title: String?
    var subtitlePresent: Bool
    var subtitle: String?
}

@_cdecl("mk_cluster_annotation_new")
public func mk_cluster_annotation_new(
    _ annotations: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let annotations, count > 0 else {
        mkrSetMessageError(outError, message: "MKClusterAnnotation requires at least one member annotation")
        return nil
    }

    do {
        let members = UnsafeBufferPointer(start: annotations, count: count).compactMap { pointer in
            pointer.map { mkrBorrow($0, as: MKPointAnnotation.self) }
        }
        let annotation = MKClusterAnnotation(memberAnnotations: members)
        return mkrRetain(annotation)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_cluster_annotation_state_json")
public func mk_cluster_annotation_state_json(
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let annotation else {
        mkrSetMessageError(outError, message: "missing MKClusterAnnotation")
        return nil
    }

    do {
        let cluster = mkrBorrow(annotation, as: MKClusterAnnotation.self)
        let payload = MKRClusterAnnotationStatePayload(
            coordinate: mkrEncodeCoordinate(cluster.coordinate),
            title: cluster.title,
            subtitle: cluster.subtitle,
            memberCount: cluster.memberAnnotations.count
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_cluster_annotation_apply_json")
public func mk_cluster_annotation_apply_json(
    _ annotation: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let annotation else {
        mkrSetMessageError(outError, message: "missing MKClusterAnnotation")
        return
    }

    do {
        let cluster = mkrBorrow(annotation, as: MKClusterAnnotation.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRClusterAnnotationOptionsPayload.self)
        if payload.titlePresent {
            cluster.title = payload.title
        }
        if payload.subtitlePresent {
            cluster.subtitle = payload.subtitle
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_cluster_annotation_release")
public func mk_cluster_annotation_release(_ annotation: UnsafeMutableRawPointer?) {
    guard let annotation else { return }
    mkrRelease(annotation)
}
