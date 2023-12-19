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

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// NOTE: This should be the collection ID of the underlying region collection.
const REGIONS_COLLECTION_ID: u32 = 42;

#[openbrush::contract(env = environment::ExtendedEnvironment)]
pub mod xc_regions {
	use crate::{
		traits::{regionmetadata_external, RegionMetadata},
		types::{VersionedRegion, XcRegionsError},
		REGIONS_COLLECTION_ID,
	};
	use ink::{
		codegen::{EmitEvent, Env},
		prelude::{string::ToString, vec::Vec},
		storage::Mapping,
	};
	use openbrush::{contracts::psp34::extensions::metadata::*, traits::Storage};
	use primitives::{
		coretime::{RawRegionId, Region, RegionId},
		ensure,
		uniques::{CollectionId, ItemDetails, UniquesCall},
		RuntimeCall, Version,
	};
	use uniques_extension::UniquesExtension;

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct XcRegions {
		pub regions: Mapping<RawRegionId, Region>,
		pub metadata_versions: Mapping<RawRegionId, Version>,
		// Mock state only used for testing. In production the contract is reading the state from
		// the chain extension.
		#[cfg(test)]
		pub items: Mapping<
			(primitives::uniques::CollectionId, primitives::coretime::RawRegionId),
			ItemDetails,
		>,
		// Mock state only used for testing. In production the contract is reading the state from
		// the chain extension.
		#[cfg(test)]
		pub account: Mapping<
			AccountId,
			Vec<(primitives::uniques::CollectionId, primitives::coretime::RawRegionId)>,
		>,
	}

	#[ink(event)]
	pub struct RegionInitialized {
		/// The identifier of the region that got initialized.
		#[ink(topic)]
		pub(crate) region_id: RawRegionId,
		/// The associated metadata.
		pub(crate) metadata: Region,
		/// The version of the metadata. This is incremented by the contract each time the same
		/// region is initialized.
		pub(crate) version: Version,
	}

	#[ink(event)]
	pub struct RegionRemoved {
		/// The identifier of the region that got removed.
		#[ink(topic)]
		pub(crate) region_id: RawRegionId,
	}

	impl PSP34 for XcRegions {
		#[ink(message)]
		fn collection_id(&self) -> Id {
			Id::U32(REGIONS_COLLECTION_ID)
		}

		#[ink(message)]
		fn balance_of(&self, who: AccountId) -> u32 {
			self.owned(who).len() as u32
		}

		#[ink(message)]
		fn owner_of(&self, id: Id) -> Option<AccountId> {
			// We expect the region id to be a `u128`.
			if let Id::U128(region_id) = id {
				self.owner(region_id)
			} else {
				None
			}
		}

		#[ink(message)]
		fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
			// We expect the region id to be a `u128`.
			let Some(Id::U128(region_id)) = id else { return false };

			if let Some(item_details) = self.item(region_id) {
				item_details.owner == owner && item_details.approved == Some(operator)
			} else {
				false
			}
		}

