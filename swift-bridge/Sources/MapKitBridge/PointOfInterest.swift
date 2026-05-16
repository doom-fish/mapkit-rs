import Foundation
import MapKit

enum MKRPointOfInterestFilterModePayload: String, Codable {
    case including
    case excluding
    case includingAll
    case excludingAll
}

struct MKRPointOfInterestFilterPayload: Codable {
    var mode: MKRPointOfInterestFilterModePayload
    var categories: [String]
}

struct MKRLocalPointsOfInterestRequestPayload: Codable {
    var coordinate: MKRCoordinatePayload?
    var radius: Double?
    var region: MKRCoordinateRegionPayload?
    var pointOfInterestFilter: MKRPointOfInterestFilterPayload?
}

func mkrPointOfInterestCategory(from value: String) -> MKPointOfInterestCategory {
    switch value.lowercased() {
    case "airport": return .airport
    case "cafe": return .cafe
    case "hotel": return .hotel
    case "library": return .library
    case "museum": return .museum
    case "park": return .park
    case "restaurant": return .restaurant
    case "school": return .school
    case "store": return .store
    case "university": return .university
    default: return MKPointOfInterestCategory(rawValue: value)
    }
}

func mkrBuildPointOfInterestFilter(
    _ payload: MKRPointOfInterestFilterPayload?
) -> MKPointOfInterestFilter? {
    guard let payload else { return nil }
    let categories = payload.categories.map(mkrPointOfInterestCategory)
    switch payload.mode {
    case .including:
        return MKPointOfInterestFilter(including: categories)
    case .excluding:
        return MKPointOfInterestFilter(excluding: categories)
    case .includingAll:
        return .includingAll
    case .excludingAll:
        return .excludingAll
    }
}
