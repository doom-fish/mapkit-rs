import AppKit
import Foundation
import MapKit

public typealias MKRContextCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

public typealias MKRMapViewEventCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int32,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Int64,
    Int64,
    UnsafeMutablePointer<CChar>?
) -> Void

public typealias MKRMapViewRendererCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Int32
) -> UnsafeMutableRawPointer?

enum MKRMapViewEvent: Int32 {
    case regionWillChange = 0
    case regionDidChange = 1
    case didChangeVisibleRegion = 2
    case willStartLoadingMap = 3
    case didFinishLoadingMap = 4
    case didFailLoadingMap = 5
    case willStartRenderingMap = 6
    case didFinishRenderingMap = 7
    case didSelectAnnotationView = 8
    case didDeselectAnnotationView = 9
    case willStartLocatingUser = 10
    case didStopLocatingUser = 11
    case didUpdateUserLocation = 12
    case didFailToLocateUser = 13
    case annotationViewDragStateChanged = 14
    case didChangeUserTrackingMode = 15
}

func mkrOverlayKind(_ overlay: MKOverlay) -> Int32 {
    switch overlay {
    case is MKCircle:
        return 0
    case is MKGeodesicPolyline:
        return 2
    case is MKPolyline:
        return 1
    case is MKPolygon:
        return 3
    case is MKMultiPolyline:
        return 4
    case is MKMultiPolygon:
        return 5
    case is MKTileOverlay:
        return 6
    default:
        return -1
    }
}

func mkrDragStateCode(_ state: MKAnnotationView.DragState) -> Int64 {
    switch state {
    case .none: return 0
    case .starting: return 1
    case .dragging: return 2
    case .canceling: return 3
    case .ending: return 4
    @unknown default: return -1
    }
}

func mkrUserTrackingModeCode(_ mode: MKUserTrackingMode) -> Int64 {
    switch mode {
    case .none: return 0
    case .follow: return 1
    case .followWithHeading: return 2
    @unknown default: return -1
    }
}

final class MKRMapViewDelegateProxy: NSObject, MKMapViewDelegate {
    private let context: UnsafeMutableRawPointer
    private let releaseContext: MKRContextCallback
    private let onEvent: MKRMapViewEventCallback
    private let onRenderer: MKRMapViewRendererCallback

    init(
        context: UnsafeMutableRawPointer,
        retainContext: MKRContextCallback,
        releaseContext: @escaping MKRContextCallback,
        onEvent: @escaping MKRMapViewEventCallback,
        onRenderer: @escaping MKRMapViewRendererCallback
    ) {
        retainContext(context)
        self.context = context
        self.releaseContext = releaseContext
        self.onEvent = onEvent
        self.onRenderer = onRenderer
    }

    deinit {
        releaseContext(context)
    }

    private func send(
        _ event: MKRMapViewEvent,
        _ mapView: MKMapView,
        object: AnyObject? = nil,
        first: Int64 = 0,
        second: Int64 = 0,
        error: Error? = nil
    ) {
        let errorJSON = error.flatMap { error in
            mkrCString((try? mkrEncodeJSON(mkrErrorPayload(from: error))) ?? error.localizedDescription)
        }
        onEvent(context, event.rawValue, mkrRetain(mapView), object.map { mkrRetain($0) }, first, second, errorJSON)
    }

    func mapView(_ mapView: MKMapView, regionWillChangeAnimated animated: Bool) {
        send(.regionWillChange, mapView, first: animated ? 1 : 0)
    }

    func mapView(_ mapView: MKMapView, regionDidChangeAnimated animated: Bool) {
        send(.regionDidChange, mapView, first: animated ? 1 : 0)
    }

    func mapViewDidChangeVisibleRegion(_ mapView: MKMapView) {
        send(.didChangeVisibleRegion, mapView)
    }

    func mapViewWillStartLoadingMap(_ mapView: MKMapView) {
        send(.willStartLoadingMap, mapView)
    }

    func mapViewDidFinishLoadingMap(_ mapView: MKMapView) {
        send(.didFinishLoadingMap, mapView)
    }

    func mapViewDidFailLoadingMap(_ mapView: MKMapView, withError error: Error) {
        send(.didFailLoadingMap, mapView, error: error)
    }

    func mapViewWillStartRenderingMap(_ mapView: MKMapView) {
        send(.willStartRenderingMap, mapView)
    }

