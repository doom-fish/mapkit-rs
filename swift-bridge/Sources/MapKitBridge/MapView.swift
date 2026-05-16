import AppKit
import Foundation
import MapKit

enum MKRMapTypePayload: String, Codable {
    case standard
    case satellite
    case hybrid
    case satelliteFlyover
    case hybridFlyover
    case mutedStandard
}

enum MKRFeatureVisibilityPayload: String, Codable {
    case adaptive
    case hidden
    case visible
}

enum MKRUserTrackingModePayload: String, Codable {
    case none
    case follow
    case followWithHeading
}

enum MKROverlayLevelPayload: String, Codable {
    case aboveRoads
    case aboveLabels
}

struct MKRMapViewStatePayload: Codable {
    var mapType: MKRMapTypePayload
    var region: MKRCoordinateRegionPayload
    var centerCoordinate: MKRCoordinatePayload
    var visibleMapRect: MKRMapRectPayload
    var camera: MKRMapCameraPayload
    var cameraZoomRange: MKRMapCameraZoomRangePayload?
    var cameraBoundary: MKRMapCameraBoundaryPayload?
    var preferredConfiguration: MKRMapConfigurationPayload?
    var zoomEnabled: Bool
    var scrollEnabled: Bool
    var rotateEnabled: Bool
    var pitchEnabled: Bool
    var showsZoomControls: Bool
    var showsCompass: Bool
    var showsScale: Bool
    var showsPointsOfInterest: Bool
    var showsUserLocation: Bool
    var showsUserTrackingButton: Bool
    var pitchButtonVisibility: MKRFeatureVisibilityPayload?
    var userTrackingMode: MKRUserTrackingModePayload
    var annotationCount: Int
    var overlayCount: Int
}

struct MKRMapViewOptionsPayload: Codable {
    var mapType: MKRMapTypePayload?
    var region: MKRCoordinateRegionPayload?
    var centerCoordinate: MKRCoordinatePayload?
    var visibleMapRect: MKRMapRectPayload?
    var camera: MKRMapCameraPayload?
    var cameraZoomRangePresent: Bool
    var cameraZoomRange: MKRMapCameraZoomRangePayload?
    var cameraBoundaryPresent: Bool
    var cameraBoundary: MKRMapCameraBoundaryPayload?
    var preferredConfiguration: MKRMapConfigurationPayload?
    var zoomEnabled: Bool?
    var scrollEnabled: Bool?
    var rotateEnabled: Bool?
    var pitchEnabled: Bool?
    var showsZoomControls: Bool?
    var showsCompass: Bool?
    var showsScale: Bool?
    var showsPointsOfInterest: Bool?
    var showsUserLocation: Bool?
    var showsUserTrackingButton: Bool?
    var pointOfInterestFilterPresent: Bool
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
    var pitchButtonVisibility: MKRFeatureVisibilityPayload?
    var userTrackingMode: MKRUserTrackingModePayload?
    var animated: Bool?
}

func mkrMapType(from payload: MKRMapTypePayload) -> MKMapType {
    switch payload {
    case .standard: return .standard
    case .satellite: return .satellite
    case .hybrid: return .hybrid
    case .satelliteFlyover: return .satelliteFlyover
    case .hybridFlyover: return .hybridFlyover
    case .mutedStandard: return .mutedStandard
    }
}

func mkrEncodeMapType(_ mapType: MKMapType) -> MKRMapTypePayload {
    switch mapType {
    case .standard: return .standard
    case .satellite: return .satellite
    case .hybrid: return .hybrid
    case .satelliteFlyover: return .satelliteFlyover
    case .hybridFlyover: return .hybridFlyover
    case .mutedStandard: return .mutedStandard
    @unknown default: return .standard
    }
}

func mkrFeatureVisibility(from payload: MKRFeatureVisibilityPayload) -> MKFeatureVisibility {
    switch payload {
    case .adaptive: return .adaptive
    case .hidden: return .hidden
    case .visible: return .visible
    }
}

func mkrEncodeFeatureVisibility(_ visibility: MKFeatureVisibility) -> MKRFeatureVisibilityPayload {
    switch visibility {
    case .adaptive: return .adaptive
    case .hidden: return .hidden
    case .visible: return .visible
    @unknown default: return .adaptive
    }
}

