import AppKit
import Foundation
import MapKit

func mkrBorrowAnnotation(_ ptr: UnsafeMutableRawPointer?) throws -> (any MKAnnotation)? {
    guard let ptr else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue()
    guard let annotation = object as? any MKAnnotation else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "object does not conform to MKAnnotation"]
        )
    }
    return annotation
}

enum MKRAnnotationViewCollisionModePayload: String, Codable {
    case rectangle
    case circle
    case none
}

enum MKRAnnotationViewDragStatePayload: String, Codable {
    case none
    case starting
    case dragging
    case canceling
    case ending
}

struct MKRAnnotationViewStatePayload: Codable {
    var reuseIdentifier: String?
    var annotationTitle: String?
    var annotationSubtitle: String?
    var centerOffset: MKRScreenPointPayload
    var calloutOffset: MKRScreenPointPayload
    var leftCalloutOffset: MKRScreenPointPayload
    var rightCalloutOffset: MKRScreenPointPayload
    var enabled: Bool
    var highlighted: Bool
    var selected: Bool
    var canShowCallout: Bool
    var draggable: Bool
    var dragState: MKRAnnotationViewDragStatePayload
    var clusteringIdentifier: String?
    var displayPriority: Float
    var zPriority: Float
    var selectedZPriority: Float
    var collisionMode: MKRAnnotationViewCollisionModePayload
}

struct MKRAnnotationViewOptionsPayload: Codable {
    var centerOffset: MKRScreenPointPayload?
    var calloutOffset: MKRScreenPointPayload?
    var leftCalloutOffset: MKRScreenPointPayload?
    var rightCalloutOffset: MKRScreenPointPayload?
    var enabled: Bool?
    var highlighted: Bool?
    var selected: Bool?
    var selectedAnimated: Bool?
    var canShowCallout: Bool?
    var draggable: Bool?
    var dragState: MKRAnnotationViewDragStatePayload?
    var dragStateAnimated: Bool?
    var clusteringIdentifierPresent: Bool
    var clusteringIdentifier: String?
    var displayPriority: Float?
    var zPriority: Float?
    var selectedZPriority: Float?
    var collisionMode: MKRAnnotationViewCollisionModePayload?
}

struct MKRMarkerAnnotationViewStatePayload: Codable {
    var base: MKRAnnotationViewStatePayload
    var titleVisibility: MKRFeatureVisibilityPayload
    var subtitleVisibility: MKRFeatureVisibilityPayload
    var glyphText: String?
    var animatesWhenAdded: Bool
}

struct MKRMarkerAnnotationViewOptionsPayload: Codable {
    var titleVisibility: MKRFeatureVisibilityPayload?
    var subtitleVisibility: MKRFeatureVisibilityPayload?
    var glyphTextPresent: Bool
    var glyphText: String?
    var animatesWhenAdded: Bool?
}

private func mkrAnnotationViewDragState(
    from payload: MKRAnnotationViewDragStatePayload
) -> MKAnnotationView.DragState {
    switch payload {
    case .none: return .none
    case .starting: return .starting
    case .dragging: return .dragging
    case .canceling: return .canceling
    case .ending: return .ending
    }
}

private func mkrEncodeAnnotationViewDragState(
    _ state: MKAnnotationView.DragState
) -> MKRAnnotationViewDragStatePayload {
    switch state {
    case .none: return .none
    case .starting: return .starting
    case .dragging: return .dragging
    case .canceling: return .canceling
    case .ending: return .ending
    @unknown default: return .none
    }
}

private func mkrAnnotationViewCollisionMode(
    from payload: MKRAnnotationViewCollisionModePayload
) -> MKAnnotationView.CollisionMode {
    switch payload {
    case .rectangle: return .rectangle
    case .circle: return .circle
    case .none: return .none
    }
}

private func mkrEncodeAnnotationViewCollisionMode(
    _ mode: MKAnnotationView.CollisionMode
) -> MKRAnnotationViewCollisionModePayload {
    switch mode {
    case .rectangle: return .rectangle
    case .circle: return .circle
    case .none: return .none
    @unknown default: return .rectangle
    }
}

