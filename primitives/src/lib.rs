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

use openbrush::traits::AccountId;

// The type used to identify collections in the underlying NFT pallet.
pub type CollectionId = u32;
// TODO: docs
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

#[derive(scale::Encode)]
pub enum RuntimeCall {
	// TODO: use proper index based on the underlying runtime.
	#[codec(index = 17)]
	Uniques(UniquesCall),
}

#[derive(scale::Encode)]
pub enum UniquesCall {
	// TODO: use proper index based on the underlying runtime.
	#[codec(index = 8)]
	Transfer { collection: CollectionId, item: RegionId, dest: AccountId },
	#[codec(index = 25)]
	ApproveTransfer { collection: CollectionId, item: RegionId, delegate: AccountId },
	#[codec(index = 26)]
	CancelApproval {
		collection: CollectionId,
		item: RegionId,
		maybe_check_delegate: Option<AccountId>,
	},
}
