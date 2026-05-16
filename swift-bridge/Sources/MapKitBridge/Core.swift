import AppKit
import CoreLocation
import Foundation
import MapKit

@_cdecl("mk_string_free")
public func mk_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@inline(__always)
public func mkrCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func mkrRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func mkrBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
public func mkrRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

public struct MKRErrorPayload: Codable {
    public var domain: String
    public var code: Int
    public var message: String
}

private let mkrFractionalDateFormatter: ISO8601DateFormatter = {
    let formatter = ISO8601DateFormatter()
    formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
    return formatter
}()

public func mkrDateString(_ date: Date?) -> String? {
    guard let date else { return nil }
    return mkrFractionalDateFormatter.string(from: date)
}

public func mkrEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let encoder = JSONEncoder()
    let data = try encoder.encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "failed to encode JSON as UTF-8"]
        )
    }
    return string
}

public func mkrDecodeJSON<T: Decodable>(_ json: UnsafePointer<CChar>?, as _: T.Type) throws -> T {
    guard let json else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "missing JSON payload"]
        )
    }

    let data = Data(String(cString: json).utf8)
    return try JSONDecoder().decode(T.self, from: data)
}

public func mkrErrorPayload(from error: Error) -> MKRErrorPayload {
    let nsError = error as NSError
    return MKRErrorPayload(
        domain: nsError.domain,
        code: nsError.code,
        message: nsError.localizedDescription
    )
}

public func mkrSetError(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) {
    guard let outError else { return }

    if let json = try? mkrEncodeJSON(mkrErrorPayload(from: error)) {
        outError.pointee = mkrCString(json)
    } else {
        outError.pointee = mkrCString((error as NSError).localizedDescription)
    }
}

public func mkrSetMessageError(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    message: String,
    domain: String = "mapkit-rs",
    code: Int = -1
) {
    guard let outError else { return }
    let payload = MKRErrorPayload(domain: domain, code: code, message: message)
    if let json = try? mkrEncodeJSON(payload) {
        outError.pointee = mkrCString(json)
    } else {
        outError.pointee = mkrCString(message)
    }
}

public func mkrSyncOnMain<T>(_ operation: @escaping () throws -> T) throws -> T {
    if Thread.isMainThread {
        return try operation()
    }

    var outcome: Result<T, Error>!
    DispatchQueue.main.sync {
        outcome = Result { try operation() }
    }
    return try outcome.get()
}

public func mkrAwaitOnMain<T>(
    timeoutSeconds: TimeInterval = 30,
    _ operation: @escaping (@escaping (Result<T, Error>) -> Void) -> Void
) throws -> T {
    if Thread.isMainThread {
        var outcome: Result<T, Error>?
        operation { outcome = $0 }
        let deadline = Date(timeIntervalSinceNow: timeoutSeconds)
        while outcome == nil && deadline.timeIntervalSinceNow > 0 {
            RunLoop.current.run(mode: .default, before: Date(timeIntervalSinceNow: 0.01))
        }
        guard let outcome else {
            throw NSError(
                domain: "mapkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "operation timed out"]
            )
        }
        return try outcome.get()
    }

    let semaphore = DispatchSemaphore(value: 0)
    var outcome: Result<T, Error>?
    DispatchQueue.main.async {
        operation {
            outcome = $0
            semaphore.signal()
        }
    }

    let timeout = DispatchTime.now() + .milliseconds(Int(timeoutSeconds * 1000))
    if semaphore.wait(timeout: timeout) == .timedOut {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "operation timed out"]
        )
    }

    guard let outcome else {
        throw NSError(
            domain: "mapkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "operation completed without a result"]
        )
    }

    return try outcome.get()
}

public func mkrImageByteLength(_ image: NSImage) -> Int {
    image.tiffRepresentation?.count ?? 0
}

public func mkrLocaleIdentifier(_ locale: Locale?) -> String? {
    locale?.identifier
}

public func mkrLocale(from identifier: String?) -> Locale? {
    guard let identifier else { return nil }
    return Locale(identifier: identifier)
}
