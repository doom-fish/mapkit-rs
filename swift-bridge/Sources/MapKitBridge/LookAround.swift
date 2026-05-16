import Foundation
import MapKit

struct MKRLookAroundSceneRequestStatePayload: Codable {
    var coordinate: MKRCoordinatePayload
    var hasMapItem: Bool
    var cancelled: Bool
    var loading: Bool
}

struct MKRLookAroundSnapshotOptionsPayload: Codable {
    var size: MKRScreenSizePayload
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
}

struct MKRLookAroundSnapshotStatePayload: Codable {
    var imageByteLen: Int
    var size: MKRScreenSizePayload
}

private func mkrBuildLookAroundSnapshotOptions(
    _ payload: MKRLookAroundSnapshotOptionsPayload
) -> MKLookAroundSnapshotter.Options {
    let options = MKLookAroundSnapshotter.Options()
    options.size = mkrScreenSize(from: payload.size)
    options.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
    return options
}

@_cdecl("mk_look_around_scene_request_new_coordinate_json")
public func mk_look_around_scene_request_new_coordinate_json(
    _ coordinateJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let coordinate = try mkrDecodeJSON(coordinateJSON, as: MKRCoordinatePayload.self)
        let request = MKLookAroundSceneRequest(coordinate: mkrCoordinate(from: coordinate))
        return mkrRetain(request)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_scene_request_new_map_item_json")
public func mk_look_around_scene_request_new_map_item_json(
    _ mapItemJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(mapItemJSON, as: MKRMapItemPayload.self)
        let request = MKLookAroundSceneRequest(mapItem: try mkrBuildMapItem(payload))
        return mkrRetain(request)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_scene_request_state_json")
public func mk_look_around_scene_request_state_json(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKLookAroundSceneRequest")
        return nil
    }

    do {
        let sceneRequest = mkrBorrow(request, as: MKLookAroundSceneRequest.self)
        let payload = MKRLookAroundSceneRequestStatePayload(
            coordinate: mkrEncodeCoordinate(sceneRequest.coordinate),
            hasMapItem: sceneRequest.mapItem != nil,
            cancelled: sceneRequest.isCancelled,
            loading: sceneRequest.isLoading
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_scene_request_get_scene")
public func mk_look_around_scene_request_get_scene(
    _ request: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let request else {
        mkrSetMessageError(outError, message: "missing MKLookAroundSceneRequest")
        return nil
    }

    let sceneRequest = mkrBorrow(request, as: MKLookAroundSceneRequest.self)
    do {
        let scene: MKLookAroundScene = try mkrAwaitOnMain { completion in
            sceneRequest.getSceneWithCompletionHandler { scene, error in
                if let scene {
                    completion(.success(scene))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKLookAroundSceneRequest failed without a scene"]
                    )))
                }
            }
        }
        return mkrRetain(scene)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_scene_request_cancel")
public func mk_look_around_scene_request_cancel(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    let sceneRequest = mkrBorrow(request, as: MKLookAroundSceneRequest.self)
    sceneRequest.cancel()
}

@_cdecl("mk_look_around_scene_request_release")
public func mk_look_around_scene_request_release(_ request: UnsafeMutableRawPointer?) {
    guard let request else { return }
    mkrRelease(request)
}

@_cdecl("mk_look_around_scene_release")
public func mk_look_around_scene_release(_ scene: UnsafeMutableRawPointer?) {
    guard let scene else { return }
    mkrRelease(scene)
}

@_cdecl("mk_look_around_snapshotter_new")
public func mk_look_around_snapshotter_new(
    _ scene: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let scene else {
        mkrSetMessageError(outError, message: "missing MKLookAroundScene")
        return nil
    }

    do {
        let lookAroundScene = mkrBorrow(scene, as: MKLookAroundScene.self)
        let payload = try mkrDecodeJSON(optionsJSON, as: MKRLookAroundSnapshotOptionsPayload.self)
        let snapshotter = MKLookAroundSnapshotter(
            scene: lookAroundScene,
            options: mkrBuildLookAroundSnapshotOptions(payload)
        )
        return mkrRetain(snapshotter)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_snapshotter_get_snapshot")
public func mk_look_around_snapshotter_get_snapshot(
    _ snapshotter: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let snapshotter else {
        mkrSetMessageError(outError, message: "missing MKLookAroundSnapshotter")
        return nil
    }

    let bridge = mkrBorrow(snapshotter, as: MKLookAroundSnapshotter.self)
    do {
        let snapshot: MKLookAroundSnapshotter.Snapshot = try mkrAwaitOnMain { completion in
            bridge.getSnapshotWithCompletionHandler { snapshot, error in
                if let snapshot {
                    completion(.success(snapshot))
                } else {
                    completion(.failure(error ?? NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "MKLookAroundSnapshotter failed without a snapshot"]
                    )))
                }
            }
        }
        return mkrRetain(snapshot)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_snapshotter_cancel")
public func mk_look_around_snapshotter_cancel(_ snapshotter: UnsafeMutableRawPointer?) {
    guard let snapshotter else { return }
    let bridge = mkrBorrow(snapshotter, as: MKLookAroundSnapshotter.self)
    bridge.cancel()
}

@_cdecl("mk_look_around_snapshotter_is_loading")
public func mk_look_around_snapshotter_is_loading(_ snapshotter: UnsafeMutableRawPointer?) -> Bool {
    guard let snapshotter else { return false }
    let bridge = mkrBorrow(snapshotter, as: MKLookAroundSnapshotter.self)
    return bridge.isLoading
}

@_cdecl("mk_look_around_snapshotter_release")
public func mk_look_around_snapshotter_release(_ snapshotter: UnsafeMutableRawPointer?) {
    guard let snapshotter else { return }
    mkrRelease(snapshotter)
}

@_cdecl("mk_look_around_snapshot_state_json")
public func mk_look_around_snapshot_state_json(
    _ snapshot: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot else {
        mkrSetMessageError(outError, message: "missing MKLookAroundSnapshot")
        return nil
    }

    do {
        let bridge = mkrBorrow(snapshot, as: MKLookAroundSnapshotter.Snapshot.self)
        let payload = MKRLookAroundSnapshotStatePayload(
            imageByteLen: mkrImageByteLength(bridge.image),
            size: mkrEncodeScreenSize(bridge.image.size)
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_look_around_snapshot_release")
public func mk_look_around_snapshot_release(_ snapshot: UnsafeMutableRawPointer?) {
    guard let snapshot else { return }
    mkrRelease(snapshot)
}
