// This file is part of RegionX.
//
// RegionX is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// RegionX is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with RegionX.  If not, see <https://www.gnu.org/licenses/>.

use primitives::{coretime::Region, Version};

#[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XcRegionsError {
	/// The provided identifier is not a valid region id.
	InvalidRegionId,
	/// The metadata is either already initialized or the caller isn't the region owner.
	CannotInitialize,
	/// The region is non existant.
	RegionNotFound,
	/// No metadata was found for the region.
	MetadataNotFound,
	/// The provided metadata doesn't match with the metadata extracted from the region id.
	InvalidMetadata,
	/// The associated metadata version was not found.
	VersionNotFound,
	/// The operation is not allowed.
	NotAllowed,
	/// An error occured in the underlying runtime.
	RuntimeError,
	/// The operation is not supported.
	NotSupported,
}

impl core::fmt::Display for XcRegionsError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			XcRegionsError::InvalidRegionId => write!(f, "InvalidRegionId"),
			XcRegionsError::CannotInitialize => write!(f, "CannotInitialize"),
			XcRegionsError::RegionNotFound => write!(f, "RegionNotFound"),
			XcRegionsError::MetadataNotFound => write!(f, "MetadataNotFound"),
			XcRegionsError::InvalidMetadata => write!(f, "InvalidMetadata"),
			XcRegionsError::VersionNotFound => write!(f, "VersionNotFound"),
			XcRegionsError::NotAllowed => write!(f, "NotAllowed"),
			XcRegionsError::RuntimeError => write!(f, "RuntimeError"),
			XcRegionsError::NotSupported => write!(f, "NotSupported"),
		}
	}
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct VersionedRegion {
	pub version: Version,
	pub region: Region,
}
