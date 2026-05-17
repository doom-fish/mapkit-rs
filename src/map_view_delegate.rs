use crate::annotation::MKUserLocation;
use crate::annotation_view::{
    MKAnnotation, MKAnnotationView, MKAnnotationViewDragState,
};
use crate::cluster_annotation::MKClusterAnnotation;
use crate::error::MapKitError;
use crate::map_view::{MKMapView, MKUserTrackingMode};
use crate::overlay::MKOverlay;
use crate::overlay_renderer::MKOverlayRenderer;
use crate::selection_accessory::MKSelectionAccessory;

pub trait MKMapViewDelegate {
    fn map_view_region_will_change_animated(&mut self, _map_view: &MKMapView, _animated: bool) {}

    fn map_view_region_did_change_animated(&mut self, _map_view: &MKMapView, _animated: bool) {}

    fn map_view_did_change_visible_region(&mut self, _map_view: &MKMapView) {}

    fn map_view_will_start_loading_map(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_finish_loading_map(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_fail_loading_map(
        &mut self,
        _map_view: &MKMapView,
        _error: &MapKitError,
    ) {
    }

    fn map_view_will_start_rendering_map(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_finish_rendering_map(
        &mut self,
        _map_view: &MKMapView,
        _fully_rendered: bool,
    ) {
    }

    fn map_view_view_for_annotation(
        &mut self,
        _map_view: &MKMapView,
        _annotation: &dyn MKAnnotation,
    ) -> Option<MKAnnotationView> {
        None
    }

    fn map_view_did_add_annotation_views(
        &mut self,
        _map_view: &MKMapView,
        _views: &[MKAnnotationView],
    ) {
    }

    fn map_view_did_select_annotation_view(
        &mut self,
        _map_view: &MKMapView,
        _view: &MKAnnotationView,
    ) {
    }

    fn map_view_did_deselect_annotation_view(
        &mut self,
        _map_view: &MKMapView,
        _view: &MKAnnotationView,
    ) {
    }

    fn map_view_selection_accessory_for_annotation(
        &mut self,
        _map_view: &MKMapView,
        _annotation: &dyn MKAnnotation,
    ) -> Option<MKSelectionAccessory> {
        None
    }

    fn map_view_will_start_locating_user(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_stop_locating_user(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_update_user_location(
        &mut self,
        _map_view: &MKMapView,
        _user_location: &MKUserLocation,
    ) {
    }

    fn map_view_did_fail_to_locate_user_with_error(
        &mut self,
        _map_view: &MKMapView,
        _error: &MapKitError,
    ) {
    }

    fn map_view_annotation_view_did_change_drag_state(
        &mut self,
        _map_view: &MKMapView,
        _view: &MKAnnotationView,
        _new_state: MKAnnotationViewDragState,
        _old_state: MKAnnotationViewDragState,
    ) {
    }

    fn map_view_did_change_user_tracking_mode(
        &mut self,
        _map_view: &MKMapView,
        _mode: MKUserTrackingMode,
        _animated: bool,
    ) {
    }

    fn map_view_renderer_for_overlay(
        &mut self,
        _map_view: &MKMapView,
        _overlay: &dyn MKOverlay,
    ) -> Option<MKOverlayRenderer> {
        None
    }

    fn map_view_did_add_overlay_renderers(
        &mut self,
        _map_view: &MKMapView,
        _renderers: &[MKOverlayRenderer],
    ) {
    }

    fn map_view_cluster_annotation_for_member_annotations(
        &mut self,
        _map_view: &MKMapView,
        _member_annotations: &[&dyn MKAnnotation],
    ) -> Option<MKClusterAnnotation> {
        None
    }
}
