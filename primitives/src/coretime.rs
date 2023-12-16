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

/// The type used for identifying regions.
///
/// This `u128` actually holds parts of the region metadata.
///
/// The type that is encoded into this `u128`:
///
/// ```rust
/// pub struct RegionId {
/// 	    /// The timeslice at which this Region begins.
/// 	    pub begin: Timeslice,
/// 	    /// The index of the Polakdot Core on which this Region will be scheduled.
/// 	    pub core: CoreIndex,
/// 	    /// The regularity parts in which this Region will be scheduled.
/// 	    pub mask: CoreMask,
/// }
/// ```
pub type RegionId = u128;

/// Relay chain block number.
pub type Timeslice = u32;

/// Index of a Polkadot Core.
pub type CoreIndex = u16;

/// All Regions are also associated with a Core Mask, an 80-bit bitmap, to denote the regularity at
/// which it may be scheduled on the core.
#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct CoreMask([u8; 10]);

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Region {
	/// The timeslice at which the reigon starts.
	begin: Timeslice,
	/// The timeslice at which the region ends.
	end: Timeslice,
	/// The index of the relay chain Core on which this Region will be scheduled.
	core: CoreIndex,
	/// The regularity parts in which this Region will be scheduled.
	mask: CoreMask,
}
