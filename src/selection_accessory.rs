use serde::{Deserialize, Serialize};

/// Wraps `MKMapItemDetailSelectionAccessoryCalloutStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKMapItemDetailSelectionAccessoryCalloutStyle {
    Automatic,
    Full,
    Compact,
}

/// Wraps `MKMapItemDetailSelectionAccessoryPresentationKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKMapItemDetailSelectionAccessoryPresentationKind {
    Automatic,
    Callout,
    Sheet,
    OpenInMaps,
}

/// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapItemDetailSelectionAccessoryPresentationStyle {
    kind: MKMapItemDetailSelectionAccessoryPresentationKind,
    callout_style: MKMapItemDetailSelectionAccessoryCalloutStyle,
    has_presenting_view_controller: bool,
}

impl MKMapItemDetailSelectionAccessoryPresentationStyle {
    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.automatic`.
    pub const fn automatic() -> Self {
        Self {
            kind: MKMapItemDetailSelectionAccessoryPresentationKind::Automatic,
            callout_style: MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic,
            has_presenting_view_controller: false,
        }
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.automaticWithPresentingViewController`.
    pub const fn automatic_with_presenting_view_controller() -> Self {
        Self {
            kind: MKMapItemDetailSelectionAccessoryPresentationKind::Automatic,
            callout_style: MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic,
            has_presenting_view_controller: true,
        }
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.callout`.
    pub const fn callout() -> Self {
        Self::callout_with_callout_style(MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic)
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.calloutWithCalloutStyle`.
    pub const fn callout_with_callout_style(
        callout_style: MKMapItemDetailSelectionAccessoryCalloutStyle,
    ) -> Self {
        Self {
            kind: MKMapItemDetailSelectionAccessoryPresentationKind::Callout,
            callout_style,
            has_presenting_view_controller: false,
        }
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.sheet`.
    pub const fn sheet() -> Self {
        Self {
            kind: MKMapItemDetailSelectionAccessoryPresentationKind::Sheet,
            callout_style: MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic,
            has_presenting_view_controller: true,
        }
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.openInMaps`.
    pub const fn open_in_maps() -> Self {
        Self {
            kind: MKMapItemDetailSelectionAccessoryPresentationKind::OpenInMaps,
            callout_style: MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic,
            has_presenting_view_controller: false,
        }
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.kind`.
    pub const fn kind(&self) -> MKMapItemDetailSelectionAccessoryPresentationKind {
        self.kind
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.calloutStyle`.
    pub const fn callout_style(&self) -> MKMapItemDetailSelectionAccessoryCalloutStyle {
        self.callout_style
    }

    /// Wraps `MKMapItemDetailSelectionAccessoryPresentationStyle.hasPresentingViewController`.
    pub const fn has_presenting_view_controller(&self) -> bool {
        self.has_presenting_view_controller
    }
}

/// Wraps `MKSelectionAccessory`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKSelectionAccessory {
    presentation_style: MKMapItemDetailSelectionAccessoryPresentationStyle,
}

impl MKSelectionAccessory {
    /// Wraps `MKSelectionAccessory.mapItemDetail`.
    pub const fn map_item_detail(
        presentation_style: MKMapItemDetailSelectionAccessoryPresentationStyle,
    ) -> Self {
        Self { presentation_style }
    }

    /// Wraps `MKSelectionAccessory.presentationStyle`.
    pub const fn presentation_style(&self) -> &MKMapItemDetailSelectionAccessoryPresentationStyle {
        &self.presentation_style
    }
}
