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

private let mkrKnownPointOfInterestCategories: [(String, MKPointOfInterestCategory)] = [
    ("animalService", MKPointOfInterestCategory(rawValue: "MKPOICategoryAnimalService")),
    ("airport", MKPointOfInterestCategory(rawValue: "MKPOICategoryAirport")),
    ("amusementPark", MKPointOfInterestCategory(rawValue: "MKPOICategoryAmusementPark")),
    ("aquarium", MKPointOfInterestCategory(rawValue: "MKPOICategoryAquarium")),
    ("atm", MKPointOfInterestCategory(rawValue: "MKPOICategoryATM")),
    ("automotiveRepair", MKPointOfInterestCategory(rawValue: "MKPOICategoryAutomotiveRepair")),
    ("bakery", MKPointOfInterestCategory(rawValue: "MKPOICategoryBakery")),
    ("bank", MKPointOfInterestCategory(rawValue: "MKPOICategoryBank")),
    ("baseball", MKPointOfInterestCategory(rawValue: "MKPOICategoryBaseball")),
    ("basketball", MKPointOfInterestCategory(rawValue: "MKPOICategoryBasketball")),
    ("beach", MKPointOfInterestCategory(rawValue: "MKPOICategoryBeach")),
    ("beauty", MKPointOfInterestCategory(rawValue: "MKPOICategoryBeauty")),
    ("bowling", MKPointOfInterestCategory(rawValue: "MKPOICategoryBowling")),
    ("brewery", MKPointOfInterestCategory(rawValue: "MKPOICategoryBrewery")),
    ("cafe", MKPointOfInterestCategory(rawValue: "MKPOICategoryCafe")),
    ("campground", MKPointOfInterestCategory(rawValue: "MKPOICategoryCampground")),
    ("carRental", MKPointOfInterestCategory(rawValue: "MKPOICategoryCarRental")),
    ("castle", MKPointOfInterestCategory(rawValue: "MKPOICategoryCastle")),
    ("conventionCenter", MKPointOfInterestCategory(rawValue: "MKPOICategoryConventionCenter")),
    ("distillery", MKPointOfInterestCategory(rawValue: "MKPOICategoryDistillery")),
    ("evCharger", MKPointOfInterestCategory(rawValue: "MKPOICategoryEVCharger")),
    ("fairground", MKPointOfInterestCategory(rawValue: "MKPOICategoryFairground")),
    ("fireStation", MKPointOfInterestCategory(rawValue: "MKPOICategoryFireStation")),
    ("fishing", MKPointOfInterestCategory(rawValue: "MKPOICategoryFishing")),
    ("fitnessCenter", MKPointOfInterestCategory(rawValue: "MKPOICategoryFitnessCenter")),
    ("foodMarket", MKPointOfInterestCategory(rawValue: "MKPOICategoryFoodMarket")),
    ("fortress", MKPointOfInterestCategory(rawValue: "MKPOICategoryFortress")),
    ("gasStation", MKPointOfInterestCategory(rawValue: "MKPOICategoryGasStation")),
    ("golf", MKPointOfInterestCategory(rawValue: "MKPOICategoryGolf")),
    ("goKart", MKPointOfInterestCategory(rawValue: "MKPOICategoryGoKart")),
    ("hiking", MKPointOfInterestCategory(rawValue: "MKPOICategoryHiking")),
    ("hospital", MKPointOfInterestCategory(rawValue: "MKPOICategoryHospital")),
    ("hotel", MKPointOfInterestCategory(rawValue: "MKPOICategoryHotel")),
    ("kayaking", MKPointOfInterestCategory(rawValue: "MKPOICategoryKayaking")),
    ("landmark", MKPointOfInterestCategory(rawValue: "MKPOICategoryLandmark")),
    ("laundry", MKPointOfInterestCategory(rawValue: "MKPOICategoryLaundry")),
    ("library", MKPointOfInterestCategory(rawValue: "MKPOICategoryLibrary")),
    ("mailbox", MKPointOfInterestCategory(rawValue: "MKPOICategoryMailbox")),
    ("marina", MKPointOfInterestCategory(rawValue: "MKPOICategoryMarina")),
    ("miniGolf", MKPointOfInterestCategory(rawValue: "MKPOICategoryMiniGolf")),
    ("movieTheater", MKPointOfInterestCategory(rawValue: "MKPOICategoryMovieTheater")),
    ("museum", MKPointOfInterestCategory(rawValue: "MKPOICategoryMuseum")),
    ("musicVenue", MKPointOfInterestCategory(rawValue: "MKPOICategoryMusicVenue")),
    ("nationalMonument", MKPointOfInterestCategory(rawValue: "MKPOICategoryNationalMonument")),
    ("nationalPark", MKPointOfInterestCategory(rawValue: "MKPOICategoryNationalPark")),
    ("nightlife", MKPointOfInterestCategory(rawValue: "MKPOICategoryNightlife")),
    ("park", MKPointOfInterestCategory(rawValue: "MKPOICategoryPark")),
    ("parking", MKPointOfInterestCategory(rawValue: "MKPOICategoryParking")),
    ("pharmacy", MKPointOfInterestCategory(rawValue: "MKPOICategoryPharmacy")),
    ("planetarium", MKPointOfInterestCategory(rawValue: "MKPOICategoryPlanetarium")),
    ("police", MKPointOfInterestCategory(rawValue: "MKPOICategoryPolice")),
    ("postOffice", MKPointOfInterestCategory(rawValue: "MKPOICategoryPostOffice")),
    ("publicTransport", MKPointOfInterestCategory(rawValue: "MKPOICategoryPublicTransport")),
    ("restaurant", MKPointOfInterestCategory(rawValue: "MKPOICategoryRestaurant")),
    ("restroom", MKPointOfInterestCategory(rawValue: "MKPOICategoryRestroom")),
    ("rockClimbing", MKPointOfInterestCategory(rawValue: "MKPOICategoryRockClimbing")),
    ("rvPark", MKPointOfInterestCategory(rawValue: "MKPOICategoryRVPark")),
    ("school", MKPointOfInterestCategory(rawValue: "MKPOICategorySchool")),
    ("skatePark", MKPointOfInterestCategory(rawValue: "MKPOICategorySkatePark")),
    ("skating", MKPointOfInterestCategory(rawValue: "MKPOICategorySkating")),
    ("skiing", MKPointOfInterestCategory(rawValue: "MKPOICategorySkiing")),
    ("soccer", MKPointOfInterestCategory(rawValue: "MKPOICategorySoccer")),
    ("spa", MKPointOfInterestCategory(rawValue: "MKPOICategorySpa")),
    ("stadium", MKPointOfInterestCategory(rawValue: "MKPOICategoryStadium")),
    ("store", MKPointOfInterestCategory(rawValue: "MKPOICategoryStore")),
    ("surfing", MKPointOfInterestCategory(rawValue: "MKPOICategorySurfing")),
    ("swimming", MKPointOfInterestCategory(rawValue: "MKPOICategorySwimming")),
    ("tennis", MKPointOfInterestCategory(rawValue: "MKPOICategoryTennis")),
    ("theater", MKPointOfInterestCategory(rawValue: "MKPOICategoryTheater")),
    ("university", MKPointOfInterestCategory(rawValue: "MKPOICategoryUniversity")),
    ("winery", MKPointOfInterestCategory(rawValue: "MKPOICategoryWinery")),
    ("volleyball", MKPointOfInterestCategory(rawValue: "MKPOICategoryVolleyball")),
    ("zoo", MKPointOfInterestCategory(rawValue: "MKPOICategoryZoo")),
]

