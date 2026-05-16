import Foundation
import MapKit

enum MKRMapElevationStylePayload: String, Codable {
    case flat
    case realistic
}

enum MKRStandardMapEmphasisStylePayload: String, Codable {
    case `default`
    case muted
}

enum MKRMapConfigurationKindPayload: String, Codable {
    case standard
    case hybrid
    case imagery
}

struct MKRMapCameraPayload: Codable {
    var centerCoordinate: MKRCoordinatePayload
    var centerCoordinateDistance: Double
    var heading: Double
    var pitch: Double
    var altitude: Double
}

struct MKRMapCameraBoundaryPayload: Codable {
    var mapRect: MKRMapRectPayload
    var region: MKRCoordinateRegionPayload
}

struct MKRMapCameraZoomRangePayload: Codable {
    var minCenterCoordinateDistance: Double?
    var maxCenterCoordinateDistance: Double?
}

struct MKRMapConfigurationPayload: Codable {
    var kind: MKRMapConfigurationKindPayload
    var elevationStyle: MKRMapElevationStylePayload
    var emphasisStyle: MKRStandardMapEmphasisStylePayload?
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
    var showsTraffic: Bool?
}

func mkrMapElevationStyle(from payload: MKRMapElevationStylePayload) -> MKMapConfiguration.ElevationStyle {
    switch payload {
    case .flat: return .flat
    case .realistic: return .realistic
    }
}

func mkrEncodeMapElevationStyle(_ style: MKMapConfiguration.ElevationStyle) -> MKRMapElevationStylePayload {
    switch style {
    case .flat: return .flat
    case .realistic: return .realistic
    @unknown default: return .flat
    }
}

func mkrStandardMapEmphasisStyle(
    from payload: MKRStandardMapEmphasisStylePayload
) -> MKStandardMapConfiguration.EmphasisStyle {
    switch payload {
    case .default: return .default
    case .muted: return .muted
    }
}

func mkrEncodeStandardMapEmphasisStyle(
    _ style: MKStandardMapConfiguration.EmphasisStyle
) -> MKRStandardMapEmphasisStylePayload {
    switch style {
    case .default: return .default
    case .muted: return .muted
    @unknown default: return .default
    }
}

func mkrBuildMapCamera(_ payload: MKRMapCameraPayload) -> MKMapCamera {
    let camera = MKMapCamera()
    camera.centerCoordinate = mkrCoordinate(from: payload.centerCoordinate)
    if #available(macOS 10.15, *) {
        camera.centerCoordinateDistance = payload.centerCoordinateDistance
    } else {
        camera.altitude = payload.altitude
    }
    camera.heading = payload.heading
    camera.pitch = payload.pitch
    return camera
}

func mkrEncodeMapCamera(_ camera: MKMapCamera) -> MKRMapCameraPayload {
    MKRMapCameraPayload(
        centerCoordinate: mkrEncodeCoordinate(camera.centerCoordinate),
        centerCoordinateDistance: {
            if #available(macOS 10.15, *) {
                return camera.centerCoordinateDistance
            }
            return camera.altitude
        }(),
        heading: camera.heading,
        pitch: camera.pitch,
        altitude: camera.altitude
    )
}

func mkrBuildMapCameraZoomRange(_ payload: MKRMapCameraZoomRangePayload) throws -> MKMapView.CameraZoomRange {
    if let minDistance = payload.minCenterCoordinateDistance,
       let maxDistance = payload.maxCenterCoordinateDistance {
        guard let range = MKMapView.CameraZoomRange(
            minCenterCoordinateDistance: minDistance,
            maxCenterCoordinateDistance: maxDistance
        ) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKMapCameraZoomRange"]
            )
        }
        return range
    }
    if let minDistance = payload.minCenterCoordinateDistance {
        guard let range = MKMapView.CameraZoomRange(minCenterCoordinateDistance: minDistance) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKMapCameraZoomRange"]
            )
        }
        return range
    }
    if let maxDistance = payload.maxCenterCoordinateDistance {
        guard let range = MKMapView.CameraZoomRange(maxCenterCoordinateDistance: maxDistance) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKMapCameraZoomRange"]
            )
        }
        return range
    }
    throw NSError(
        domain: "mapkit-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "MKMapCameraZoomRange requires a min or max distance"]
    )
}

