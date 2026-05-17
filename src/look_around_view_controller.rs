use crate::look_around::MKLookAroundScene;
use crate::point_of_interest::MKPointOfInterestFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MKLookAroundBadgePosition {
    TopLeading,
    TopTrailing,
    BottomTrailing,
}

pub trait MKLookAroundViewControllerDelegate {
    fn look_around_view_controller_will_update_scene(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
    }

    fn look_around_view_controller_did_update_scene(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
    }

    fn look_around_view_controller_will_present_full_screen(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
    }

    fn look_around_view_controller_did_present_full_screen(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
    }

    fn look_around_view_controller_will_dismiss_full_screen(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
    }

    fn look_around_view_controller_did_dismiss_full_screen(
        &mut self,
        _view_controller: &MKLookAroundViewController,
    ) {
    }
}

#[derive(Debug)]
pub struct MKLookAroundViewController {
    scene: Option<MKLookAroundScene>,
    navigation_enabled: bool,
    shows_road_labels: bool,
    point_of_interest_filter: Option<MKPointOfInterestFilter>,
    badge_position: MKLookAroundBadgePosition,
    presenting_full_screen: bool,
}

impl MKLookAroundViewController {
    pub fn new(scene: MKLookAroundScene) -> Self {
        Self::with_optional_scene(Some(scene))
    }

    pub fn with_optional_scene(scene: Option<MKLookAroundScene>) -> Self {
        Self {
            scene,
            navigation_enabled: true,
            shows_road_labels: true,
            point_of_interest_filter: None,
            badge_position: MKLookAroundBadgePosition::TopLeading,
            presenting_full_screen: false,
        }
    }

    pub const fn scene(&self) -> Option<&MKLookAroundScene> {
        self.scene.as_ref()
    }

    pub fn take_scene(&mut self) -> Option<MKLookAroundScene> {
        self.scene.take()
    }

    pub fn set_scene(&mut self, scene: Option<MKLookAroundScene>) {
        self.scene = scene;
    }

    pub fn update_scene<D: MKLookAroundViewControllerDelegate>(
        &mut self,
        scene: Option<MKLookAroundScene>,
        delegate: &mut D,
    ) {
        delegate.look_around_view_controller_will_update_scene(self);
        self.scene = scene;
        delegate.look_around_view_controller_did_update_scene(self);
    }

    pub const fn is_navigation_enabled(&self) -> bool {
        self.navigation_enabled
    }

    pub fn set_navigation_enabled(&mut self, navigation_enabled: bool) {
        self.navigation_enabled = navigation_enabled;
    }

    pub const fn shows_road_labels(&self) -> bool {
        self.shows_road_labels
    }

    pub fn set_shows_road_labels(&mut self, shows_road_labels: bool) {
        self.shows_road_labels = shows_road_labels;
    }

    pub fn point_of_interest_filter(&self) -> Option<&MKPointOfInterestFilter> {
        self.point_of_interest_filter.as_ref()
    }

    pub fn set_point_of_interest_filter(
        &mut self,
        point_of_interest_filter: Option<MKPointOfInterestFilter>,
    ) {
        self.point_of_interest_filter = point_of_interest_filter;
    }

    pub const fn badge_position(&self) -> MKLookAroundBadgePosition {
        self.badge_position
    }

    pub fn set_badge_position(&mut self, badge_position: MKLookAroundBadgePosition) {
        self.badge_position = badge_position;
    }

    pub const fn is_presenting_full_screen(&self) -> bool {
        self.presenting_full_screen
    }

    pub fn present_full_screen<D: MKLookAroundViewControllerDelegate>(&mut self, delegate: &mut D) {
        delegate.look_around_view_controller_will_present_full_screen(self);
        self.presenting_full_screen = true;
        delegate.look_around_view_controller_did_present_full_screen(self);
    }

    pub fn dismiss_full_screen<D: MKLookAroundViewControllerDelegate>(&mut self, delegate: &mut D) {
        delegate.look_around_view_controller_will_dismiss_full_screen(self);
        self.presenting_full_screen = false;
        delegate.look_around_view_controller_did_dismiss_full_screen(self);
    }
}
