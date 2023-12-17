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

use crate::REGIONS_COLLECTION_ID;
use ink::storage::Mapping;
use openbrush::traits::AccountId;
use primitives::{
	coretime::RegionId,
	uniques::{CollectionId, ItemDetails},
	Balance,
};
use scale::{Decode, Encode};
use uniques_extension::{UniquesError, UniquesExtension};

#[derive(Default, Debug)]
pub struct MockExtension {
	items: Mapping<(CollectionId, RegionId), ItemDetails>,
	account: Mapping<AccountId, Vec<(CollectionId, RegionId)>>,
}

#[obce::mock]
impl UniquesExtension for MockExtension {
	/// The owner of the specific item.
	fn owner(
		&self,
		collection_id: CollectionId,
		region_id: RegionId,
	) -> Result<Option<AccountId>, UniquesError> {
		Ok(self.items.get((collection_id, region_id)).map(|a| a.owner))
	}

	/// All items owned by `who`.
	fn owned(&self, who: AccountId) -> Result<Vec<(CollectionId, RegionId)>, UniquesError> {
		Ok(self.account.get(who).map(|a| a).unwrap_or_default())
	}

	/// An item within a collection.
	fn item(
		&self,
		collection_id: CollectionId,
		region_id: RegionId,
	) -> Result<Option<ItemDetails>, UniquesError> {
		Ok(self.items.get((collection_id, region_id)))
	}
}

// Helper functions for modifying the mock state.
impl MockExtension {
	pub fn mint(&mut self, region_id: RegionId, owner: AccountId) {
		self.items.insert(
			(REGIONS_COLLECTION_ID, region_id),
			&ItemDetails { owner, approved: None, is_frozen: false, deposit: Default::default() },
		);

		let mut owned = self.account.get(owner).map(|a| a).unwrap_or_default();
		owned.push((REGIONS_COLLECTION_ID, region_id));
		self.account.insert(owner, &owned);
	}

	pub fn burn(&mut self, region_id: RegionId) {
		let owner = self
			.items
			.get((REGIONS_COLLECTION_ID, region_id))
			.map(|a| a.owner)
			.expect("Item not found");

		let mut owned = self.account.get(owner).map(|a| a).unwrap_or_default();
		owned.retain(|a| *a != (REGIONS_COLLECTION_ID, region_id));
		self.account.insert(owner, &owned);

		self.items.remove((REGIONS_COLLECTION_ID, region_id));
	}
}
