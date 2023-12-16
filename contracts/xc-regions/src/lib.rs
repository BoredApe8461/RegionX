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
		traits::{regionmetadata_external, RegionMetadata},
		types::{Region, RegionId, XcRegionsError},
		REGIONS_COLLECTION_ID,
	};
	use ink::{codegen::Env, storage::Mapping};
	use openbrush::{contracts::psp34::extensions::metadata::*, traits::Storage};
	use primitives::{ensure, uniques::UniquesCall, RuntimeCall};
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
		fn allowance(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>) -> bool {
			// Resolvable with: https://github.com/paritytech/polkadot-sdk/pull/2727
			todo!()
		}

		#[ink(message)]
		fn approve(
			&mut self,
			operator: AccountId,
			id: Option<Id>,
			approved: bool,
		) -> Result<(), PSP34Error> {
			let Some(Id::U128(id)) = id else {
				return Err(PSP34Error::Custom(XcRegionsError::InvalidRegionId.to_string()))
			};

			if approved {
				// Approve:
				self.env()
					.call_runtime(&RuntimeCall::Uniques(UniquesCall::ApproveTransfer {
						collection: REGIONS_COLLECTION_ID,
						item: id,
						delegate: operator,
					}))
					.map_err(|_| PSP34Error::Custom(XcRegionsError::RuntimeError.to_string()))
			} else {
				// Cancel approval:
				self.env()
					.call_runtime(&RuntimeCall::Uniques(UniquesCall::CancelApproval {
						collection: REGIONS_COLLECTION_ID,
						item: id,
						maybe_check_delegate: Some(operator),
					}))
					.map_err(|_| PSP34Error::Custom(XcRegionsError::RuntimeError.to_string()))
			}
		}

		#[ink(message)]
		fn transfer(&mut self, to: AccountId, id: Id, _data: Vec<u8>) -> Result<(), PSP34Error> {
			let Id::U128(id) = id else {
				return Err(PSP34Error::Custom(XcRegionsError::InvalidRegionId.to_string()))
			};

			self.env()
				.call_runtime(&RuntimeCall::Uniques(UniquesCall::Transfer {
					collection: REGIONS_COLLECTION_ID,
					item: id,
					dest: to,
				}))
				.map_err(|_| PSP34Error::Custom(XcRegionsError::RuntimeError.to_string()))
		}

		#[ink(message)]
		fn total_supply(&self) -> Balance {
			// Unsupported since it would reuire a lot of storage reads.
			Default::default()
		}
	}

	impl RegionMetadata for XcRegions {
		#[ink(message)]
		fn init(&mut self, region_id: RegionId, _metadata: Region) -> Result<(), XcRegionsError> {
			ensure!(self.exists(region_id), XcRegionsError::MetadataAlreadyInitialized);
			todo!()
		}

		#[ink(message)]
		fn get_metadata(&self, _id: RegionId) -> Result<Region, XcRegionsError> {
			todo!()
		}

		#[ink(message)]
		fn destroy(&mut self, _id: RegionId) -> Result<(), XcRegionsError> {
			todo!()
		}
	}

	impl XcRegions {
		#[ink(constructor)]
		pub fn new() -> Self {
			Default::default()
		}
	}

	impl XcRegions {
		fn exists(&self, region_id: RegionId) -> bool {
			if let Ok(maybe_item) = self.env().extension().item(REGIONS_COLLECTION_ID, region_id) {
				maybe_item.is_some()
			} else {
				false
			}
		}
	}
}
