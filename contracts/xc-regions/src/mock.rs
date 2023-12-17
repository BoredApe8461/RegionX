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

use ink::storage::Mapping;
use openbrush::traits::AccountId;
use primitives::{coretime::RegionId, uniques::CollectionId, Balance};
use scale::{Decode, Encode};
use uniques_extension::{UniquesError, UniquesExtension};

/// Information concerning the ownership of a single unique item.
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default, scale_info::TypeInfo)]
struct ItemDetails<AccountId, DepositBalance> {
	/// The owner of this item.
	pub(super) owner: AccountId,
	/// The approved transferrer of this item, if one is set.
	pub(super) approved: Option<AccountId>,
	/// Whether the item can be transferred or not.
	pub(super) is_frozen: bool,
	/// The amount held in the pallet's default account for this item. Free-hold items will have
	/// this as zero.
	pub(super) deposit: DepositBalance,
}

#[derive(Default, Debug)]
pub struct MockExtension {
	items: Mapping<(CollectionId, RegionId), ItemDetails<AccountId, Balance>>,
}

#[obce::mock]
impl UniquesExtension for MockExtension {
	/// The owner of the specific item.
	fn owner(
		&self,
		collection_id: CollectionId,
		item_id: RegionId,
	) -> Result<AccountId, UniquesError> {
		todo!()
	}
}