private func mkrEncodeAnnotationViewState(_ view: MKAnnotationView) -> MKRAnnotationViewStatePayload {
    let annotation = view.annotation
    return MKRAnnotationViewStatePayload(
        reuseIdentifier: view.reuseIdentifier,
        annotationTitle: annotation?.title ?? nil,
        annotationSubtitle: annotation?.subtitle ?? nil,
        centerOffset: mkrEncodeScreenPoint(view.centerOffset),
        calloutOffset: mkrEncodeScreenPoint(view.calloutOffset),
        leftCalloutOffset: mkrEncodeScreenPoint(view.leftCalloutOffset),
        rightCalloutOffset: mkrEncodeScreenPoint(view.rightCalloutOffset),
        enabled: view.isEnabled,
        highlighted: view.isHighlighted,
        selected: view.isSelected,
        canShowCallout: view.canShowCallout,
        draggable: view.isDraggable,
        dragState: mkrEncodeAnnotationViewDragState(view.dragState),
        clusteringIdentifier: view.clusteringIdentifier,
        displayPriority: view.displayPriority.rawValue,
        zPriority: view.zPriority.rawValue,
        selectedZPriority: view.selectedZPriority.rawValue,
        collisionMode: mkrEncodeAnnotationViewCollisionMode(view.collisionMode)
    )
}

@_cdecl("mk_annotation_callout_info_did_change_notification")
public func mk_annotation_callout_info_did_change_notification(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    mkrCString(NSNotification.Name.MKAnnotationCalloutInfoDidChange.rawValue)
}

