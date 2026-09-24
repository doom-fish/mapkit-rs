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

public typealias MKRLocalSearchCompleterEventCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<CChar>?,
    UnsafeMutablePointer<CChar>?
) -> Void

final class MKRLocalSearchCompleterDelegateProxy: NSObject, MKLocalSearchCompleterDelegate {
    private let context: UnsafeMutableRawPointer
    private let releaseContext: MKRContextCallback
    private let onEvent: MKRLocalSearchCompleterEventCallback

    init(
        context: UnsafeMutableRawPointer,
        retainContext: MKRContextCallback,
        releaseContext: @escaping MKRContextCallback,
        onEvent: @escaping MKRLocalSearchCompleterEventCallback
    ) {
        retainContext(context)
        self.context = context
        self.releaseContext = releaseContext
        self.onEvent = onEvent
    }

    deinit {
        releaseContext(context)
    }

    func completerDidUpdateResults(_ completer: MKLocalSearchCompleter) {
        do {
            let json = try mkrEncodeJSON(completer.results.map(mkrEncodeLocalSearchCompletion))
            onEvent(context, mkrRetain(completer), mkrCString(json), nil)
        } catch {
            fail(completer, error)
        }
    }

    func completer(_ completer: MKLocalSearchCompleter, didFailWithError error: any Error) {
        fail(completer, error)
    }

    private func fail(_ completer: MKLocalSearchCompleter, _ error: Error) {
        let json = (try? mkrEncodeJSON(mkrErrorPayload(from: error))) ?? error.localizedDescription
        onEvent(context, mkrRetain(completer), nil, mkrCString(json))
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
    do {
        return try mkrOnMainThread("MKLocalSearchCompleter") {
            mkrRetain(MKLocalSearchCompleter())
        }
    } catch {
        mkrSetError(outError, error)
        return nil
    }
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
        let payload = try mkrOnMainThread("MKLocalSearchCompleter") {
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
        try mkrOnMainThread("MKLocalSearchCompleter") {
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

@_cdecl("mk_local_search_completer_set_delegate")
public func mk_local_search_completer_set_delegate(
    _ completer: UnsafeMutableRawPointer?,
    _ context: UnsafeMutableRawPointer?,
    _ retainContext: MKRContextCallback?,
    _ releaseContext: MKRContextCallback?,
    _ onEvent: MKRLocalSearchCompleterEventCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let completer, let context, let retainContext, let releaseContext, let onEvent else {
        mkrSetMessageError(outError, message: "missing MKLocalSearchCompleter delegate arguments")
        return nil
    }

    do {
        return try mkrOnMainThread("MKLocalSearchCompleter") {
            let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
            let proxy = MKRLocalSearchCompleterDelegateProxy(
                context: context,
                retainContext: retainContext,
                releaseContext: releaseContext,
                onEvent: onEvent
            )
            bridge.delegate = proxy
            return mkrRetain(proxy)
        }
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_local_search_completer_clear_delegate")
public func mk_local_search_completer_clear_delegate(
    _ completer: UnsafeMutableRawPointer?,
    _ proxy: UnsafeMutableRawPointer?
) {
    guard let completer, let proxy else { return }
    let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
    let delegate = mkrBorrow(proxy, as: MKRLocalSearchCompleterDelegateProxy.self)
    let clear = {
        if (bridge.delegate as AnyObject?) === delegate {
            bridge.delegate = nil
        }
    }
    if Thread.isMainThread {
        clear()
    } else {
        DispatchQueue.main.async(execute: clear)
    }
}

@_cdecl("mk_local_search_completer_cancel")
public func mk_local_search_completer_cancel(_ completer: UnsafeMutableRawPointer?) {
    guard let completer, Thread.isMainThread else { return }
    let bridge = mkrBorrow(completer, as: MKLocalSearchCompleter.self)
    bridge.cancel()
}

@_cdecl("mk_local_search_completer_release")
public func mk_local_search_completer_release(_ completer: UnsafeMutableRawPointer?) {
    guard let completer else { return }
    mkrRelease(completer)
}