func mkrUserTrackingMode(from payload: MKRUserTrackingModePayload) -> MKUserTrackingMode {
    switch payload {
    case .none: return .none
    case .follow: return .follow
    case .followWithHeading: return .follow
    }
}

func mkrEncodeUserTrackingMode(_ mode: MKUserTrackingMode) -> MKRUserTrackingModePayload {
    switch mode {
    case .none: return .none
    case .follow: return .follow
    case .followWithHeading: return .followWithHeading
    @unknown default: return .none
    }
}

func mkrOverlayLevel(from payload: MKROverlayLevelPayload) -> MKOverlayLevel {
    switch payload {
    case .aboveRoads: return .aboveRoads
    case .aboveLabels: return .aboveLabels
    }
}

private func mkrEncodeMapViewState(_ mapView: MKMapView) -> MKRMapViewStatePayload {
    let pitchButtonVisibility: MKRFeatureVisibilityPayload?
    if #available(macOS 14.0, *) {
        pitchButtonVisibility = mkrEncodeFeatureVisibility(mapView.pitchButtonVisibility)
    } else {
        pitchButtonVisibility = nil
    }

    let userTrackingMode: MKRUserTrackingModePayload
    if #available(macOS 11.0, *) {
        userTrackingMode = mkrEncodeUserTrackingMode(mapView.userTrackingMode)
    } else {
        userTrackingMode = .none
    }

    let showsUserTrackingButton: Bool
    if #available(macOS 14.0, *) {
        showsUserTrackingButton = mapView.showsUserTrackingButton
    } else {
        showsUserTrackingButton = false
    }

    return MKRMapViewStatePayload(
        mapType: mkrEncodeMapType(mapView.mapType),
        region: mkrEncodeRegion(mapView.region),
        centerCoordinate: mkrEncodeCoordinate(mapView.centerCoordinate),
        visibleMapRect: mkrEncodeMapRect(mapView.visibleMapRect),
        camera: mkrEncodeMapCamera(mapView.camera),
        cameraZoomRange: mkrEncodeMapCameraZoomRange(mapView.cameraZoomRange),
        cameraBoundary: mkrEncodeMapCameraBoundary(mapView.cameraBoundary),
        preferredConfiguration: mkrEncodeMapConfiguration(mapView.preferredConfiguration),
        zoomEnabled: mapView.isZoomEnabled,
        scrollEnabled: mapView.isScrollEnabled,
        rotateEnabled: mapView.isRotateEnabled,
        pitchEnabled: mapView.isPitchEnabled,
        showsZoomControls: mapView.showsZoomControls,
        showsCompass: mapView.showsCompass,
        showsScale: mapView.showsScale,
        showsPointsOfInterest: mapView.showsPointsOfInterest,
        showsUserLocation: mapView.showsUserLocation,
        showsUserTrackingButton: showsUserTrackingButton,
        pitchButtonVisibility: pitchButtonVisibility,
        userTrackingMode: userTrackingMode,
        annotationCount: mapView.annotations.count,
        overlayCount: mapView.overlays.count
    )
}

