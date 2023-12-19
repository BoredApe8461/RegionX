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

use crate::{coretime::RawRegionId, Balance};
use openbrush::traits::AccountId;

// The type used to identify collections in the underlying uniques pallet.
pub type CollectionId = u32;

#[derive(scale::Encode)]
pub enum UniquesCall {
	// TODO: use proper index based on the underlying runtime.
	#[codec(index = 8)]
	Transfer { collection: CollectionId, item: RawRegionId, dest: AccountId },
	#[codec(index = 25)]
	ApproveTransfer { collection: CollectionId, item: RawRegionId, delegate: AccountId },
	#[codec(index = 26)]
	CancelApproval {
		collection: CollectionId,
		item: RawRegionId,
		maybe_check_delegate: Option<AccountId>,
	},
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct CollectionDetails {
	/// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
	pub owner: AccountId,
	/// Can mint tokens.
	pub issuer: AccountId,
	/// Can thaw tokens, force transfers and burn tokens from any account.
	pub admin: AccountId,
	/// Can freeze tokens.
	pub freezer: AccountId,
	/// The total balance deposited for the all storage associated with this collection.
	/// Used by `destroy`.
	pub total_deposit: Balance,
	/// If `true`, then no deposit is needed to hold items of this collection.
	pub free_holding: bool,
	/// The total number of outstanding items of this collection.
	pub items: u32,
	/// The total number of outstanding item metadata of this collection.
	pub item_metadatas: u32,
	/// The total number of attributes for this collection.
	pub attributes: u32,
	/// Whether the collection is frozen for non-admin transfers.
	pub is_frozen: bool,
}

/// Information concerning the ownership of a single unique item.
#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct ItemDetails {
	/// The owner of this item.
	pub owner: AccountId,
	/// The approved transferrer of this item, if one is set.
	pub approved: Option<AccountId>,
	/// Whether the item can be transferred or not.
	pub is_frozen: bool,
	/// The amount held in the pallet's default account for this item. Free-hold items will have
	/// this as zero.
	pub deposit: Balance,
}
