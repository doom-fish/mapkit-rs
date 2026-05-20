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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn automatic_style_uses_automatic_defaults() {
        let style = MKMapItemDetailSelectionAccessoryPresentationStyle::automatic();

        assert_eq!(
            style.kind(),
            MKMapItemDetailSelectionAccessoryPresentationKind::Automatic
        );
        assert_eq!(
            style.callout_style(),
            MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic
        );
        assert!(!style.has_presenting_view_controller());
    }

    #[test]
    fn automatic_style_can_mark_presenting_view_controller() {
        let style =
            MKMapItemDetailSelectionAccessoryPresentationStyle::automatic_with_presenting_view_controller();

        assert_eq!(
            style.kind(),
            MKMapItemDetailSelectionAccessoryPresentationKind::Automatic
        );
        assert!(style.has_presenting_view_controller());
    }

    #[test]
    fn callout_style_preserves_custom_callout_variant() {
        let style = MKMapItemDetailSelectionAccessoryPresentationStyle::callout_with_callout_style(
            MKMapItemDetailSelectionAccessoryCalloutStyle::Compact,
        );

        assert_eq!(
            style.kind(),
            MKMapItemDetailSelectionAccessoryPresentationKind::Callout
        );
        assert_eq!(
            style.callout_style(),
            MKMapItemDetailSelectionAccessoryCalloutStyle::Compact
        );
        assert!(!style.has_presenting_view_controller());
    }

    #[test]
    fn default_callout_style_is_automatic() {
        let style = MKMapItemDetailSelectionAccessoryPresentationStyle::callout();

        assert_eq!(
            style.callout_style(),
            MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic
        );
    }

    #[test]
    fn sheet_style_requires_a_presenting_view_controller() {
        let style = MKMapItemDetailSelectionAccessoryPresentationStyle::sheet();

        assert_eq!(
            style.kind(),
            MKMapItemDetailSelectionAccessoryPresentationKind::Sheet
        );
        assert!(style.has_presenting_view_controller());
    }

    #[test]
    fn open_in_maps_style_uses_expected_kind() {
        let style = MKMapItemDetailSelectionAccessoryPresentationStyle::open_in_maps();

        assert_eq!(
            style.kind(),
            MKMapItemDetailSelectionAccessoryPresentationKind::OpenInMaps
        );
        assert_eq!(
            style.callout_style(),
            MKMapItemDetailSelectionAccessoryCalloutStyle::Automatic
        );
        assert!(!style.has_presenting_view_controller());
    }

    #[test]
    fn selection_accessory_wraps_presentation_style() {
        let presentation_style = MKMapItemDetailSelectionAccessoryPresentationStyle::sheet();
        let accessory = MKSelectionAccessory::map_item_detail(presentation_style.clone());

        assert_eq!(accessory.presentation_style(), &presentation_style);
    }

    #[test]
    fn presentation_style_round_trips_through_json() {
        let style = MKMapItemDetailSelectionAccessoryPresentationStyle::callout_with_callout_style(
            MKMapItemDetailSelectionAccessoryCalloutStyle::Compact,
        );

        let value = serde_json::to_value(style.clone()).unwrap();

        assert_eq!(
            value,
            json!({
                "kind": "callout",
                "calloutStyle": "compact",
                "hasPresentingViewController": false,
            })
        );

        let decoded: MKMapItemDetailSelectionAccessoryPresentationStyle =
            serde_json::from_value(value).unwrap();

        assert_eq!(decoded, style);
    }
}
