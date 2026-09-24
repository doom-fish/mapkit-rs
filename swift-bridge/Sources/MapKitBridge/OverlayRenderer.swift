import Foundation
import MapKit

struct MKROverlayRendererStatePayload: Codable {
    var alpha: Double
    var contentScaleFactor: Double
}

struct MKROverlayRendererOptionsPayload: Codable {
    var alpha: Double?
}

struct MKRColorPayload: Codable {
    var red: Double
    var green: Double
    var blue: Double
    var alpha: Double
}

enum MKRLineCapPayload: String, Codable {
    case butt
    case round
    case square
}

enum MKRLineJoinPayload: String, Codable {
    case miter
    case round
    case bevel
}

struct MKROverlayPathRendererStatePayload: Codable {
    var base: MKROverlayRendererStatePayload
    var strokeColor: MKRColorPayload?
    var fillColor: MKRColorPayload?
    var lineWidth: Double
    var lineCap: MKRLineCapPayload
    var lineJoin: MKRLineJoinPayload
    var miterLimit: Double
    var lineDashPhase: Double
    var lineDashPattern: [Double]?
    var shouldRasterize: Bool
}

struct MKROverlayPathRendererOptionsPayload: Codable {
    var strokeColorPresent: Bool
    var strokeColor: MKRColorPayload?
    var fillColorPresent: Bool
    var fillColor: MKRColorPayload?
    var lineWidth: Double?
    var lineCap: MKRLineCapPayload?
    var lineJoin: MKRLineJoinPayload?
    var miterLimit: Double?
    var lineDashPhase: Double?
    var lineDashPatternPresent: Bool
    var lineDashPattern: [Double]?
    var shouldRasterize: Bool?
}

struct MKRStrokeRendererStatePayload: Codable {
    var strokeStart: Double
    var strokeEnd: Double
}

struct MKRStrokeRendererOptionsPayload: Codable {
    var strokeStart: Double?
    var strokeEnd: Double?
}

