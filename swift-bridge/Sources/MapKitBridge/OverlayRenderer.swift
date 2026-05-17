import Foundation
import MapKit

struct MKROverlayRendererStatePayload: Codable {
    var alpha: Double
    var contentScaleFactor: Double
}

struct MKROverlayRendererOptionsPayload: Codable {
    var alpha: Double?
}

struct MKROverlayPathRendererStatePayload: Codable {
    var base: MKROverlayRendererStatePayload
    var lineWidth: Double
    var lineDashPhase: Double
    var lineDashPattern: [Double]?
    var shouldRasterize: Bool
}

struct MKROverlayPathRendererOptionsPayload: Codable {
    var lineWidth: Double?
    var lineDashPhase: Double?
    var lineDashPatternPresent: Bool
    var lineDashPattern: [Double]?
    var shouldRasterize: Bool?
}

struct MKRStrokeRendererStatePayload: Codable {
    var base: MKROverlayPathRendererStatePayload
    var strokeStart: Double
    var strokeEnd: Double
}

struct MKRStrokeRendererOptionsPayload: Codable {
    var strokeStart: Double?
    var strokeEnd: Double?
}

struct MKRMapRectZoomScalePayload: Codable {
    var mapRect: MKRMapRectPayload
    var zoomScale: Double
}

private func mkrEncodeOverlayRendererState(_ renderer: MKOverlayRenderer) -> MKROverlayRendererStatePayload {
    MKROverlayRendererStatePayload(
        alpha: renderer.alpha,
        contentScaleFactor: renderer.contentScaleFactor
    )
}

private func mkrEncodeOverlayPathRendererState(
    _ renderer: MKOverlayPathRenderer
) -> MKROverlayPathRendererStatePayload {
    MKROverlayPathRendererStatePayload(
        base: mkrEncodeOverlayRendererState(renderer),
        lineWidth: renderer.lineWidth,
        lineDashPhase: renderer.lineDashPhase,
        lineDashPattern: renderer.lineDashPattern?.map(\.doubleValue),
        shouldRasterize: renderer.shouldRasterize
    )
}

private func mkrEncodeStrokeRendererState<T: MKOverlayPathRenderer>(
    _ renderer: T,
    strokeStart: CGFloat,
    strokeEnd: CGFloat
) -> MKRStrokeRendererStatePayload {
    MKRStrokeRendererStatePayload(
        base: mkrEncodeOverlayPathRendererState(renderer),
        strokeStart: strokeStart,
        strokeEnd: strokeEnd
    )
}

@_cdecl("mk_road_width_at_zoom_scale")
public func mk_road_width_at_zoom_scale(_ zoomScale: Double) -> Double {
    MKRoadWidthAtZoomScale(zoomScale)
}

@_cdecl("mk_overlay_renderer_new")
public func mk_overlay_renderer_new(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKOverlay")
        return nil
    }

    do {
        let renderer = try mkrSyncOnMain {
            MKOverlayRenderer(overlay: try mkrBorrowOverlay(overlay))
        }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_overlay_renderer_state_json")
public func mk_overlay_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
        let payload = try mkrSyncOnMain { mkrEncodeOverlayRendererState(bridge) }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_overlay_renderer_apply_options_json")
public func mk_overlay_renderer_apply_options_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKROverlayRendererOptionsPayload.self)
        try mkrSyncOnMain {
            if let alpha = payload.alpha {
                bridge.alpha = alpha
            }
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_overlay_renderer_can_draw_map_rect_json")
public func mk_overlay_renderer_can_draw_map_rect_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayRenderer")
        return false
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMapRectZoomScalePayload.self)
        return try mkrSyncOnMain {
            bridge.canDraw(mkrMapRect(from: payload.mapRect), zoomScale: payload.zoomScale)
        }
    } catch {
        mkrSetError(outError, error)
        return false
    }
}