    func mapViewDidFinishRenderingMap(_ mapView: MKMapView, fullyRendered: Bool) {
        send(.didFinishRenderingMap, mapView, first: fullyRendered ? 1 : 0)
    }

    func mapView(_ mapView: MKMapView, didSelect view: MKAnnotationView) {
        send(.didSelectAnnotationView, mapView, object: view)
    }

    func mapView(_ mapView: MKMapView, didDeselect view: MKAnnotationView) {
        send(.didDeselectAnnotationView, mapView, object: view)
    }

    func mapViewWillStartLocatingUser(_ mapView: MKMapView) {
        send(.willStartLocatingUser, mapView)
    }

    func mapViewDidStopLocatingUser(_ mapView: MKMapView) {
        send(.didStopLocatingUser, mapView)
    }

    func mapView(_ mapView: MKMapView, didUpdate userLocation: MKUserLocation) {
        send(.didUpdateUserLocation, mapView, object: userLocation)
    }

    func mapView(_ mapView: MKMapView, didFailToLocateUserWithError error: Error) {
        send(.didFailToLocateUser, mapView, error: error)
    }

    func mapView(
        _ mapView: MKMapView,
        annotationView view: MKAnnotationView,
        didChange newState: MKAnnotationView.DragState,
        fromOldState oldState: MKAnnotationView.DragState
    ) {
        send(
            .annotationViewDragStateChanged,
            mapView,
            object: view,
            first: mkrDragStateCode(newState),
            second: mkrDragStateCode(oldState)
        )
    }

    func mapView(_ mapView: MKMapView, didChange mode: MKUserTrackingMode, animated: Bool) {
        send(.didChangeUserTrackingMode, mapView, first: mkrUserTrackingModeCode(mode), second: animated ? 1 : 0)
    }

    func mapView(_ mapView: MKMapView, rendererFor overlay: MKOverlay) -> MKOverlayRenderer {
        let kind = mkrOverlayKind(overlay)
        if kind >= 0,
           let raw = onRenderer(context, mkrRetain(mapView), mkrRetain(overlay as AnyObject), kind),
           let renderer = Unmanaged<AnyObject>.fromOpaque(raw).takeRetainedValue() as? MKOverlayRenderer {
            return renderer
        }
        return MKOverlayRenderer(overlay: overlay)
    }
}

@_cdecl("mk_map_view_set_delegate")
public func mk_map_view_set_delegate(
    _ mapView: UnsafeMutableRawPointer?,
    _ context: UnsafeMutableRawPointer?,
    _ retainContext: MKRContextCallback?,
    _ releaseContext: MKRContextCallback?,
    _ onEvent: MKRMapViewEventCallback?,
    _ onRenderer: MKRMapViewRendererCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let mapView, let context, let retainContext, let releaseContext, let onEvent, let onRenderer else {
        mkrSetMessageError(outError, message: "missing MKMapView delegate arguments")
        return nil
    }

    do {
        return try mkrOnMainThread {
            let view = mkrBorrow(mapView, as: MKMapView.self)
            let proxy = MKRMapViewDelegateProxy(
                context: context,
                retainContext: retainContext,
                releaseContext: releaseContext,
                onEvent: onEvent,
                onRenderer: onRenderer
            )
            view.delegate = proxy
            return mkrRetain(proxy)
        }
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_view_clear_delegate")
public func mk_map_view_clear_delegate(_ mapView: UnsafeMutableRawPointer?, _ proxy: UnsafeMutableRawPointer?) {
    guard let mapView, let proxy else { return }
    let view = mkrBorrow(mapView, as: MKMapView.self)
    let delegate = mkrBorrow(proxy, as: MKRMapViewDelegateProxy.self)
    let clear = {
        if (view.delegate as AnyObject?) === delegate {
            view.delegate = nil
        }
    }
    if Thread.isMainThread {
        clear()
    } else {
        DispatchQueue.main.async(execute: clear)
    }
}

@_cdecl("mk_object_retain")
public func mk_object_retain(_ object: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object else { return nil }
    _ = Unmanaged<AnyObject>.fromOpaque(object).retain()
    return object
}

@_cdecl("mk_object_release")
public func mk_object_release(_ object: UnsafeMutableRawPointer?) {
    guard let object else { return }
    mkrRelease(object)
}