struct MKRGradientPolylineColorsPayload: Codable {
    var colors: [MKRColorPayload]
    var locations: [Double]
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

private func mkrEncodeColor(_ color: NSColor) throws -> MKRColorPayload {
    guard let srgb = color.usingColorSpace(.sRGB) else {
        throw mkrBridgeError("MapKit returned a color that has no sRGB representation")
    }
    return MKRColorPayload(
        red: srgb.redComponent,
        green: srgb.greenComponent,
        blue: srgb.blueComponent,
        alpha: srgb.alphaComponent
    )
}

private func mkrColor(from payload: MKRColorPayload) -> NSColor {
    NSColor(srgbRed: payload.red, green: payload.green, blue: payload.blue, alpha: payload.alpha)
}

private func mkrEncodeLineCap(_ lineCap: CGLineCap) throws -> MKRLineCapPayload {
    switch lineCap {
    case .butt: return .butt
    case .round: return .round
    case .square: return .square
    @unknown default:
        throw mkrBridgeError("MapKit returned unknown line cap \(lineCap.rawValue)")
    }
}

private func mkrLineCap(from payload: MKRLineCapPayload) -> CGLineCap {
    switch payload {
    case .butt: return .butt
    case .round: return .round
    case .square: return .square
    }
}

private func mkrEncodeLineJoin(_ lineJoin: CGLineJoin) throws -> MKRLineJoinPayload {
    switch lineJoin {
    case .miter: return .miter
    case .round: return .round
    case .bevel: return .bevel
    @unknown default:
        throw mkrBridgeError("MapKit returned unknown line join \(lineJoin.rawValue)")
    }
}

private func mkrLineJoin(from payload: MKRLineJoinPayload) -> CGLineJoin {
    switch payload {
    case .miter: return .miter
    case .round: return .round
    case .bevel: return .bevel
    }
}

private func mkrEncodeOverlayPathRendererState(
    _ renderer: MKOverlayPathRenderer
) throws -> MKROverlayPathRendererStatePayload {
    MKROverlayPathRendererStatePayload(
        base: mkrEncodeOverlayRendererState(renderer),
        strokeColor: try renderer.strokeColor.map(mkrEncodeColor),
        fillColor: try renderer.fillColor.map(mkrEncodeColor),
        lineWidth: renderer.lineWidth,
        lineCap: try mkrEncodeLineCap(renderer.lineCap),
        lineJoin: try mkrEncodeLineJoin(renderer.lineJoin),
        miterLimit: renderer.miterLimit,
        lineDashPhase: renderer.lineDashPhase,
        lineDashPattern: renderer.lineDashPattern?.map(\.doubleValue),
        shouldRasterize: renderer.shouldRasterize
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
        let renderer = MKOverlayRenderer(overlay: try mkrBorrowOverlay(overlay))
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
        let payload = mkrEncodeOverlayRendererState(bridge)
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
        if let alpha = payload.alpha {
            bridge.alpha = alpha
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
        return bridge.canDraw(mkrMapRect(from: payload.mapRect), zoomScale: payload.zoomScale)
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

    let bridge = mkrBorrow(renderer, as: MKOverlayRenderer.self)
    bridge.setNeedsDisplay()
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
        bridge.setNeedsDisplay(mkrMapRect(from: payload))
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
        bridge.setNeedsDisplay(mkrMapRect(from: payload.mapRect), zoomScale: payload.zoomScale)
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
        let renderer = MKOverlayPathRenderer(overlay: try mkrBorrowOverlay(overlay))
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
        let payload = try mkrEncodeOverlayPathRendererState(bridge)
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
        if payload.strokeColorPresent {
            bridge.strokeColor = payload.strokeColor.map(mkrColor(from:))
        }
        if payload.fillColorPresent {
            bridge.fillColor = payload.fillColor.map(mkrColor(from:))
        }
        if let lineWidth = payload.lineWidth {
            bridge.lineWidth = lineWidth
        }
        if let lineCap = payload.lineCap {
            bridge.lineCap = mkrLineCap(from: lineCap)
        }
        if let lineJoin = payload.lineJoin {
            bridge.lineJoin = mkrLineJoin(from: lineJoin)
        }
        if let miterLimit = payload.miterLimit {
            bridge.miterLimit = miterLimit
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

    let overlay = mkrBorrow(circle, as: MKCircle.self)
    let renderer = MKCircleRenderer(circle: overlay)
    return mkrRetain(renderer)
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
        let payload = MKRStrokeRendererStatePayload(strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
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
        mkrApplyStrokeRendererOptions(
            bridge,
            payload,
            setStart: { $0.strokeStart = $1 },
            setEnd: { $0.strokeEnd = $1 }
        )
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

    let overlay = mkrBorrow(polyline, as: MKPolyline.self)
    let renderer = MKPolylineRenderer(polyline: overlay)
    return mkrRetain(renderer)
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
        let payload = MKRStrokeRendererStatePayload(strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
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
        mkrApplyStrokeRendererOptions(
            bridge,
            payload,
            setStart: { $0.strokeStart = $1 },
            setEnd: { $0.strokeEnd = $1 }
        )
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

    let overlay = mkrBorrow(polyline, as: MKPolyline.self)
    let renderer = MKGradientPolylineRenderer(polyline: overlay)
    return mkrRetain(renderer)
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
        let payload = MKRStrokeRendererStatePayload(strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
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
        mkrApplyStrokeRendererOptions(
            bridge,
            payload,
            setStart: { $0.strokeStart = $1 },
            setEnd: { $0.strokeEnd = $1 }
        )
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_gradient_polyline_renderer_release")
public func mk_gradient_polyline_renderer_release(_ renderer: UnsafeMutableRawPointer?) {
    guard let renderer else { return }
    mkrRelease(renderer)
}

@_cdecl("mk_gradient_polyline_renderer_colors_json")
public func mk_gradient_polyline_renderer_colors_json(
    _ renderer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let renderer else {
        mkrSetMessageError(outError, message: "missing MKGradientPolylineRenderer")
        return nil
    }

    do {
        let bridge = mkrBorrow(renderer, as: MKGradientPolylineRenderer.self)
        let payload = MKRGradientPolylineColorsPayload(
            colors: try bridge.colors.map(mkrEncodeColor),
            locations: bridge.locations.map { Double($0) }
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_gradient_polyline_renderer_set_colors_json")
public func mk_gradient_polyline_renderer_set_colors_json(
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
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRGradientPolylineColorsPayload.self)
        guard payload.colors.count == payload.locations.count else {
            throw mkrBridgeError("MKGradientPolylineRenderer needs exactly one location per color")
        }
        bridge.setColors(payload.colors.map(mkrColor(from:)), locations: payload.locations.map { CGFloat($0) })
    } catch {
        mkrSetError(outError, error)
    }
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

    let overlay = mkrBorrow(polygon, as: MKPolygon.self)
    let renderer = MKPolygonRenderer(polygon: overlay)
    return mkrRetain(renderer)
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
        let payload = MKRStrokeRendererStatePayload(strokeStart: bridge.strokeStart, strokeEnd: bridge.strokeEnd)
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
        mkrApplyStrokeRendererOptions(
            bridge,
            payload,
            setStart: { $0.strokeStart = $1 },
            setEnd: { $0.strokeEnd = $1 }
        )
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

    let bridge = mkrBorrow(overlay, as: MKTileOverlay.self)
    let renderer = MKTileOverlayRenderer(tileOverlay: bridge)
    return mkrRetain(renderer)
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
        let payload = mkrEncodeOverlayRendererState(bridge)
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

    let bridge = mkrBorrow(renderer, as: MKTileOverlayRenderer.self)
    bridge.reloadData()
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

    let multiPolyline = mkrBorrow(overlay, as: MKMultiPolyline.self)
    let renderer = MKMultiPolylineRenderer(multiPolyline: multiPolyline)
    return mkrRetain(renderer)
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

    let multiPolygon = mkrBorrow(overlay, as: MKMultiPolygon.self)
    let renderer = MKMultiPolygonRenderer(multiPolygon: multiPolygon)
    return mkrRetain(renderer)
}
