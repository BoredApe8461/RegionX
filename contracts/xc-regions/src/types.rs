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

pub type RegionId = u128;

pub type Timeslice = u32;

/// Index of a Polkadot Core.
pub type CoreIndex = u16;

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct CoreMask([u8; 10]);

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Region {
	begin: Timeslice,
	end: Timeslice,
	core: CoreIndex,
	mask: CoreMask,
}

#[derive(scale::Decode, scale::Encode, Debug)]
pub enum XcRegionsError {
	/// The provided identifier is not a valid region id.
	InvalidRegionId,
	/// An error occured in the underlying runtime.
	RuntimeError,
	/// The operation is not supported.
	NotSupported,
}

impl core::fmt::Display for XcRegionsError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			XcRegionsError::InvalidRegionId => write!(f, "InvalidRegionId"),
			XcRegionsError::RuntimeError => write!(f, "RuntimeError"),
			XcRegionsError::NotSupported => write!(f, "NotSupported"),
		}
	}
}
