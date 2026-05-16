use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKAddressFilterOption(pub u64);

impl MKAddressFilterOption {
    pub const COUNTRY: Self = Self(1 << 0);
    pub const ADMINISTRATIVE_AREA: Self = Self(1 << 1);
    pub const SUB_ADMINISTRATIVE_AREA: Self = Self(1 << 2);
    pub const LOCALITY: Self = Self(1 << 3);
    pub const SUB_LOCALITY: Self = Self(1 << 4);
    pub const POSTAL_CODE: Self = Self(1 << 5);
    pub const ALL: Self = Self(
        Self::COUNTRY.0
            | Self::ADMINISTRATIVE_AREA.0
            | Self::SUB_ADMINISTRATIVE_AREA.0
            | Self::LOCALITY.0
            | Self::SUB_LOCALITY.0
            | Self::POSTAL_CODE.0,
    );

    pub const fn bits(self) -> u64 {
        self.0
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAddressFilterMode {
    Including,
    Excluding,
    IncludingAll,
    ExcludingAll,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKAddressFilter {
    pub mode: MKAddressFilterMode,
    pub options: MKAddressFilterOption,
}

impl MKAddressFilter {
    pub const fn including_all() -> Self {
        Self {
            mode: MKAddressFilterMode::IncludingAll,
            options: MKAddressFilterOption::ALL,
        }
    }

    pub const fn excluding_all() -> Self {
        Self {
            mode: MKAddressFilterMode::ExcludingAll,
            options: MKAddressFilterOption::ALL,
        }
    }

    pub const fn including(options: MKAddressFilterOption) -> Self {
        Self {
            mode: MKAddressFilterMode::Including,
            options,
        }
    }

    pub const fn excluding(options: MKAddressFilterOption) -> Self {
        Self {
            mode: MKAddressFilterMode::Excluding,
            options,
        }
    }

    pub const fn includes_options(&self, options: MKAddressFilterOption) -> bool {
        match self.mode {
            MKAddressFilterMode::Including => self.options.contains(options),
            MKAddressFilterMode::IncludingAll => true,
            MKAddressFilterMode::Excluding | MKAddressFilterMode::ExcludingAll => false,
        }
    }

    pub const fn excludes_options(&self, options: MKAddressFilterOption) -> bool {
        match self.mode {
            MKAddressFilterMode::Excluding => self.options.contains(options),
            MKAddressFilterMode::ExcludingAll => true,
            MKAddressFilterMode::Including | MKAddressFilterMode::IncludingAll => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKAddress {
    pub full_address: String,
    pub short_address: Option<String>,
}

impl MKAddress {
    pub fn new(full_address: impl Into<String>, short_address: Option<impl Into<String>>) -> Self {
        Self {
            full_address: full_address.into(),
            short_address: short_address.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAddressRepresentationsContextStyle {
    Automatic,
    Short,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKAddressRepresentations {
    pub city_name: Option<String>,
    pub city_with_context: Option<String>,
    pub city_with_context_short: Option<String>,
    pub city_with_context_full: Option<String>,
    pub region_name: Option<String>,
    pub region_code: Option<String>,
    pub full_address_including_region_multiline: Option<String>,
    pub full_address_including_region_single_line: Option<String>,
    pub full_address_excluding_region_multiline: Option<String>,
    pub full_address_excluding_region_single_line: Option<String>,
}

impl MKAddressRepresentations {
    pub fn full_address(&self, including_region: bool, single_line: bool) -> Option<&str> {
        match (including_region, single_line) {
            (true, true) => self.full_address_including_region_single_line.as_deref(),
            (true, false) => self.full_address_including_region_multiline.as_deref(),
            (false, true) => self.full_address_excluding_region_single_line.as_deref(),
            (false, false) => self.full_address_excluding_region_multiline.as_deref(),
        }
    }

    pub fn city_with_context_using_style(
        &self,
        style: MKAddressRepresentationsContextStyle,
    ) -> Option<&str> {
        match style {
            MKAddressRepresentationsContextStyle::Automatic => {
                self.city_with_context.as_deref()
            }
            MKAddressRepresentationsContextStyle::Short => {
                self.city_with_context_short.as_deref()
            }
            MKAddressRepresentationsContextStyle::Full => {
                self.city_with_context_full.as_deref()
            }
        }
    }
}
