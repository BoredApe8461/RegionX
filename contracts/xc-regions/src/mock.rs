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
use ink::env::{
	test::{default_accounts, DefaultAccounts},
	DefaultEnvironment,
};
use openbrush::traits::AccountId;
use primitives::{
	coretime::RawRegionId,
	uniques::{CollectionId, ItemDetails},
};
use uniques_extension::{UniquesError, UniquesExtension};

#[derive(Default, Debug)]
pub struct MockExtension;

#[obce::mock]
impl UniquesExtension for MockExtension {
	/// The owner of the specific item.
	fn owner(
		&self,
		_collection_id: CollectionId,
		_region_id: RawRegionId,
	) -> Result<Option<AccountId>, UniquesError> {
		// In test environment the state isn't read from chain extensions.
		unimplemented!()
	}

	/// All items owned by `who`.
	fn owned(&self, _who: AccountId) -> Result<Vec<(CollectionId, RawRegionId)>, UniquesError> {
		// In test environment the state isn't read from chain extensions.
		unimplemented!()
	}

	/// An item within a collection.
	fn item(
		&self,
		_collection_id: CollectionId,
		_region_id: RawRegionId,
	) -> Result<Option<ItemDetails>, UniquesError> {
		// In test environment the state isn't read from chain extensions.
		unimplemented!()
	}
}

pub fn region_id(region_id: RawRegionId) -> (CollectionId, RawRegionId) {
	(REGIONS_COLLECTION_ID, region_id)
}

pub fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
	default_accounts::<DefaultEnvironment>()
}
