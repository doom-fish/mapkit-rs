use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

/// Wraps `MKAddressFilterOption`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKAddressFilterOption(pub u64);

impl MKAddressFilterOption {
    /// Wraps `MKAddressFilterOption.country`.
    pub const COUNTRY: Self = Self(1 << 0);
    /// Wraps `MKAddressFilterOption.administrativeArea`.
    pub const ADMINISTRATIVE_AREA: Self = Self(1 << 1);
    /// Wraps `MKAddressFilterOption.subAdministrativeArea`.
    pub const SUB_ADMINISTRATIVE_AREA: Self = Self(1 << 2);
    /// Wraps `MKAddressFilterOption.locality`.
    pub const LOCALITY: Self = Self(1 << 3);
    /// Wraps `MKAddressFilterOption.subLocality`.
    pub const SUB_LOCALITY: Self = Self(1 << 4);
    /// Wraps `MKAddressFilterOption.postalCode`.
    pub const POSTAL_CODE: Self = Self(1 << 5);
    /// Wraps `MKAddressFilterOption.all`.
    pub const ALL: Self = Self(
        Self::COUNTRY.0
            | Self::ADMINISTRATIVE_AREA.0
            | Self::SUB_ADMINISTRATIVE_AREA.0
            | Self::LOCALITY.0
            | Self::SUB_LOCALITY.0
            | Self::POSTAL_CODE.0,
    );

    /// Wraps `MKAddressFilterOption.bits`.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps `MKAddressFilterOption.contains`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for MKAddressFilterOption {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MKAddressFilterOption {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Wraps `MKAddressFilterMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAddressFilterMode {
    Including,
    Excluding,
    IncludingAll,
    ExcludingAll,
}

/// Wraps `MKAddressFilter`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKAddressFilter {
    /// Wraps `MKAddressFilter.mode`.
    pub mode: MKAddressFilterMode,
    /// Wraps `MKAddressFilter.options`.
    pub options: MKAddressFilterOption,
}

impl MKAddressFilter {
    /// Wraps `MKAddressFilter.includingAll`.
    pub const fn including_all() -> Self {
        Self {
            mode: MKAddressFilterMode::IncludingAll,
            options: MKAddressFilterOption::ALL,
        }
    }

    /// Wraps `MKAddressFilter.excludingAll`.
    pub const fn excluding_all() -> Self {
        Self {
            mode: MKAddressFilterMode::ExcludingAll,
            options: MKAddressFilterOption::ALL,
        }
    }

    /// Wraps `MKAddressFilter.including`.
    pub const fn including(options: MKAddressFilterOption) -> Self {
        Self {
            mode: MKAddressFilterMode::Including,
            options,
        }
    }

    /// Wraps `MKAddressFilter.excluding`.
    pub const fn excluding(options: MKAddressFilterOption) -> Self {
        Self {
            mode: MKAddressFilterMode::Excluding,
            options,
        }
    }

    /// Wraps `MKAddressFilter.includesOptions`.
    pub const fn includes_options(&self, options: MKAddressFilterOption) -> bool {
        match self.mode {
            MKAddressFilterMode::Including => self.options.contains(options),
            MKAddressFilterMode::IncludingAll => true,
            MKAddressFilterMode::Excluding | MKAddressFilterMode::ExcludingAll => false,
        }
    }

    /// Wraps `MKAddressFilter.excludesOptions`.
    pub const fn excludes_options(&self, options: MKAddressFilterOption) -> bool {
        match self.mode {
            MKAddressFilterMode::Excluding => self.options.contains(options),
            MKAddressFilterMode::ExcludingAll => true,
            MKAddressFilterMode::Including | MKAddressFilterMode::IncludingAll => false,
        }
    }
}

/// Wraps `MKAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKAddress {
    /// Wraps `MKAddress.fullAddress`.
    pub full_address: String,
    /// Wraps `MKAddress.shortAddress`.
    pub short_address: Option<String>,
}

impl MKAddress {
    /// Creates a wrapper for `MKAddress`.
    pub fn new(full_address: impl Into<String>, short_address: Option<impl Into<String>>) -> Self {
        Self {
            full_address: full_address.into(),
            short_address: short_address.map(Into::into),
        }
    }
}

/// Wraps `MKAddressRepresentationsContextStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAddressRepresentationsContextStyle {
    Automatic,
    Short,
    Full,
}

/// Wraps `MKAddressRepresentations`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKAddressRepresentations {
    /// Wraps `MKAddressRepresentations.cityName`.
    pub city_name: Option<String>,
    /// Wraps `MKAddressRepresentations.cityWithContext`.
    pub city_with_context: Option<String>,
    /// Wraps `MKAddressRepresentations.cityWithContextShort`.
    pub city_with_context_short: Option<String>,
    /// Wraps `MKAddressRepresentations.cityWithContextFull`.
    pub city_with_context_full: Option<String>,
    /// Wraps `MKAddressRepresentations.regionName`.
    pub region_name: Option<String>,
    /// Wraps `MKAddressRepresentations.regionCode`.
    pub region_code: Option<String>,
    /// Wraps `MKAddressRepresentations.fullAddressIncludingRegionMultiline`.
    pub full_address_including_region_multiline: Option<String>,
    /// Wraps `MKAddressRepresentations.fullAddressIncludingRegionSingleLine`.
    pub full_address_including_region_single_line: Option<String>,
    /// Wraps `MKAddressRepresentations.fullAddressExcludingRegionMultiline`.
    pub full_address_excluding_region_multiline: Option<String>,
    /// Wraps `MKAddressRepresentations.fullAddressExcludingRegionSingleLine`.
    pub full_address_excluding_region_single_line: Option<String>,
}

impl MKAddressRepresentations {
    /// Wraps `MKAddressRepresentations.fullAddress`.
    pub fn full_address(&self, including_region: bool, single_line: bool) -> Option<&str> {
        match (including_region, single_line) {
            (true, true) => self.full_address_including_region_single_line.as_deref(),
            (true, false) => self.full_address_including_region_multiline.as_deref(),
            (false, true) => self.full_address_excluding_region_single_line.as_deref(),
            (false, false) => self.full_address_excluding_region_multiline.as_deref(),
        }
    }

    /// Wraps `MKAddressRepresentations.cityWithContextUsingStyle`.
    pub fn city_with_context_using_style(
        &self,
        style: MKAddressRepresentationsContextStyle,
    ) -> Option<&str> {
        match style {
            MKAddressRepresentationsContextStyle::Automatic => self.city_with_context.as_deref(),
            MKAddressRepresentationsContextStyle::Short => self.city_with_context_short.as_deref(),
            MKAddressRepresentationsContextStyle::Full => self.city_with_context_full.as_deref(),
        }
    }
}
