import Foundation
import MapKit

struct MKRMapSnapshotOptionsPayload: Codable {
    var region: MKRCoordinateRegionPayload?
    var mapRect: MKRMapRectPayload?
    var mapType: MKRMapTypePayload?
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
    var showsPointsOfInterest: Bool
    var showsBuildings: Bool
    var size: MKRScreenSizePayload
}

struct MKRMapSnapshotStatePayload: Codable {
    var imageByteLen: Int
    var size: MKRScreenSizePayload
}

private func mkrBuildMapSnapshotOptions(
    _ payload: MKRMapSnapshotOptionsPayload
) -> MKMapSnapshotter.Options {
    let options = MKMapSnapshotter.Options()
    if let region = payload.region {
        options.region = mkrRegion(from: region)
    }
    if let mapRect = payload.mapRect {
        options.mapRect = mkrMapRect(from: mapRect)
    }
    if let mapType = payload.mapType {
        options.mapType = mkrMapType(from: mapType)
    }
    if #available(macOS 10.15, *) {
        options.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
    }
    options.showsPointsOfInterest = payload.showsPointsOfInterest
    options.showsBuildings = payload.showsBuildings
    options.size = mkrScreenSize(from: payload.size)
    return options
}

@_cdecl("mk_map_snapshotter_new")
public func mk_map_snapshotter_new(
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let payload = try mkrDecodeJSON(optionsJSON, as: MKRMapSnapshotOptionsPayload.self)
        let snapshotter = MKMapSnapshotter(options: mkrBuildMapSnapshotOptions(payload))
        return mkrRetain(snapshotter)
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_snapshotter_start")
public func mk_map_snapshotter_start(
    _ snapshotter: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let snapshotter else {
        mkrSetMessageError(outError, message: "missing MKMapSnapshotter")
        return nil
    }

    let bridge = mkrBorrow(snapshotter, as: MKMapSnapshotter.self)
    do {
        let semaphore = DispatchSemaphore(value: 0)
        let outcome = MKRLockedOutcome<MKMapSnapshotter.Snapshot>()
        bridge.start(with: DispatchQueue.global()) { snapshot, error in
            if let snapshot {
                outcome.set(.success(snapshot))
            } else {
                outcome.set(.failure(error ?? mkrBridgeError("MKMapSnapshotter failed without a snapshot")))
            }
            semaphore.signal()
        }
        if semaphore.wait(timeout: mkrDeadline(seconds: 30)) == .timedOut {
            bridge.cancel()
            throw mkrBridgeError("MKMapSnapshotter timed out")
        }
        guard let result = outcome.get() else {
            throw mkrBridgeError("MKMapSnapshotter completed without a result")
        }
        return mkrRetain(try result.get())
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_snapshotter_cancel")
public func mk_map_snapshotter_cancel(_ snapshotter: UnsafeMutableRawPointer?) {
    guard let snapshotter else { return }
    let bridge = mkrBorrow(snapshotter, as: MKMapSnapshotter.self)
    bridge.cancel()
}

@_cdecl("mk_map_snapshotter_is_loading")
public func mk_map_snapshotter_is_loading(_ snapshotter: UnsafeMutableRawPointer?) -> Bool {
    guard let snapshotter else { return false }
    let bridge = mkrBorrow(snapshotter, as: MKMapSnapshotter.self)
    return bridge.isLoading
}

@_cdecl("mk_map_snapshotter_release")
public func mk_map_snapshotter_release(_ snapshotter: UnsafeMutableRawPointer?) {
    guard let snapshotter else { return }
    mkrRelease(snapshotter)
}

@_cdecl("mk_map_snapshot_state_json")
public func mk_map_snapshot_state_json(
    _ snapshot: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot else {
        mkrSetMessageError(outError, message: "missing MKMapSnapshot")
        return nil
    }

    do {
        let bridge = mkrBorrow(snapshot, as: MKMapSnapshotter.Snapshot.self)
        let payload = MKRMapSnapshotStatePayload(
            imageByteLen: mkrImageByteLength(bridge.image),
            size: mkrEncodeScreenSize(bridge.image.size)
        )
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_snapshot_point_for_coordinate_json")
public func mk_map_snapshot_point_for_coordinate_json(
    _ snapshot: UnsafeMutableRawPointer?,
    _ coordinateJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot else {
        mkrSetMessageError(outError, message: "missing MKMapSnapshot")
        return nil
    }

    do {
        let bridge = mkrBorrow(snapshot, as: MKMapSnapshotter.Snapshot.self)
        let coordinate = try mkrDecodeJSON(coordinateJSON, as: MKRCoordinatePayload.self)
        let point = bridge.point(for: mkrCoordinate(from: coordinate))
        return mkrCString(try mkrEncodeJSON(mkrEncodeScreenPoint(point)))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_map_snapshot_release")
public func mk_map_snapshot_release(_ snapshot: UnsafeMutableRawPointer?) {
    guard let snapshot else { return }
    mkrRelease(snapshot)
}
