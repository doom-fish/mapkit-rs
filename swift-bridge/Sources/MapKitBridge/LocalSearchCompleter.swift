import Foundation
import MapKit

struct MKRTextHighlightRangePayload: Codable {
    var location: Int
    var length: Int
}

struct MKRLocalSearchCompletionPayload: Codable {
    var title: String
    var titleHighlightRanges: [MKRTextHighlightRangePayload]
    var subtitle: String
    var subtitleHighlightRanges: [MKRTextHighlightRangePayload]
}

struct MKRLocalSearchCompleterStatePayload: Codable {
    var queryFragment: String
    var region: MKRCoordinateRegionPayload
    var regionPriority: MKRLocalSearchRegionPriorityPayload
    var resultTypes: UInt
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
    var addressFilter: MKRAddressFilterPayload?
    var results: [MKRLocalSearchCompletionPayload]
    var searching: Bool
}

struct MKRLocalSearchCompleterOptionsPayload: Codable {
    var queryFragmentPresent: Bool
    var queryFragment: String?
    var region: MKRCoordinateRegionPayload?
    var regionPriority: MKRLocalSearchRegionPriorityPayload?
    var resultTypes: UInt?
    var pointOfInterestFilterPresent: Bool
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
    var addressFilterPresent: Bool
    var addressFilter: MKRAddressFilterPayload?
}

private final class MKRLocalSearchCompleterProxy: NSObject, MKLocalSearchCompleterDelegate {
    private let callback: (Result<[MKRLocalSearchCompletionPayload], Error>) -> Void
    private var completed = false

    init(callback: @escaping (Result<[MKRLocalSearchCompletionPayload], Error>) -> Void) {
        self.callback = callback
    }

    func completerDidUpdateResults(_ completer: MKLocalSearchCompleter) {
        finish(.success(completer.results.map(mkrEncodeLocalSearchCompletion)))
    }

    func completer(_ completer: MKLocalSearchCompleter, didFailWithError error: any Error) {
        finish(.failure(error))
    }

    private func finish(_ result: Result<[MKRLocalSearchCompletionPayload], Error>) {
        guard !completed else { return }
        completed = true
        callback(result)
    }
}

private func mkrEncodeTextHighlightRanges(_ values: [NSValue]) -> [MKRTextHighlightRangePayload] {
    values.map {
        let range = $0.rangeValue
        return MKRTextHighlightRangePayload(location: range.location, length: range.length)
    }
}

func mkrEncodeLocalSearchCompletion(
    _ completion: MKLocalSearchCompletion
) -> MKRLocalSearchCompletionPayload {
    MKRLocalSearchCompletionPayload(
        title: completion.title,
        titleHighlightRanges: mkrEncodeTextHighlightRanges(completion.titleHighlightRanges),
        subtitle: completion.subtitle,
        subtitleHighlightRanges: mkrEncodeTextHighlightRanges(completion.subtitleHighlightRanges)
    )
}

@_cdecl("mk_local_search_completer_new")
public func mk_local_search_completer_new(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let completer = MKLocalSearchCompleter()
    return mkrRetain(completer)
}

@_cdecl("mk_local_search_completer_state_json")
public func mk_local_search_completer_state_json(
    _ completer: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let completer else {
        mkrSetMessageError(outError, message: "missing MKLocalSearchCompleter")
        return nil
    }

    do {
        let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
        let payload = try mkrSyncOnMain {
            MKRLocalSearchCompleterStatePayload(
                queryFragment: bridge.queryFragment,
                region: mkrEncodeRegion(bridge.region),
                regionPriority: {
                    if #available(macOS 15.0, *) {
                        return bridge.regionPriority == .required ? .required : .default
                    }
                    return .default
                }(),
                resultTypes: bridge.resultTypes.rawValue,
                pointOfInterestFilter: mkrEncodePointOfInterestFilter(bridge.pointOfInterestFilter),
                addressFilter: {
                    if #available(macOS 15.0, *) {
                        return mkrEncodeAddressFilter(bridge.addressFilter)
                    }
                    return nil
                }(),
                results: bridge.results.map(mkrEncodeLocalSearchCompletion),
                searching: bridge.isSearching
            )
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_local_search_completer_apply_options_json")
public func mk_local_search_completer_apply_options_json(
    _ completer: UnsafeMutableRawPointer?,
    _ payloadJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let completer else {
        mkrSetMessageError(outError, message: "missing MKLocalSearchCompleter")
        return
    }

    do {
        let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
        let payload = try mkrDecodeJSON(payloadJSON, as: MKRLocalSearchCompleterOptionsPayload.self)
        try mkrSyncOnMain {
            if payload.queryFragmentPresent {
                bridge.queryFragment = payload.queryFragment ?? ""
            }
            if let region = payload.region {
                bridge.region = mkrRegion(from: region)
            }
            if let regionPriority = payload.regionPriority, #available(macOS 15.0, *) {
                bridge.regionPriority = regionPriority == .required ? .required : .default
            }
            if let resultTypes = payload.resultTypes {
                bridge.resultTypes = .init(rawValue: resultTypes)
            }
            if payload.pointOfInterestFilterPresent {
                if #available(macOS 10.15, *) {
                    bridge.pointOfInterestFilter = mkrBuildPointOfInterestFilter(payload.pointOfInterestFilter)
                } else if payload.pointOfInterestFilter != nil {
                    throw NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "pointOfInterestFilter requires macOS 10.15+"]
                    )
                }
            }
            if payload.addressFilterPresent {
                if #available(macOS 15.0, *) {
                    bridge.addressFilter = try mkrBuildAddressFilter(payload.addressFilter)
                } else if payload.addressFilter != nil {
                    throw NSError(
                        domain: "mapkit-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "addressFilter requires macOS 15.0+"]
                    )
                }
            }
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_local_search_completer_refresh_json")
public func mk_local_search_completer_refresh_json(
    _ completer: UnsafeMutableRawPointer?,
    _ timeoutMillis: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let completer else {
        mkrSetMessageError(outError, message: "missing MKLocalSearchCompleter")
        return nil
    }

    do {
        let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
        let timeoutSeconds = max(Double(timeoutMillis) / 1000, 0.1)
        let results = try mkrAwaitOnMain(timeoutSeconds: timeoutSeconds) { completion in
            if !bridge.isSearching {
                completion(.success(bridge.results.map(mkrEncodeLocalSearchCompletion)))
                return
            }

            var proxy: MKRLocalSearchCompleterProxy?
            proxy = MKRLocalSearchCompleterProxy { result in
                bridge.delegate = nil
                _ = proxy
                proxy = nil
                completion(result)
            }
            bridge.delegate = proxy
        }
        return mkrCString(try mkrEncodeJSON(results))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_local_search_completer_cancel")
public func mk_local_search_completer_cancel(_ completer: UnsafeMutableRawPointer?) {
    guard let completer else { return }
    let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
    bridge.cancel()
}

@_cdecl("mk_local_search_completer_release")
public func mk_local_search_completer_release(_ completer: UnsafeMutableRawPointer?) {
    guard let completer else { return }
    mkrRelease(completer)
}