@_cdecl("mk_annotation_view_new")
public func mk_annotation_view_new(
    _ annotation: UnsafeMutableRawPointer?,
    _ reuseIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let view = try mkrSyncOnMain {
            MKAnnotationView(
                annotation: try mkrBorrowAnnotation(annotation),
                reuseIdentifier: reuseIdentifier.map(String.init(cString:))
            )
        }
        return mkrRetain(view)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_annotation_view_state_json")
public func mk_annotation_view_state_json(
    _ annotationView: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let annotationView else {
        mkrSetMessageError(outError, message: "missing MKAnnotationView")
        return nil
    }

    do {
        let view = mkrBorrow(annotationView, as: MKAnnotationView.self)
        let payload = try mkrSyncOnMain { mkrEncodeAnnotationViewState(view) }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_annotation_view_apply_options_json")
public func mk_annotation_view_apply_options_json(
    _ annotationView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let annotationView else {
        mkrSetMessageError(outError, message: "missing MKAnnotationView")
        return
    }

    do {
        let view = mkrBorrow(annotationView, as: MKAnnotationView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRAnnotationViewOptionsPayload.self)
        try mkrSyncOnMain {
            if let centerOffset = payload.centerOffset {
                view.centerOffset = mkrScreenPoint(from: centerOffset)
            }
            if let calloutOffset = payload.calloutOffset {
                view.calloutOffset = mkrScreenPoint(from: calloutOffset)
            }
            if let leftCalloutOffset = payload.leftCalloutOffset {
                view.leftCalloutOffset = mkrScreenPoint(from: leftCalloutOffset)
            }
            if let rightCalloutOffset = payload.rightCalloutOffset {
                view.rightCalloutOffset = mkrScreenPoint(from: rightCalloutOffset)
            }
            if let enabled = payload.enabled {
                view.isEnabled = enabled
            }
            if let highlighted = payload.highlighted {
                view.isHighlighted = highlighted
            }
            if let selected = payload.selected {
                view.setSelected(selected, animated: payload.selectedAnimated ?? false)
            }
            if let canShowCallout = payload.canShowCallout {
                view.canShowCallout = canShowCallout
            }
            if let draggable = payload.draggable {
                view.isDraggable = draggable
            }
            if let dragState = payload.dragState {
                view.setDragState(
                    mkrAnnotationViewDragState(from: dragState),
                    animated: payload.dragStateAnimated ?? false
                )
            }
            if payload.clusteringIdentifierPresent {
                view.clusteringIdentifier = payload.clusteringIdentifier
            }
            if let displayPriority = payload.displayPriority {
                view.displayPriority = MKFeatureDisplayPriority(rawValue: displayPriority)
            }
            if let zPriority = payload.zPriority {
                view.zPriority = MKAnnotationViewZPriority(rawValue: zPriority)
            }
            if let selectedZPriority = payload.selectedZPriority {
                view.selectedZPriority = MKAnnotationViewZPriority(rawValue: selectedZPriority)
            }
            if let collisionMode = payload.collisionMode {
                view.collisionMode = mkrAnnotationViewCollisionMode(from: collisionMode)
            }
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_annotation_view_prepare_for_reuse")
public func mk_annotation_view_prepare_for_reuse(
    _ annotationView: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let annotationView else {
        mkrSetMessageError(outError, message: "missing MKAnnotationView")
        return
    }

    do {
        let view = mkrBorrow(annotationView, as: MKAnnotationView.self)
        try mkrSyncOnMain {
            view.prepareForReuse()
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_annotation_view_prepare_for_display")
public func mk_annotation_view_prepare_for_display(
    _ annotationView: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let annotationView else {
        mkrSetMessageError(outError, message: "missing MKAnnotationView")
        return
    }

    do {
        let view = mkrBorrow(annotationView, as: MKAnnotationView.self)
        try mkrSyncOnMain {
            view.prepareForDisplay()
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_annotation_view_release")
public func mk_annotation_view_release(_ annotationView: UnsafeMutableRawPointer?) {
    guard let annotationView else { return }
    mkrRelease(annotationView)
}

@_cdecl("mk_marker_annotation_view_new")
public func mk_marker_annotation_view_new(
    _ annotation: UnsafeMutableRawPointer?,
    _ reuseIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let view = try mkrSyncOnMain {
            MKMarkerAnnotationView(
                annotation: try mkrBorrowAnnotation(annotation),
                reuseIdentifier: reuseIdentifier.map(String.init(cString:))
            )
        }
        return mkrRetain(view)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_marker_annotation_view_state_json")
public func mk_marker_annotation_view_state_json(
    _ annotationView: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let annotationView else {
        mkrSetMessageError(outError, message: "missing MKMarkerAnnotationView")
        return nil
    }

    do {
        let view = mkrBorrow(annotationView, as: MKMarkerAnnotationView.self)
        let payload = try mkrSyncOnMain {
            MKRMarkerAnnotationViewStatePayload(
                base: mkrEncodeAnnotationViewState(view),
                titleVisibility: mkrEncodeFeatureVisibility(view.titleVisibility),
                subtitleVisibility: mkrEncodeFeatureVisibility(view.subtitleVisibility),
                glyphText: view.glyphText,
                animatesWhenAdded: view.animatesWhenAdded
            )
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_marker_annotation_view_apply_options_json")
public func mk_marker_annotation_view_apply_options_json(
    _ annotationView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let annotationView else {
        mkrSetMessageError(outError, message: "missing MKMarkerAnnotationView")
        return
    }

    do {
        let view = mkrBorrow(annotationView, as: MKMarkerAnnotationView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMarkerAnnotationViewOptionsPayload.self)
        try mkrSyncOnMain {
            if let titleVisibility = payload.titleVisibility {
                view.titleVisibility = mkrFeatureVisibility(from: titleVisibility)
            }
            if let subtitleVisibility = payload.subtitleVisibility {
                view.subtitleVisibility = mkrFeatureVisibility(from: subtitleVisibility)
            }
            if payload.glyphTextPresent {
                view.glyphText = payload.glyphText
            }
            if let animatesWhenAdded = payload.animatesWhenAdded {
                view.animatesWhenAdded = animatesWhenAdded
            }
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_marker_annotation_view_release")
public func mk_marker_annotation_view_release(_ annotationView: UnsafeMutableRawPointer?) {
    guard let annotationView else { return }
    mkrRelease(annotationView)
}