func mkrPointOfInterestCategory(from value: String) -> MKPointOfInterestCategory {
    switch value.lowercased() {
    case "animalservice": return MKPointOfInterestCategory(rawValue: "MKPOICategoryAnimalService")
    case "airport": return MKPointOfInterestCategory(rawValue: "MKPOICategoryAirport")
    case "amusementpark": return MKPointOfInterestCategory(rawValue: "MKPOICategoryAmusementPark")
    case "aquarium": return MKPointOfInterestCategory(rawValue: "MKPOICategoryAquarium")
    case "atm": return MKPointOfInterestCategory(rawValue: "MKPOICategoryATM")
    case "automotiverepair": return MKPointOfInterestCategory(rawValue: "MKPOICategoryAutomotiveRepair")
    case "bakery": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBakery")
    case "bank": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBank")
    case "baseball": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBaseball")
    case "basketball": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBasketball")
    case "beach": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBeach")
    case "beauty": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBeauty")
    case "bowling": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBowling")
    case "brewery": return MKPointOfInterestCategory(rawValue: "MKPOICategoryBrewery")
    case "cafe": return MKPointOfInterestCategory(rawValue: "MKPOICategoryCafe")
    case "campground": return MKPointOfInterestCategory(rawValue: "MKPOICategoryCampground")
    case "carrental": return MKPointOfInterestCategory(rawValue: "MKPOICategoryCarRental")
    case "castle": return MKPointOfInterestCategory(rawValue: "MKPOICategoryCastle")
    case "conventioncenter": return MKPointOfInterestCategory(rawValue: "MKPOICategoryConventionCenter")
    case "distillery": return MKPointOfInterestCategory(rawValue: "MKPOICategoryDistillery")
    case "evcharger": return MKPointOfInterestCategory(rawValue: "MKPOICategoryEVCharger")
    case "fairground": return MKPointOfInterestCategory(rawValue: "MKPOICategoryFairground")
    case "firestation": return MKPointOfInterestCategory(rawValue: "MKPOICategoryFireStation")
    case "fishing": return MKPointOfInterestCategory(rawValue: "MKPOICategoryFishing")
    case "fitnesscenter": return MKPointOfInterestCategory(rawValue: "MKPOICategoryFitnessCenter")
    case "foodmarket": return MKPointOfInterestCategory(rawValue: "MKPOICategoryFoodMarket")
    case "fortress": return MKPointOfInterestCategory(rawValue: "MKPOICategoryFortress")
    case "gasstation": return MKPointOfInterestCategory(rawValue: "MKPOICategoryGasStation")
    case "golf": return MKPointOfInterestCategory(rawValue: "MKPOICategoryGolf")
    case "gokart": return MKPointOfInterestCategory(rawValue: "MKPOICategoryGoKart")
    case "hiking": return MKPointOfInterestCategory(rawValue: "MKPOICategoryHiking")
    case "hospital": return MKPointOfInterestCategory(rawValue: "MKPOICategoryHospital")
    case "hotel": return MKPointOfInterestCategory(rawValue: "MKPOICategoryHotel")
    case "kayaking": return MKPointOfInterestCategory(rawValue: "MKPOICategoryKayaking")
    case "landmark": return MKPointOfInterestCategory(rawValue: "MKPOICategoryLandmark")
    case "laundry": return MKPointOfInterestCategory(rawValue: "MKPOICategoryLaundry")
    case "library": return MKPointOfInterestCategory(rawValue: "MKPOICategoryLibrary")
    case "mailbox": return MKPointOfInterestCategory(rawValue: "MKPOICategoryMailbox")
    case "marina": return MKPointOfInterestCategory(rawValue: "MKPOICategoryMarina")
    case "minigolf": return MKPointOfInterestCategory(rawValue: "MKPOICategoryMiniGolf")
    case "movietheater": return MKPointOfInterestCategory(rawValue: "MKPOICategoryMovieTheater")
    case "museum": return MKPointOfInterestCategory(rawValue: "MKPOICategoryMuseum")
    case "musicvenue": return MKPointOfInterestCategory(rawValue: "MKPOICategoryMusicVenue")
    case "nationalmonument": return MKPointOfInterestCategory(rawValue: "MKPOICategoryNationalMonument")
    case "nationalpark": return MKPointOfInterestCategory(rawValue: "MKPOICategoryNationalPark")
    case "nightlife": return MKPointOfInterestCategory(rawValue: "MKPOICategoryNightlife")
    case "park": return MKPointOfInterestCategory(rawValue: "MKPOICategoryPark")
    case "parking": return MKPointOfInterestCategory(rawValue: "MKPOICategoryParking")
    case "pharmacy": return MKPointOfInterestCategory(rawValue: "MKPOICategoryPharmacy")
    case "planetarium": return MKPointOfInterestCategory(rawValue: "MKPOICategoryPlanetarium")
    case "police": return MKPointOfInterestCategory(rawValue: "MKPOICategoryPolice")
    case "postoffice": return MKPointOfInterestCategory(rawValue: "MKPOICategoryPostOffice")
    case "publictransport": return MKPointOfInterestCategory(rawValue: "MKPOICategoryPublicTransport")
    case "restaurant": return MKPointOfInterestCategory(rawValue: "MKPOICategoryRestaurant")
    case "restroom": return MKPointOfInterestCategory(rawValue: "MKPOICategoryRestroom")
    case "rockclimbing": return MKPointOfInterestCategory(rawValue: "MKPOICategoryRockClimbing")
    case "rvpark": return MKPointOfInterestCategory(rawValue: "MKPOICategoryRVPark")
    case "school": return MKPointOfInterestCategory(rawValue: "MKPOICategorySchool")
    case "skatepark": return MKPointOfInterestCategory(rawValue: "MKPOICategorySkatePark")
    case "skating": return MKPointOfInterestCategory(rawValue: "MKPOICategorySkating")
    case "skiing": return MKPointOfInterestCategory(rawValue: "MKPOICategorySkiing")
    case "soccer": return MKPointOfInterestCategory(rawValue: "MKPOICategorySoccer")
    case "spa": return MKPointOfInterestCategory(rawValue: "MKPOICategorySpa")
    case "stadium": return MKPointOfInterestCategory(rawValue: "MKPOICategoryStadium")
    case "store": return MKPointOfInterestCategory(rawValue: "MKPOICategoryStore")
    case "surfing": return MKPointOfInterestCategory(rawValue: "MKPOICategorySurfing")
    case "swimming": return MKPointOfInterestCategory(rawValue: "MKPOICategorySwimming")
    case "tennis": return MKPointOfInterestCategory(rawValue: "MKPOICategoryTennis")
    case "theater": return MKPointOfInterestCategory(rawValue: "MKPOICategoryTheater")
    case "university": return MKPointOfInterestCategory(rawValue: "MKPOICategoryUniversity")
    case "winery": return MKPointOfInterestCategory(rawValue: "MKPOICategoryWinery")
    case "volleyball": return MKPointOfInterestCategory(rawValue: "MKPOICategoryVolleyball")
    case "zoo": return MKPointOfInterestCategory(rawValue: "MKPOICategoryZoo")
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

func mkrEncodePointOfInterestFilter(
    _ filter: MKPointOfInterestFilter?
) -> MKRPointOfInterestFilterPayload? {
    guard let filter else { return nil }

    var included: [String] = []
    var excluded: [String] = []
    for (name, category) in mkrKnownPointOfInterestCategories {
        if filter.includes(category) {
            included.append(name)
        }
        if filter.excludes(category) {
            excluded.append(name)
        }
    }

    if included.count == mkrKnownPointOfInterestCategories.count {
        return MKRPointOfInterestFilterPayload(mode: .includingAll, categories: [])
    }
    if excluded.count == mkrKnownPointOfInterestCategories.count {
        return MKRPointOfInterestFilterPayload(mode: .excludingAll, categories: [])
    }
    if !included.isEmpty {
        return MKRPointOfInterestFilterPayload(mode: .including, categories: included)
    }
    if !excluded.isEmpty {
        return MKRPointOfInterestFilterPayload(mode: .excluding, categories: excluded)
    }
    return nil
}

@_cdecl("mk_points_of_interest_request_max_radius")
public func mk_points_of_interest_request_max_radius() -> Double {
    MKLocalPointsOfInterestRequest.maxRadius
}