@_cdecl("mk_overlay_renderer_set_needs_display")
public func mk_overlay_renderer_set_needs_display(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
        try mkrSyncOnMain {
            bridge.setNeedsDisplay()
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_overlay_renderer_set_needs_display_in_map_rect_json")
public func mk_overlay_renderer_set_needs_display_in_map_rect_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMapRectPayload.self)
        try mkrSyncOnMain {
            bridge.setNeedsDisplay(mkrMapRect(from: payload))
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_overlay_renderer_set_needs_display_in_map_rect_zoom_scale_json")
public func mk_overlay_renderer_set_needs_display_in_map_rect_zoom_scale_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRMapRectZoomScalePayload.self)
        try mkrSyncOnMain {
            bridge.setNeedsDisplay(mkrMapRect(from: payload.mapRect), zoomScale: payload.zoomScale)
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_overlay_renderer_release")
public func mk_overlay_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_overlay_path_renderer_new")
public func mk_overlay_path_renderer_new(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKOverlay")
        return nil
    }

    do {
        let renderer = try mkrSyncOnMain {
            MKOverlayPathRenderer(overlay: try mkrBorrowOverlay(overlay))
        }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_overlay_path_renderer_state_json")
public func mk_overlay_path_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayPathRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayPathRenderer.self)
        let payload = try mkrSyncOnMain { mkrEncodeOverlayPathRendererState(bridge) }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_overlay_path_renderer_apply_options_json")
public func mk_overlay_path_renderer_apply_options_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKOverlayPathRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKOverlayPathRenderer.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKROverlayPathRendererOptionsPayload.self)
        try mkrSyncOnMain {
            if let lineWidth = payload.lineWidth {
                bridge.lineWidth = lineWidth
            }
            if let lineDashPhase = payload.lineDashPhase {
                bridge.lineDashPhase = lineDashPhase
            }
            if payload.lineDashPatternPresent {
                bridge.lineDashPattern = payload.lineDashPattern?.map(NSNumber.init(value:))
            }
            if let shouldRasterize = payload.shouldRasterize {
                bridge.shouldRasterize = shouldRasterize
            }
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_overlay_path_renderer_release")
public func mk_overlay_path_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

private func mkrBuildStrokeRendererOptions(
    _ payloadJSON: UnsafePointer<CChar>?
) throws -> MKRStrokeRendererOptionsPayload {
    try mkrDecodeJSON(payloadJSON, as: MKRStrokeRendererOptionsPayload.self)
}

private func mkrApplyStrokeRendererOptions<T: AnyObject>(
    _ renderer: T,
    _ payload: MKRStrokeRendererOptionsPayload,
    setStart: (T, Double) -> Void,
    setEnd: (T, Double) -> Void
) {
    if let strokeStart = payload.strokeStart {
        setStart(renderer, strokeStart)
    }
    if let strokeEnd = payload.strokeEnd {
        setEnd(renderer, strokeEnd)
    }
}

@_cdecl("mk_circle_renderer_new")
public func mk_circle_renderer_new(
    _ circle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let circle else {
        mkrSetMessageError(outError, message: "missing MKCircle")
        return nil
    }

    do {
        let overlay = mkrBorrow(circle, as: MKCircle.self)
        let renderer = try mkrSyncOnMain { MKCircleRenderer(circle: overlay) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_circle_renderer_state_json")
public func mk_circle_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKCircleRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKCircleRenderer.self)
        let payload = try mkrSyncOnMain {
            mkrEncodeStrokeRendererState(bridge, strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_circle_renderer_apply_options_json")
public func mk_circle_renderer_apply_options_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKCircleRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKCircleRenderer.self)
        let payload = try mkrBuildStrokeRendererOptions(payloadJSON)
        try mkrSyncOnMain {
            mkrApplyStrokeRendererOptions(
                bridge,
                payload,
                setStart: { $0.strokeStart = $1 },
                setEnd: { $0.strokeEnd = $1 }
            )
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_circle_renderer_release")
public func mk_circle_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_polyline_renderer_new")
public func mk_polyline_renderer_new(
    _ polyline: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let polyline else {
        mkrSetMessageError(outError, message: "missing MKPolyline")
        return nil
    }

    do {
        let overlay = mkrBorrow(polyline, as: MKPolyline.self)
        let renderer = try mkrSyncOnMain { MKPolylineRenderer(polyline: overlay) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polyline_renderer_state_json")
public func mk_polyline_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKPolylineRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKPolylineRenderer.self)
        let payload = try mkrSyncOnMain {
            mkrEncodeStrokeRendererState(bridge, strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polyline_renderer_apply_options_json")
public func mk_polyline_renderer_apply_options_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKPolylineRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKPolylineRenderer.self)
        let payload = try mkrBuildStrokeRendererOptions(payloadJSON)
        try mkrSyncOnMain {
            mkrApplyStrokeRendererOptions(
                bridge,
                payload,
                setStart: { $0.strokeStart = $1 },
                setEnd: { $0.strokeEnd = $1 }
            )
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_polyline_renderer_release")
public func mk_polyline_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_gradient_polyline_renderer_new")
public func mk_gradient_polyline_renderer_new(
    _ polyline: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let polyline else {
        mkrSetMessageError(outError, message: "missing MKPolyline")
        return nil
    }

    do {
        let overlay = mkrBorrow(polyline, as: MKPolyline.self)
        let renderer = try mkrSyncOnMain { MKGradientPolylineRenderer(polyline: overlay) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_gradient_polyline_renderer_state_json")
public func mk_gradient_polyline_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKGradientPolylineRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKGradientPolylineRenderer.self)
        let payload = try mkrSyncOnMain {
            mkrEncodeStrokeRendererState(bridge, strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_gradient_polyline_renderer_apply_options_json")
public func mk_gradient_polyline_renderer_apply_options_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKGradientPolylineRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKGradientPolylineRenderer.self)
        let payload = try mkrBuildStrokeRendererOptions(payloadJSON)
        try mkrSyncOnMain {
            mkrApplyStrokeRendererOptions(
                bridge,
                payload,
                setStart: { $0.strokeStart = $1 },
                setEnd: { $0.strokeEnd = $1 }
            )
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_gradient_polyline_renderer_release")
public func mk_gradient_polyline_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_polygon_renderer_new")
public func mk_polygon_renderer_new(
    _ polygon: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let polygon else {
        mkrSetMessageError(outError, message: "missing MKPolygon")
        return nil
    }

    do {
        let overlay = mkrBorrow(polygon, as: MKPolygon.self)
        let renderer = try mkrSyncOnMain { MKPolygonRenderer(polygon: overlay) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polygon_renderer_state_json")
public func mk_polygon_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKPolygonRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKPolygonRenderer.self)
        let payload = try mkrSyncOnMain {
            mkrEncodeStrokeRendererState(bridge, strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_polygon_renderer_apply_options_json")
public func mk_polygon_renderer_apply_options_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKPolygonRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKPolygonRenderer.self)
        let payload = try mkrBuildStrokeRendererOptions(payloadJSON)
        try mkrSyncOnMain {
            mkrApplyStrokeRendererOptions(
                bridge,
                payload,
                setStart: { $0.strokeStart = $1 },
                setEnd: { $0.strokeEnd = $1 }
            )
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_polygon_renderer_release")
public func mk_polygon_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_tile_overlay_renderer_new")
public func mk_tile_overlay_renderer_new(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKTileOverlay")
        return nil
    }

    do {
        let bridge = mkrBorrow(overlay, as: MKTileOverlay.self)
        let renderer = try mkrSyncOnMain { MKTileOverlayRenderer(tileOverlay: bridge) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_tile_overlay_renderer_state_json")
public func mk_tile_overlay_renderer_state_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKTileOverlayRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKTileOverlayRenderer.self)
        let payload = try mkrSyncOnMain { mkrEncodeOverlayRendererState(bridge) }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_tile_overlay_renderer_reload_data")
public func mk_tile_overlay_renderer_reload_data(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKTileOverlayRenderer")
        return
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKTileOverlayRenderer.self)
        try mkrSyncOnMain {
            bridge.reloadData()
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_tile_overlay_renderer_release")
public func mk_tile_overlay_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_multi_polyline_renderer_new")
public func mk_multi_polyline_renderer_new(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKMultiPolyline")
        return nil
    }

    do {
        let multiPolyline = mkrBorrow(overlay, as: MKMultiPolyline.self)
        let renderer = try mkrSyncOnMain { MKMultiPolylineRenderer(multiPolyline: multiPolyline) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_multi_polygon_renderer_new")
public func mk_multi_polygon_renderer_new(
    _ overlay: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let overlay else {
        mkrSetMessageError(outError, message: "missing MKMultiPolygon")
        return nil
    }

    do {
        let multiPolygon = mkrBorrow(overlay, as: MKMultiPolygon.self)
        let renderer = try mkrSyncOnMain { MKMultiPolygonRenderer(multiPolygon: multiPolygon) }
        return mkrRetain(renderer)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}