@_cdecl("mk_map_view_new")
public func mk_map_view_new(
    _ width: Double,
    _ height: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let mapView = try mkrSyncOnMain {
            MKMapView(frame: NSRect(x: 0, y: 0, width: width, height: height))
        }
        return mkrRetain(mapView)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_state_json")
public func mk_map_view_state_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return nil
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrSyncOnMain { mkrEncodeMapViewState(view) }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_apply_options_json")
public func mk_map_view_apply_options_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMapViewOptionsPayload.self)
        try mkrSyncOnMain {
            if let mapType = payload.mapType {
                view.mapType = mkrMapType(from: mapType)
            }
            if let region = payload.region {
                view.setRegion(mkrRegion(from: region), animated: payload.animated ?? false)
            }
            if let centerCoordinate = payload.centerCoordinate {
                view.setCenter(
                    mkrCoordinate(from: centerCoordinate),
                    animated: payload.animated ?? false
                )
            }
            if let visibleMapRect = payload.visibleMapRect {
                view.setVisibleMapRect(
                    mkrMapRect(from: visibleMapRect),
                    animated: payload.animated ?? false
                )
            }
            if let camera = payload.camera {
                view.setCamera(mkrBuildMapCamera(camera), animated: payload.animated ?? false)
            }
            if payload.cameraZoomRangePresent {
                view.setCameraZoomRange(
                    try payload.cameraZoomRange.map(mkrBuildMapCameraZoomRange),
                    animated: payload.animated ?? false
                )
            }
            if payload.cameraBoundaryPresent {
                view.setCameraBoundary(
                    try payload.cameraBoundary.map(mkrBuildMapCameraBoundary),
                    animated: payload.animated ?? false
                )
            }
            if let preferredConfiguration = payload.preferredConfiguration {
                view.preferredConfiguration = mkrBuildMapConfiguration(preferredConfiguration)
            }
            if let zoomEnabled = payload.zoomEnabled {
                view.isZoomEnabled = zoomEnabled
            }
            if let scrollEnabled = payload.scrollEnabled {
                view.isScrollEnabled = scrollEnabled
            }
            if let rotateEnabled = payload.rotateEnabled {
                view.isRotateEnabled = rotateEnabled
            }
            if let pitchEnabled = payload.pitchEnabled {
                view.isPitchEnabled = pitchEnabled
            }
            if let showsZoomControls = payload.showsZoomControls {
                view.showsZoomControls = showsZoomControls
            }
            if let showsCompass = payload.showsCompass {
                view.showsCompass = showsCompass
            }
            if let showsScale = payload.showsScale {
                view.showsScale = showsScale
            }
            if let showsPointsOfInterest = payload.showsPointsOfInterest {
                view.showsPointsOfInterest = showsPointsOfInterest
            }
            if let showsUserLocation = payload.showsUserLocation {
                view.showsUserLocation = showsUserLocation
            }
            if let showsUserTrackingButton = payload.showsUserTrackingButton {
                guard #available(macOS 14.0, *) else {
                    throw NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "showsUserTrackingButton requires macOS 14.0+"]
                    )
                }
                view.showsUserTrackingButton = showsUserTrackingButton
            }
            if payload.pointOfInterestFilterPresent {
                if #available(macOS 10.15, *) {
                    view.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
                } else if payload.pointOfInterestFilter != nil {
                    throw NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "pointOfInterestFilter requires macOS 10.15+"]
                    )
                }
            }
            if let pitchButtonVisibility = payload.pitchButtonVisibility {
                guard #available(macOS 14.0, *) else {
                    throw NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "pitchButtonVisibility requires macOS 14.0+"]
                    )
                }
                view.pitchButtonVisibility = mkrFeatureVisibility(from: pitchButtonVisibility)
            }
            if let userTrackingMode = payload.userTrackingMode {
                guard #available(macOS 11.0, *) else {
                    throw NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "userTrackingMode requires macOS 11.0+"]
                    )
                }
                view.setUserTrackingMode(
                    mkrUserTrackingMode(from: userTrackingMode),
                    animated: payload.animated ?? false
                )
            }
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_region_that_fits_json")
public func mk_map_view_region_that_fits_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return nil
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRCoordinateRegionPayload.self)
        let fitted = try mkrSyncOnMain {
            view.regionThatFits(mkrRegion(from: payload))
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeRegion(fitted)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_map_rect_that_fits_json")
public func mk_map_view_map_rect_that_fits_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return nil
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMapRectPayload.self)
        let fitted = try mkrSyncOnMain {
            view.mapRectThatFits(mkrMapRect(from: payload))
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeMapRect(fitted)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_convert_coordinate_to_point_json")
public func mk_map_view_convert_coordinate_to_point_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return nil
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRCoordinatePayload.self)
        let point = try mkrSyncOnMain {
            view.convert(mkrCoordinate(from: payload), toPointTo: nil)
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeScreenPoint(point)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_convert_point_to_coordinate_json")
public func mk_map_view_convert_point_to_coordinate_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return nil
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRScreenPointPayload.self)
        let coordinate = try mkrSyncOnMain {
            view.convert(mkrScreenPoint(from: payload), toCoordinateFrom: nil)
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeCoordinate(coordinate)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_add_point_annotation")
public func mk_map_view_add_point_annotation(
    _ mapView: UnsafeMutableRawPointer?,
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let annotation else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKPointAnnotation")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let pointAnnotation = mkrBorrow(annotation, as: MKPointAnnotation.self)
        try mkrSyncOnMain {
            view.addAnnotation(pointAnnotation)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_remove_point_annotation")
public func mk_map_view_remove_point_annotation(
    _ mapView: UnsafeMutableRawPointer?,
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let annotation else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKPointAnnotation")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let pointAnnotation = mkrBorrow(annotation, as: MKPointAnnotation.self)
        try mkrSyncOnMain {
            view.removeAnnotation(pointAnnotation)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_add_cluster_annotation")
public func mk_map_view_add_cluster_annotation(
    _ mapView: UnsafeMutableRawPointer?,
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let annotation else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKClusterAnnotation")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let clusterAnnotation = mkrBorrow(annotation, as: MKClusterAnnotation.self)
        try mkrSyncOnMain {
            view.addAnnotation(clusterAnnotation)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_remove_cluster_annotation")
public func mk_map_view_remove_cluster_annotation(
    _ mapView: UnsafeMutableRawPointer?,
    _ annotation: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let annotation else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKClusterAnnotation")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let clusterAnnotation = mkrBorrow(annotation, as: MKClusterAnnotation.self)
        try mkrSyncOnMain {
            view.removeAnnotation(clusterAnnotation)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

private func mkrDecodeOverlayLevel(_ levelJSON: UnsafePointer<CChar>?) throws -> MKOverlayLevel {
    let payload = try mkrDecodeJSON(levelJSON, as: MKROverlayLevelPayload.self)
    return mkrOverlayLevel(from: payload)
}

@_cdecl("mk_map_view_add_circle")
public func mk_map_view_add_circle(
    _ mapView: UnsafeMutableRawPointer?,
    _ circle: UnsafeMutableRawPointer?,
    _ levelJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let circle else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKCircle")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let overlay = mkrBorrow(circle, as: MKCircle.self)
        let level = try mkrDecodeOverlayLevel(levelJSON)
        try mkrSyncOnMain {
            view.addOverlay(overlay, level: level)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_remove_circle")
public func mk_map_view_remove_circle(
    _ mapView: UnsafeMutableRawPointer?,
    _ circle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let circle else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKCircle")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let overlay = mkrBorrow(circle, as: MKCircle.self)
        try mkrSyncOnMain {
            view.removeOverlay(overlay)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_add_polyline")
public func mk_map_view_add_polyline(
    _ mapView: UnsafeMutableRawPointer?,
    _ polyline: UnsafeMutableRawPointer?,
    _ levelJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let polyline else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKPolyline")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let overlay = mkrBorrow(polyline, as: MKPolyline.self)
        let level = try mkrDecodeOverlayLevel(levelJSON)
        try mkrSyncOnMain {
            view.addOverlay(overlay, level: level)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_remove_polyline")
public func mk_map_view_remove_polyline(
    _ mapView: UnsafeMutableRawPointer?,
    _ polyline: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let polyline else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKPolyline")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let overlay = mkrBorrow(polyline, as: MKPolyline.self)
        try mkrSyncOnMain {
            view.removeOverlay(overlay)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_add_polygon")
public func mk_map_view_add_polygon(
    _ mapView: UnsafeMutableRawPointer?,
    _ polygon: UnsafeMutableRawPointer?,
    _ levelJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let polygon else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKPolygon")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let overlay = mkrBorrow(polygon, as: MKPolygon.self)
        let level = try mkrDecodeOverlayLevel(levelJSON)
        try mkrSyncOnMain {
            view.addOverlay(overlay, level: level)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_remove_polygon")
public func mk_map_view_remove_polygon(
    _ mapView: UnsafeMutableRawPointer?,
    _ polygon: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView, let polygon else {
        mkrSetMessageError(outError, message: "missing MKMapView or MKPolygon")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let overlay = mkrBorrow(polygon, as: MKPolygon.self)
        try mkrSyncOnMain {
            view.removeOverlay(overlay)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_map_view_release")
public func mk_map_view_release(_ mapView: UnsafeMutableRawPointer?) {
    guard let mapView else { return }
    mkrRelease(mapView)
}
