use crate::map_item::MKMapItem;

pub trait MKMapItemDetailViewControllerDelegate {
    fn map_item_detail_view_controller_did_finish(
        &mut self,
        _detail_view_controller: &MKMapItemDetailViewController,
    ) {
    }
}

#[derive(Debug, Clone)]
pub struct MKMapItemDetailViewController {
    map_item: Option<MKMapItem>,
    displays_map: bool,
}

impl MKMapItemDetailViewController {
    pub fn new(map_item: Option<MKMapItem>) -> Self {
        Self::with_map_item(map_item, true)
    }

    pub const fn with_map_item(map_item: Option<MKMapItem>, displays_map: bool) -> Self {
        Self {
            map_item,
            displays_map,
        }
    }

    pub const fn map_item(&self) -> Option<&MKMapItem> {
        self.map_item.as_ref()
    }

    pub fn set_map_item(&mut self, map_item: Option<MKMapItem>) {
        self.map_item = map_item;
    }

    pub const fn displays_map(&self) -> bool {
        self.displays_map
    }

    pub fn finish<D: MKMapItemDetailViewControllerDelegate>(&self, delegate: &mut D) {
        delegate.map_item_detail_view_controller_did_finish(self);
    }
}