func mkrEncodeMapCameraZoomRange(_ range: MKMapView.CameraZoomRange?) -> MKRMapCameraZoomRangePayload? {
    guard let range else { return nil }
    return MKRMapCameraZoomRangePayload(
        minCenterCoordinateDistance: range.minCenterCoordinateDistance,
        maxCenterCoordinateDistance: range.maxCenterCoordinateDistance
    )
}

func mkrBuildMapCameraBoundary(from payload: MKRMapCameraBoundaryPayload) throws -> MKMapView.CameraBoundary {
    guard let boundary = MKMapView.CameraBoundary(mapRect: mkrMapRect(from: payload.mapRect)) else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "failed to create MKMapCameraBoundary"]
        )
    }
    return boundary
}

func mkrEncodeMapCameraBoundary(_ boundary: MKMapView.CameraBoundary?) -> MKRMapCameraBoundaryPayload? {
    guard let boundary else { return nil }
    return MKRMapCameraBoundaryPayload(
        mapRect: mkrEncodeMapRect(boundary.mapRect),
        region: mkrEncodeRegion(boundary.region)
    )
}

func mkrBuildMapConfiguration(_ payload: MKRMapConfigurationPayload) -> MKMapConfiguration {
    switch payload.kind {
    case .standard:
        let configuration = MKStandardMapConfiguration(
            elevationStyle: mkrMapElevationStyle(from: payload.elevationStyle),
            emphasisStyle: mkrStandardMapEmphasisStyle(
                from: payload.emphasisStyle ?? .default
            )
        )
        configuration.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
        configuration.showsTraffic = payload.showsTraffic ?? false
        return configuration
    case .hybrid:
        let configuration = MKHybridMapConfiguration(
            elevationStyle: mkrMapElevationStyle(from: payload.elevationStyle)
        )
        configuration.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
        configuration.showsTraffic = payload.showsTraffic ?? false
        return configuration
    case .imagery:
        return MKImageryMapConfiguration(
            elevationStyle: mkrMapElevationStyle(from: payload.elevationStyle)
        )
    }
}

func mkrEncodeMapConfiguration(_ configuration: MKMapConfiguration?) -> MKRMapConfigurationPayload? {
    guard let configuration else { return nil }

    if let standard = configuration as? MKStandardMapConfiguration {
        return MKRMapConfigurationPayload(
            kind: .standard,
            elevationStyle: mkrEncodeMapElevationStyle(standard.elevationStyle),
            emphasisStyle: mkrEncodeStandardMapEmphasisStyle(standard.emphasisStyle),
            pointOfInterestFilter: mkrEncodePointOfInterestFilter(standard.pointOfInterestFilter),
            showsTraffic: standard.showsTraffic
        )
    }

    if let hybrid = configuration as? MKHybridMapConfiguration {
        return MKRMapConfigurationPayload(
            kind: .hybrid,
            elevationStyle: mkrEncodeMapElevationStyle(hybrid.elevationStyle),
            emphasisStyle: nil,
            pointOfInterestFilter: mkrEncodePointOfInterestFilter(hybrid.pointOfInterestFilter),
            showsTraffic: hybrid.showsTraffic
        )
    }

    let imagery = configuration as! MKImageryMapConfiguration
    return MKRMapConfigurationPayload(
        kind: .imagery,
        elevationStyle: mkrEncodeMapElevationStyle(imagery.elevationStyle),
        emphasisStyle: nil,
        pointOfInterestFilter: nil,
        showsTraffic: nil
    )
}

@_cdecl("mk_map_camera_boundary_from_map_rect_json")
public func mk_map_camera_boundary_from_map_rect_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let mapRect = try mkrDecodeJSON(payloadJSON, as: MKRMapRectPayload.self)
        guard let boundary = MKMapView.CameraBoundary(mapRect: mkrMapRect(from: mapRect)) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKMapCameraBoundary"]
            )
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeMapCameraBoundary(boundary)!))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_camera_boundary_from_region_json")
public func mk_map_camera_boundary_from_region_json(
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let region = try mkrDecodeJSON(payloadJSON, as: MKRCoordinateRegionPayload.self)
        guard let boundary = MKMapView.CameraBoundary(coordinateRegion: mkrRegion(from: region)) else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "failed to create MKMapCameraBoundary"]
            )
        }
        return mkrCString(try mkrEncodeJSON(mkrEncodeMapCameraBoundary(boundary)!))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_camera_zoom_default")
public func mk_map_camera_zoom_default() -> Double {
    MKMapCameraZoomDefault
}
