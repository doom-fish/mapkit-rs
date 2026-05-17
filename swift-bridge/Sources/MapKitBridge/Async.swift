import Foundation
import MapKit

// MARK: - Async bridge for MapKit completion-handler APIs
//
// Each @_cdecl thunk:
// 1. Retains the caller-supplied object for the Task's lifetime (via
//    Unmanaged.retain / defer { Unmanaged.release }) so the Rust side may
//    release its handle independently without causing use-after-free.
// 2. Wraps the completion-handler API in withCheckedThrowingContinuation.
// 3. Fires the C callback with either a JSON string (result) or an error
//    C-string.  JSON strings are provided via withCString{} so they are
//    valid only for the callback's duration; the Rust side copies them.
// 4. Uses Task { @MainActor in ... } for APIs that require the main thread
//    (MKLocalSearch, MKDirections) and Task { ... } for the rest.

// MARK: MKLocalSearch.start

@_cdecl("mk_local_search_start_async")
public func mk_local_search_start_async(
    _ search: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let search else {
        "missing MKLocalSearch".withCString { cb(nil, $0, ctx) }
        return
    }
    let unmanaged = Unmanaged<MKLocalSearch>.fromOpaque(search)
    _ = unmanaged.retain()
    let localSearch = unmanaged.takeUnretainedValue()
    Task { @MainActor in
        defer { unmanaged.release() }
        do {
            let response = try await withCheckedThrowingContinuation { cont in
                localSearch.start { response, error in
                    if let response {
                        cont.resume(returning: response)
                    } else {
                        cont.resume(throwing: error ?? NSError(
                            domain: "mapkit-rs",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "MKLocalSearch.start failed without response"]
                        ))
                    }
                }
            }
            let json = try mkrEncodeJSON(mkrEncodeLocalSearchResponse(response))
            json.withCString { cb($0, nil, ctx) }
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// MARK: MKDirections.calculate

@_cdecl("mk_directions_calculate_async")
public func mk_directions_calculate_async(
    _ directions: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let directions else {
        "missing MKDirections".withCString { cb(nil, $0, ctx) }
        return
    }
    let unmanaged = Unmanaged<MKDirections>.fromOpaque(directions)
    _ = unmanaged.retain()
    let routeService = unmanaged.takeUnretainedValue()
    Task { @MainActor in
        defer { unmanaged.release() }
        do {
            let response = try await withCheckedThrowingContinuation { cont in
                routeService.calculate { response, error in
                    if let response {
                        cont.resume(returning: response)
                    } else {
                        cont.resume(throwing: error ?? NSError(
                            domain: "mapkit-rs",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "MKDirections.calculate failed without response"]
                        ))
                    }
                }
            }
            let json = try mkrEncodeJSON(mkrEncodeDirectionsResponse(response))
            json.withCString { cb($0, nil, ctx) }
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// MARK: MKDirections.calculateETA

@_cdecl("mk_directions_calculate_eta_async")
public func mk_directions_calculate_eta_async(
    _ directions: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let directions else {
        "missing MKDirections".withCString { cb(nil, $0, ctx) }
        return
    }
    let unmanaged = Unmanaged<MKDirections>.fromOpaque(directions)
    _ = unmanaged.retain()
    let routeService = unmanaged.takeUnretainedValue()
    Task { @MainActor in
        defer { unmanaged.release() }
        do {
            let response = try await withCheckedThrowingContinuation { cont in
                routeService.calculateETA { response, error in
                    if let response {
                        cont.resume(returning: response)
                    } else {
                        cont.resume(throwing: error ?? NSError(
                            domain: "mapkit-rs",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "MKDirections.calculateETA failed without response"]
                        ))
                    }
                }
            }
            let json = try mkrEncodeJSON(mkrEncodeETAResponse(response))
            json.withCString { cb($0, nil, ctx) }
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// MARK: MKMapSnapshotter.start
// Does NOT require @MainActor — snapshotter accepts a background DispatchQueue.

@_cdecl("mk_map_snapshotter_start_async")
public func mk_map_snapshotter_start_async(
    _ snapshotter: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let snapshotter else {
        "missing MKMapSnapshotter".withCString { cb(nil, $0, ctx) }
        return
    }
    let unmanaged = Unmanaged<MKMapSnapshotter>.fromOpaque(snapshotter)
    _ = unmanaged.retain()
    let bridge = unmanaged.takeUnretainedValue()
    Task {
        defer { unmanaged.release() }
        do {
            let snapshot = try await withCheckedThrowingContinuation { cont in
                bridge.start(with: DispatchQueue.global(qos: .userInitiated)) { snapshot, error in
                    if let snapshot {
                        cont.resume(returning: snapshot)
                    } else {
                        cont.resume(throwing: error ?? NSError(
                            domain: "mapkit-rs",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "MKMapSnapshotter.start failed without snapshot"]
                        ))
                    }
                }
            }
            cb(mkrRetain(snapshot), nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// MARK: MKGeocodingRequest.getMapItems (macOS 26.0+)

@available(macOS 26.0, *)
@_cdecl("mk_geocoding_request_map_items_async")
public func mk_geocoding_request_map_items_async(
    _ request: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let request else {
        "missing MKGeocodingRequest".withCString { cb(nil, $0, ctx) }
        return
    }
    let unmanaged = Unmanaged<MKGeocodingRequest>.fromOpaque(request)
    _ = unmanaged.retain()
    let geocoder = unmanaged.takeUnretainedValue()
    Task { @MainActor in
        defer { unmanaged.release() }
        do {
            let mapItems = try await withCheckedThrowingContinuation { cont in
                geocoder.getMapItems { mapItems, error in
                    if let mapItems {
                        cont.resume(returning: mapItems)
                    } else {
                        cont.resume(throwing: error ?? NSError(
                            domain: "mapkit-rs",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "MKGeocodingRequest.getMapItems failed without items"]
                        ))
                    }
                }
            }
            let json = try mkrEncodeJSON(mapItems.map(mkrEncodeMapItem))
            json.withCString { cb($0, nil, ctx) }
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// MARK: MKReverseGeocodingRequest.getMapItems (macOS 26.0+)

@available(macOS 26.0, *)
@_cdecl("mk_reverse_geocoding_request_map_items_async")
public func mk_reverse_geocoding_request_map_items_async(
    _ request: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let request else {
        "missing MKReverseGeocodingRequest".withCString { cb(nil, $0, ctx) }
        return
    }
    let unmanaged = Unmanaged<MKReverseGeocodingRequest>.fromOpaque(request)
    _ = unmanaged.retain()
    let geocoder = unmanaged.takeUnretainedValue()
    Task { @MainActor in
        defer { unmanaged.release() }
        do {
            let mapItems = try await withCheckedThrowingContinuation { cont in
                geocoder.getMapItems { mapItems, error in
                    if let mapItems {
                        cont.resume(returning: mapItems)
                    } else {
                        cont.resume(throwing: error ?? NSError(
                            domain: "mapkit-rs",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "MKReverseGeocodingRequest.getMapItems failed without items"]
                        ))
                    }
                }
            }
            let json = try mkrEncodeJSON(mapItems.map(mkrEncodeMapItem))
            json.withCString { cb($0, nil, ctx) }
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}
