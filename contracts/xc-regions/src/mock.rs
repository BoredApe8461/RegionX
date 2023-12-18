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
use ink::{
	env::{
		test::{default_accounts, DefaultAccounts},
		DefaultEnvironment,
	},
	storage::Mapping,
};
use openbrush::traits::AccountId;
use primitives::{
	coretime::RegionId,
	ensure,
	uniques::{CollectionId, ItemDetails},
};
use uniques_extension::{UniquesError, UniquesExtension};

#[derive(Default, Debug)]
pub struct MockExtension {
	pub(crate) items: Mapping<(CollectionId, RegionId), ItemDetails>,
	pub(crate) account: Mapping<AccountId, Vec<(CollectionId, RegionId)>>,
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
	pub fn mint(
		&mut self,
		id: (CollectionId, RegionId),
		owner: AccountId,
	) -> Result<(), &'static str> {
		ensure!(self.items.get((id.0, id.1)).is_none(), "Item already exists");
		self.items.insert(
			(id.0, id.1),
			&ItemDetails { owner, approved: None, is_frozen: false, deposit: Default::default() },
		);

		let mut owned = self.account.get(owner).map(|a| a).unwrap_or_default();
		owned.push((id.0, id.1));
		self.account.insert(owner, &owned);

		Ok(())
	}

	pub fn burn(&mut self, id: (CollectionId, RegionId)) -> Result<(), &'static str> {
		let Some(owner) = self.items.get((id.0, id.1)).map(|a| a.owner) else {
			return Err("Item not found")
		};

		let mut owned = self.account.get(owner).map(|a| a).unwrap_or_default();
		owned.retain(|a| *a != (id.0, id.1));

		if owned.is_empty() {
			self.account.remove(owner);
		} else {
			self.account.insert(owner, &owned);
		}

		self.items.remove((id.0, id.1));

		Ok(())
	}
}

pub fn region_id(region_id: RegionId) -> (CollectionId, RegionId) {
	(REGIONS_COLLECTION_ID, region_id)
}

pub fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
	default_accounts::<DefaultEnvironment>()
}
