use crate::look_around::MKLookAroundScene;
use crate::point_of_interest::MKPointOfInterestFilter;

/// Wraps `MKLookAroundBadgePosition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MKLookAroundBadgePosition {
    TopLeading,
    TopTrailing,
    BottomTrailing,
}

/// Wraps `MKLookAroundViewControllerDelegate`.
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

/// Wraps `MKLookAroundViewController`.
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
    /// Creates a wrapper for `MKLookAroundViewController`.
    pub fn new(scene: MKLookAroundScene) -> Self {
        Self::with_optional_scene(Some(scene))
    }

    /// Wraps `MKLookAroundViewController.optionalScene`.
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

    /// Wraps `MKLookAroundViewController.scene`.
    pub const fn scene(&self) -> Option<&MKLookAroundScene> {
        self.scene.as_ref()
    }

    /// Wraps `MKLookAroundViewController.takeScene`.
    pub fn take_scene(&mut self) -> Option<MKLookAroundScene> {
        self.scene.take()
    }

    /// Wraps `MKLookAroundViewController.scene`.
    pub fn set_scene(&mut self, scene: Option<MKLookAroundScene>) {
        self.scene = scene;
    }

    /// Wraps `MKLookAroundViewController.updateScene`.
    pub fn update_scene<D: MKLookAroundViewControllerDelegate>(
        &mut self,
        scene: Option<MKLookAroundScene>,
        delegate: &mut D,
    ) {
        delegate.look_around_view_controller_will_update_scene(self);
        self.scene = scene;
        delegate.look_around_view_controller_did_update_scene(self);
    }

    /// Wraps `MKLookAroundViewController.isNavigationEnabled`.
    pub const fn is_navigation_enabled(&self) -> bool {
        self.navigation_enabled
    }

    /// Wraps `MKLookAroundViewController.navigationEnabled`.
    pub fn set_navigation_enabled(&mut self, navigation_enabled: bool) {
        self.navigation_enabled = navigation_enabled;
    }

    /// Wraps `MKLookAroundViewController.showsRoadLabels`.
    pub const fn shows_road_labels(&self) -> bool {
        self.shows_road_labels
    }

    /// Wraps `MKLookAroundViewController.showsRoadLabels`.
    pub fn set_shows_road_labels(&mut self, shows_road_labels: bool) {
        self.shows_road_labels = shows_road_labels;
    }

    /// Wraps `MKLookAroundViewController.pointOfInterestFilter`.
    pub fn point_of_interest_filter(&self) -> Option<&MKPointOfInterestFilter> {
        self.point_of_interest_filter.as_ref()
    }

    /// Wraps `MKLookAroundViewController.pointOfInterestFilter`.
    pub fn set_point_of_interest_filter(
        &mut self,
        point_of_interest_filter: Option<MKPointOfInterestFilter>,
    ) {
        self.point_of_interest_filter = point_of_interest_filter;
    }

    /// Wraps `MKLookAroundViewController.badgePosition`.
    pub const fn badge_position(&self) -> MKLookAroundBadgePosition {
        self.badge_position
    }

    /// Wraps `MKLookAroundViewController.badgePosition`.
    pub fn set_badge_position(&mut self, badge_position: MKLookAroundBadgePosition) {
        self.badge_position = badge_position;
    }

    /// Wraps `MKLookAroundViewController.isPresentingFullScreen`.
    pub const fn is_presenting_full_screen(&self) -> bool {
        self.presenting_full_screen
    }

    /// Wraps `MKLookAroundViewController.presentFullScreen`.
    pub fn present_full_screen<D: MKLookAroundViewControllerDelegate>(&mut self, delegate: &mut D) {
        delegate.look_around_view_controller_will_present_full_screen(self);
        self.presenting_full_screen = true;
        delegate.look_around_view_controller_did_present_full_screen(self);
    }

    /// Wraps `MKLookAroundViewController.dismissFullScreen`.
    pub fn dismiss_full_screen<D: MKLookAroundViewControllerDelegate>(&mut self, delegate: &mut D) {
        delegate.look_around_view_controller_will_dismiss_full_screen(self);
        self.presenting_full_screen = false;
        delegate.look_around_view_controller_did_dismiss_full_screen(self);
    }
}
