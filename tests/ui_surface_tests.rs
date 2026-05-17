use mapkit::prelude::*;

struct DetailDelegate {
    finished: bool,
}

impl MKMapItemDetailViewControllerDelegate for DetailDelegate {
    fn map_item_detail_view_controller_did_finish(
        &mut self,
        _detail_view_controller: &MKMapItemDetailViewController,
    ) {
        self.finished = true;
    }
}

struct LookAroundDelegate {
    updates: usize,
    presentations: usize,
    dismissals: usize,
}

impl MKLookAroundViewControllerDelegate for LookAroundDelegate {
    fn look_around_view_controller_will_update_scene(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
        self.updates += 1;
    }

    fn look_around_view_controller_did_update_scene(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
        self.updates += 1;
    }

    fn look_around_view_controller_did_present_full_screen(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
        self.presentations += 1;
    }

    fn look_around_view_controller_did_dismiss_full_screen(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
        self.dismissals += 1;
    }
}

struct DummyMapViewDelegate;

impl MKMapViewDelegate for DummyMapViewDelegate {}

const fn accept_map_view_delegate<D: MKMapViewDelegate>(_delegate: &D) {}

#[test]
fn ui_surface_wrappers_build() {
    let mut compass = MKCompassButton::new(None);
    compass.set_compass_visibility(MKFeatureVisibility::Visible);
    assert_eq!(compass.compass_visibility(), MKFeatureVisibility::Visible);
    assert!(compass.shows_compass().is_err());

    let pitch = MKPitchControl::new(None);
    assert!(pitch.is_pitch_enabled().is_err());

    let zoom = MKZoomControl::new(None);
    assert!(zoom.shows_zoom_controls().is_err());

    let presentation = MKMapItemDetailSelectionAccessoryPresentationStyle::callout_with_callout_style(
        MKMapItemDetailSelectionAccessoryCalloutStyle::Compact,
    );
    assert_eq!(
        presentation.kind(),
        MKMapItemDetailSelectionAccessoryPresentationKind::Callout
    );
    assert_eq!(
        presentation.callout_style(),
        MKMapItemDetailSelectionAccessoryCalloutStyle::Compact
    );
    let accessory = MKSelectionAccessory::map_item_detail(presentation.clone());
    assert_eq!(accessory.presentation_style(), &presentation);

    let item = MKMapItem::new(MKPlacemark::new(MKCoordinate::new(37.3349, -122.0090)));
    let detail_controller = MKMapItemDetailViewController::with_map_item(Some(item), false);
    assert!(!detail_controller.displays_map());
    let mut detail_delegate = DetailDelegate { finished: false };
    detail_controller.finish(&mut detail_delegate);
    assert!(detail_delegate.finished);
    let _: fn(Option<MKMapItem>) -> MKMapItemDetailViewController =
        MKMapItemDetailViewController::new;

    let mut look_around_controller = MKLookAroundViewController::with_optional_scene(None);
    look_around_controller.set_badge_position(MKLookAroundBadgePosition::BottomTrailing);
    look_around_controller.set_navigation_enabled(false);
    look_around_controller.set_shows_road_labels(false);
    look_around_controller
        .set_point_of_interest_filter(Some(MKPointOfInterestFilter::including_all()));
    assert_eq!(
        look_around_controller.badge_position(),
        MKLookAroundBadgePosition::BottomTrailing
    );
    assert!(!look_around_controller.is_navigation_enabled());
    assert!(!look_around_controller.shows_road_labels());
    assert!(look_around_controller.point_of_interest_filter().is_some());

    let mut look_around_delegate = LookAroundDelegate {
        updates: 0,
        presentations: 0,
        dismissals: 0,
    };
    look_around_controller.update_scene(None, &mut look_around_delegate);
    look_around_controller.present_full_screen(&mut look_around_delegate);
    look_around_controller.dismiss_full_screen(&mut look_around_delegate);
    assert_eq!(look_around_delegate.updates, 2);
    assert_eq!(look_around_delegate.presentations, 1);
    assert_eq!(look_around_delegate.dismissals, 1);
    let _: fn(MKLookAroundScene) -> MKLookAroundViewController =
        MKLookAroundViewController::new;

    accept_map_view_delegate(&DummyMapViewDelegate);
}
