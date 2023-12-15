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

#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

mod traits;
mod types;

// NOTE: This should be the collection ID of the underlying region collection.
const REGIONS_COLLECTION_ID: u32 = 42;

#[openbrush::contract(env = environment::ExtendedEnvironment)]
pub mod xc_regions {
	use crate::{
		traits::{
			regionmetadata_external, NonFungiblesInspect, RegionMetadata, RegionMetadataError,
		},
		types::{Region, RegionId},
		REGIONS_COLLECTION_ID,
	};
	use ink::{codegen::Env, storage::Mapping};
	use openbrush::{contracts::psp34::extensions::metadata::*, traits::Storage};
	use primitives::CollectionId;
	use uniques_extension::UniquesExtension;

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct XcRegions {
		pub metadata: Mapping<RegionId, Region>,
	}

	impl PSP34 for XcRegions {
		#[ink(message)]
		fn collection_id(&self) -> Id {
			Id::U32(REGIONS_COLLECTION_ID)
		}

		#[ink(message)]
		fn balance_of(&self, who: AccountId) -> u32 {
			if let Ok(owned) = self.env().extension().owned(who) {
				owned.len() as u32
			} else {
				0u32
			}
		}

		#[ink(message)]
		fn owner_of(&self, id: Id) -> Option<AccountId> {
			if let Id::U128(region_id) = id {
				self.env().extension().owner(REGIONS_COLLECTION_ID, region_id).ok()
			} else {
				None
			}
		}

		#[ink(message)]
		fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
			todo!()
		}

		#[ink(message)]
		fn approve(
			&mut self,
			operator: AccountId,
			id: Option<Id>,
			approved: bool,
		) -> Result<(), PSP34Error> {
			todo!()
		}

		#[ink(message)]
		fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
			todo!()
		}

		#[ink(message)]
		fn total_supply(&self) -> Balance {
			todo!()
		}
	}

	impl RegionMetadata for XcRegions {
		#[ink(message)]
		fn init(&mut self, id: RegionId, metadata: Region) -> Result<(), RegionMetadataError> {
			todo!()
		}

		#[ink(message)]
		fn get_metadata(&self, id: RegionId) -> Result<Region, RegionMetadataError> {
			todo!()
		}

		#[ink(message)]
		fn destroy(&mut self, id: RegionId) -> Result<(), RegionMetadataError> {
			todo!()
		}
	}

	impl XcRegions {
		#[ink(constructor)]
		pub fn new() -> Self {
			Default::default()
		}
	}

	impl NonFungiblesInspect<RegionId, AccountId> for XcRegions {
		fn _exists(&self, id: RegionId) -> bool {
			todo!()
		}

		fn _owned(&self, id: RegionId) -> AccountId {
			todo!()
		}
	}
}
