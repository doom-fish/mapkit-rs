use crate::map_item::MKMapItem;

/// Wraps `MKMapItemDetailViewControllerDelegate`.
pub trait MKMapItemDetailViewControllerDelegate {
    fn map_item_detail_view_controller_did_finish(
        &mut self,
        _detail_view_controller: &MKMapItemDetailViewController,
    ) {
    }
}

/// Wraps `MKMapItemDetailViewController`.
#[derive(Debug, Clone)]
pub struct MKMapItemDetailViewController {
    map_item: Option<MKMapItem>,
    displays_map: bool,
}

impl MKMapItemDetailViewController {
    /// Creates a wrapper for `MKMapItemDetailViewController`.
    pub fn new(map_item: Option<MKMapItem>) -> Self {
        Self::with_map_item(map_item, true)
    }

    /// Wraps `MKMapItemDetailViewController.mapItem`.
    pub const fn with_map_item(map_item: Option<MKMapItem>, displays_map: bool) -> Self {
        Self {
            map_item,
            displays_map,
        }
    }

    /// Wraps `MKMapItemDetailViewController.mapItem`.
    pub const fn map_item(&self) -> Option<&MKMapItem> {
        self.map_item.as_ref()
    }

    /// Wraps `MKMapItemDetailViewController.mapItem`.
    pub fn set_map_item(&mut self, map_item: Option<MKMapItem>) {
        self.map_item = map_item;
    }

    /// Wraps `MKMapItemDetailViewController.displaysMap`.
    pub const fn displays_map(&self) -> bool {
        self.displays_map
    }

    /// Wraps `MKMapItemDetailViewController.finish`.
    pub fn finish<D: MKMapItemDetailViewControllerDelegate>(&self, delegate: &mut D) {
        delegate.map_item_detail_view_controller_did_finish(self);
    }
}