		#[ink(message)]
		fn approve(
			&mut self,
			operator: AccountId,
			id: Option<Id>,
			approved: bool,
		) -> Result<(), PSP34Error> {
			// We expect the region id to be a `u128`.
			let Some(Id::U128(region_id)) = id else {
				return Err(PSP34Error::Custom(XcRegionsError::InvalidRegionId.to_string()))
			};

			if approved {
				// Approve:
				self.env()
					.call_runtime(&RuntimeCall::Uniques(UniquesCall::ApproveTransfer {
						collection: REGIONS_COLLECTION_ID,
						item: region_id,
						delegate: operator,
					}))
					.map_err(|_| PSP34Error::Custom(XcRegionsError::RuntimeError.to_string()))
			} else {
				// Cancel approval:
				self.env()
					.call_runtime(&RuntimeCall::Uniques(UniquesCall::CancelApproval {
						collection: REGIONS_COLLECTION_ID,
						item: region_id,
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
			if let Ok(Some(collection)) = self.env().extension().collection(REGIONS_COLLECTION_ID) {
				collection.items.into()
			} else {
				Default::default()
			}
		}
	}

	impl RegionMetadata for XcRegions {
		#[ink(message)]
		fn init(
			&mut self,
			raw_region_id: RawRegionId,
			region: Region,
		) -> Result<(), XcRegionsError> {
			ensure!(
				Some(self.env().caller()) == self.owner_of(Id::U128(raw_region_id)),
				XcRegionsError::CannotInitialize
			);

			// Cannot initialize a region that already has metadata stored.
			ensure!(self.regions.get(raw_region_id).is_none(), XcRegionsError::CannotInitialize);

			// Do a sanity check to ensure that the provided region metadata matches with the
			// metadata extracted from the region id.
			let region_id = RegionId::from(raw_region_id);
			ensure!(region_id.begin == region.begin, XcRegionsError::InvalidMetadata);
			ensure!(region_id.core == region.core, XcRegionsError::InvalidMetadata);
			ensure!(region_id.mask == region.mask, XcRegionsError::InvalidMetadata);

			let new_version = if let Some(version) = self.metadata_versions.get(raw_region_id) {
				version.saturating_add(1)
			} else {
				Default::default()
			};

			self.metadata_versions.insert(raw_region_id, &new_version);
			self.regions.insert(raw_region_id, &region);

			self.env().emit_event(RegionInitialized {
				region_id: raw_region_id,
				metadata: region,
				version: new_version,
			});

			Ok(())
		}

		#[ink(message)]
		fn get_metadata(&self, region_id: RawRegionId) -> Result<VersionedRegion, XcRegionsError> {
			// We must first ensure that the region is still present on this chain before retrieving
			// the metadata.
			ensure!(self.exists(region_id), XcRegionsError::RegionNotFound);

			let Some(region) = self.regions.get(region_id) else {
				return Err(XcRegionsError::MetadataNotFound)
			};
			let Some(version) = self.metadata_versions.get(region_id) else {
				// This should never really happen; if a region has its metadata stored, its version
				// should be stored as well.
				return Err(XcRegionsError::VersionNotFound)
			};

			Ok(VersionedRegion { version, region })
		}

		#[ink(message)]
		fn remove(&mut self, region_id: RawRegionId) -> Result<(), XcRegionsError> {
			// We only allow the removal of regions that no longer exist in the underlying nft
			// pallet.
			ensure!(!self.exists(region_id), XcRegionsError::CannotRemove);
			self.regions.remove(region_id);

			self.env().emit_event(RegionRemoved { region_id });
			Ok(())
		}
	}

	impl XcRegions {
		#[ink(constructor)]
		pub fn new() -> Self {
			Default::default()
		}
	}

	impl XcRegions {
		fn exists(&self, region_id: RawRegionId) -> bool {
			self.item(region_id).is_some()
		}

		fn item(&self, item_id: RawRegionId) -> Option<ItemDetails> {
			#[cfg(not(test))]
			{
				self.env().extension().item(REGIONS_COLLECTION_ID, item_id).ok()?
			}
			// When testing we use mock state.
			#[cfg(test)]
			{
				self.items.get((REGIONS_COLLECTION_ID, item_id))
			}
		}

		fn owner(&self, region_id: RawRegionId) -> Option<AccountId> {
			#[cfg(not(test))]
			{
				self.env().extension().owner(REGIONS_COLLECTION_ID, region_id).ok()?
			}
			// When testing we use mock state.
			#[cfg(test)]
			{
				self.items.get((REGIONS_COLLECTION_ID, region_id)).map(|a| a.owner)
			}
		}

		fn owned(&self, who: AccountId) -> Vec<(CollectionId, RawRegionId)> {
			#[cfg(not(test))]
			{
				self.env().extension().owned(who).unwrap_or_default()
			}
			// When testing we use mock state.
			#[cfg(test)]
			{
				self.account.get(who).map(|a| a).unwrap_or_default()
			}
		}
	}

	// Helper functions for modifying the mock state.
	#[cfg(test)]
	impl XcRegions {
		pub fn mint(
			&mut self,
			id: (CollectionId, RawRegionId),
			owner: AccountId,
		) -> Result<(), &'static str> {
			ensure!(self.items.get((id.0, id.1)).is_none(), "Item already exists");
			self.items.insert(
				(id.0, id.1),
				&ItemDetails {
					owner,
					approved: None,
					is_frozen: false,
					deposit: Default::default(),
				},
			);

			let mut owned = self.account.get(owner).map(|a| a).unwrap_or_default();
			owned.push((id.0, id.1));
			self.account.insert(owner, &owned);

			Ok(())
		}

		pub fn burn(&mut self, id: (CollectionId, RawRegionId)) -> Result<(), &'static str> {
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

	#[cfg(all(test, feature = "e2e-tests"))]
	pub mod tests {
		use crate::{
			mock::{region_id, register_chain_extensions, MockExtension},
			xc_regions::XcRegionsRef,
		};
		use ink::env::{test::DefaultAccounts, DefaultEnvironment};
		use ink_e2e::build_message;
		use openbrush::contracts::psp34::psp34_external::PSP34;
		use primitives::{address_of, assert_ok};

		type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

		#[ink_e2e::test]
		async fn test_1(
			mut client: ink_e2e::Client<C, environment::ExtendedEnvironment>,
		) -> E2EResult<()> {
			let mut mock = MockExtension::default();
			register_chain_extensions(mock);

			let constructor = XcRegionsRef::new();

			/* Related issue: https://substrate.stackexchange.com/questions/10675/ink-e2e-tests-with-custom-environment
			let address = client
				.instantiate("xc_regions", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;
				*/

			Ok(())
		}
	}
}
