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

use openbrush::traits::BlockNumber;

/// The type used for identifying regions.
///
/// This `u128` actually holds parts of the region metadata.
pub type RawRegionId = u128;

/// Relay chain block number.
pub type Timeslice = u32;

/// Index of a Polkadot Core.
pub type CoreIndex = u16;

pub const TIMESLICE_DURATION_IN_BLOCKS: BlockNumber = 80;

/// All Regions are also associated with a Core Mask, an 80-bit bitmap, to denote the regularity at
/// which it may be scheduled on the core.
#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct CoreMask([u8; 10]);

impl CoreMask {
	pub fn void() -> Self {
		Self([0u8; 10])
	}
	pub fn complete() -> Self {
		Self([255u8; 10])
	}
	pub fn count_zeros(&self) -> u32 {
		self.0.iter().map(|i| i.count_zeros()).sum()
	}
	pub fn count_ones(&self) -> u32 {
		self.0.iter().map(|i| i.count_ones()).sum()
	}
	pub fn from_chunk(from: u32, to: u32) -> Self {
		let mut v = [0u8; 10];
		for i in (from.min(80) as usize)..(to.min(80) as usize) {
			v[i / 8] |= 128 >> (i % 8);
		}
		Self(v)
	}
}

impl From<u128> for CoreMask {
	fn from(x: u128) -> Self {
		let mut v = [0u8; 10];
		v.iter_mut().rev().fold(x, |a, i| {
			*i = a as u8;
			a >> 8
		});
		Self(v)
	}
}
impl From<CoreMask> for u128 {
	fn from(x: CoreMask) -> Self {
		x.0.into_iter().fold(0u128, |a, i| a << 8 | i as u128)
	}
}

/// Self-describing identity for a Region of Bulk Coretime.
#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct RegionId {
	/// The timeslice at which the region starts.
	pub begin: Timeslice,
	/// The index of the relay chain Core on which this Region will be scheduled.
	pub core: CoreIndex,
	/// The regularity parts in which this Region will be scheduled.
	pub mask: CoreMask,
}

impl From<RawRegionId> for RegionId {
	fn from(x: u128) -> Self {
		Self { begin: (x >> 96) as u32, core: (x >> 80) as u16, mask: x.into() }
	}
}

impl From<RegionId> for RawRegionId {
	fn from(x: RegionId) -> Self {
		(x.begin as u128) << 96 | (x.core as u128) << 80 | u128::from(x.mask)
	}
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Region {
	/// The timeslice at which the reigon starts.
	pub begin: Timeslice,
	/// The timeslice at which the region ends.
	pub end: Timeslice,
	/// The index of the relay chain Core on which this Region will be scheduled.
	pub core: CoreIndex,
	/// The regularity parts in which this Region will be scheduled.
	pub mask: CoreMask,
}
