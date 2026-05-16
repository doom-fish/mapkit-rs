import Foundation
import MapKit

@_cdecl("mk_distance_formatter_new")
public func mk_distance_formatter_new() -> UnsafeMutableRawPointer {
    mkrRetain(MKDistanceFormatter())
}

@_cdecl("mk_distance_formatter_set_units")
public func mk_distance_formatter_set_units(_ formatter: UnsafeMutableRawPointer?, _ rawUnits: UInt64) {
    guard let formatter else { return }
    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    switch rawUnits {
    case 1:
        distanceFormatter.units = .metric
    case 2:
        distanceFormatter.units = .imperial
    case 3:
        distanceFormatter.units = .imperialWithYards
    default:
        distanceFormatter.units = .default
    }
}

@_cdecl("mk_distance_formatter_set_unit_style")
public func mk_distance_formatter_set_unit_style(_ formatter: UnsafeMutableRawPointer?, _ rawUnitStyle: UInt64) {
    guard let formatter else { return }
    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    switch rawUnitStyle {
    case 1:
        distanceFormatter.unitStyle = .abbreviated
    case 2:
        distanceFormatter.unitStyle = .full
    default:
        distanceFormatter.unitStyle = .default
    }
}

@_cdecl("mk_distance_formatter_string_from_distance")
public func mk_distance_formatter_string_from_distance(
    _ formatter: UnsafeMutableRawPointer?,
    _ distance: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let formatter else {
        mkrSetMessageError(outError, message: "missing MKDistanceFormatter")
        return nil
    }

    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    return mkrCString(distanceFormatter.string(fromDistance: distance))
}

@_cdecl("mk_distance_formatter_distance_from_string")
public func mk_distance_formatter_distance_from_string(
    _ formatter: UnsafeMutableRawPointer?,
    _ distanceString: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    guard let formatter else {
        mkrSetMessageError(outError, message: "missing MKDistanceFormatter")
        return -1
    }
    guard let distanceString else {
        mkrSetMessageError(outError, message: "missing distance string")
        return -1
    }

    let distanceFormatter = mkrBorrow(formatter, as: MKDistanceFormatter.self)
    let parsed = distanceFormatter.distance(from: String(cString: distanceString))
    if parsed < 0 {
        mkrSetMessageError(outError, message: "failed to parse distance string")
    }
    return parsed
}

@_cdecl("mk_distance_formatter_release")
public func mk_distance_formatter_release(_ formatter: UnsafeMutableRawPointer?) {
    guard let formatter else { return }
    mkrRelease(formatter)
}
