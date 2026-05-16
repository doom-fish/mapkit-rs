import Foundation
import MapKit

struct MKRUserTrackingButtonStatePayload: Codable {
    var visible: Bool
    var trackingMode: MKRUserTrackingModePayload
}

@_cdecl("mk_user_tracking_button_state_json")
public func mk_user_tracking_button_state_json(
    _ mapView: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return nil
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrSyncOnMain {
            let visible: Bool
            if #available(macOS 14.0, *) {
                visible = view.showsUserTrackingButton
            } else {
                visible = false
            }
            let trackingMode: MKRUserTrackingModePayload
            if #available(macOS 11.0, *) {
                trackingMode = mkrEncodeUserTrackingMode(view.userTrackingMode)
            } else {
                trackingMode = .none
            }
            return MKRUserTrackingButtonStatePayload(visible: visible, trackingMode: trackingMode)
        }
        return mkrCString(try mkrEncodeJSON(payload))
    } catch {
        mkrSetError(outError, error)
        return nil
    }
}

@_cdecl("mk_user_tracking_button_set_visible")
public func mk_user_tracking_button_set_visible(
    _ mapView: UnsafeMutableRawPointer?,
    _ visible: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        try mkrSyncOnMain {
            guard #available(macOS 14.0, *) else {
                throw NSError(
                    domain: "mapkit-rs",
                    code: -1,
                    userInfo: [NSLocalizedDescriptionKey: "showsUserTrackingButton requires macOS 14.0+"]
                )
            }
            view.showsUserTrackingButton = visible
        }
    } catch {
        mkrSetError(outError, error)
    }
}

@_cdecl("mk_user_tracking_button_set_tracking_mode")
public func mk_user_tracking_button_set_tracking_mode(
    _ mapView: UnsafeMutableRawPointer?,
    _ modeJSON: UnsafePointer<CChar>?,
    _ animated: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let mapView else {
        mkrSetMessageError(outError, message: "missing MKMapView")
        return
    }

    do {
        let view = mkrBorrow(mapView, as: MKMapView.self)
        let payload = try mkrDecodeJSON(modeJSON, as: MKRUserTrackingModePayload.self)
        try mkrSyncOnMain {
            guard #available(macOS 11.0, *) else {
                throw NSError(
                    domain: "mapkit-rs",
                    code: -1,
                    userInfo: [NSLocalizedDescriptionKey: "userTrackingMode requires macOS 11.0+"]
                )
            }
            view.setUserTrackingMode(mkrUserTrackingMode(from: payload), animated: animated)
        }
    } catch {
        mkrSetError(outError, error)
    }
}
