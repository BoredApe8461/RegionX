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
mod tests;

// NOTE: This should be the collection ID of the underlying region collection.
const REGIONS_COLLECTION_ID: u32 = 42;

#[openbrush::implementation(PSP34, PSP34Enumerable)]
#[openbrush::contract(env = environment::ExtendedEnvironment)]
pub mod xc_regions {
	use crate::{
		traits::{regionmetadata_external, RegionMetadata},
		types::{VersionedRegion, XcRegionsError},
		REGIONS_COLLECTION_ID,
	};
	use ink::{
		codegen::{EmitEvent, Env},
		storage::Mapping,
	};
	use openbrush::traits::Storage;
	use primitives::{
		coretime::{RawRegionId, Region, RegionId},
		ensure,
		uniques::{ItemDetails, UniquesCall},
		RuntimeCall, Version,
	};
	use uniques_extension::UniquesExtension;

	#[cfg(test)]
	use primitives::uniques::CollectionId;

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct XcRegions {
		#[storage_field]
		psp34: psp34::Data,
		#[storage_field]
		enumerable: enumerable::Data,
		/// A mapping that links RawRegionId to its corresponding region metadata.
		pub regions: Mapping<RawRegionId, Region>,
		/// A mapping that keeps track of the metadata version for each region.
		///
		/// This version gets incremented for a region each time it gets re-initialized.
		pub metadata_versions: Mapping<RawRegionId, Version>,
		// Mock chain extension state only used for integration testing.
		#[cfg(test)]
		pub items: Mapping<
			(primitives::uniques::CollectionId, primitives::coretime::RawRegionId),
			ItemDetails,
		>,
		// Mock chain extension state only used for integration testing.
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

	#[overrider(PSP34)]
	fn collection_id(&self) -> Id {
		Id::U32(REGIONS_COLLECTION_ID)
	}

	impl RegionMetadata for XcRegions {
		/// A function for minting a wrapped xcRegion and initializing the metadata of it. It can
		/// only be called if the specified region exists on this chain and the caller is the actual
		/// owner of the region.
		///
		/// ## Arguments:
		/// - `raw_region_id` - The `u128` encoded region identifier.
		/// - `region` - The corresponding region metadata.
		///
		/// This function conducts a sanity check to verify that the metadata derived from the
		/// `raw_region_id` aligns with the respective components of the metadata supplied through
		/// the region argument.
		///
		/// If this is not the first time that this region is inititalized, the metadata version
		/// will get incremented.
		///
		/// The underlying region will be transferred to this contract, and in response, a wrapped
		/// token will be minted for the caller.
		///
		/// NOTE: Prior to invoking this ink message, the caller must grant approval to the contract
		/// for the region, enabling its transfer.
		///
		/// ## Events:
		/// On success this ink message emits the `RegionInitialized` event.
		#[ink(message)]
		fn init(
			&mut self,
			raw_region_id: RawRegionId,
			region: Region,
		) -> Result<(), XcRegionsError> {
			let caller = self.env().caller();
			ensure!(
				Some(caller) == self._uniques_owner(raw_region_id),
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

			// After passing all checks we will transfer the region to the contract and mint a
			// wrapped xcRegion token.
			let contract = self.env().account_id();
			self._transfer(raw_region_id, contract)?;

			let new_version = if let Some(version) = self.metadata_versions.get(raw_region_id) {
				version.saturating_add(1)
			} else {
				Default::default()
			};

			self.metadata_versions.insert(raw_region_id, &new_version);
			self.regions.insert(raw_region_id, &region);

			psp34::InternalImpl::_mint_to(self, caller, Id::U128(raw_region_id))
				.map_err(XcRegionsError::Psp34)?;

			self.env().emit_event(RegionInitialized {
				region_id: raw_region_id,
				metadata: region,
				version: new_version,
			});

			Ok(())
		}

		/// A function to retrieve all metadata associated with a specific region.
		///
		/// The function returns a `VersionedRegion`, encompassing the version of the retrieved
		/// metadata that is intended for client-side verification.
		///
		/// ## Arguments:
		/// - `raw_region_id` - The `u128` encoded region identifier.
		#[ink(message)]
		fn get_metadata(&self, region_id: RawRegionId) -> Result<VersionedRegion, XcRegionsError> {
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

		/// A function to return the region to its owner.
		///
		/// This process involves burning the wrapped region and eliminating its associated
		/// metadata.
		///
		/// Only the owner of the wrapped region can call this function.
		///
		/// ## Arguments:
		/// - `raw_region_id` - The `u128` encoded region identifier.
		///
		/// ## Events:
		/// On success this ink message emits the `RegionRemoved` event.
		#[ink(message)]
		fn remove(&mut self, region_id: RawRegionId) -> Result<(), XcRegionsError> {
			let id = Id::U128(region_id);
			let owner =
				psp34::PSP34Impl::owner_of(self, id.clone()).ok_or(XcRegionsError::CannotRemove)?;

			ensure!(owner == self.env().caller(), XcRegionsError::CannotRemove);
			self.regions.remove(region_id);

			psp34::InternalImpl::_burn_from(self, owner, id).map_err(XcRegionsError::Psp34)?;
			self._transfer(region_id, owner)?;

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

	// Internal functions:
	#[cfg(not(test))]
	impl XcRegions {
		fn _transfer(&self, region_id: RawRegionId, dest: AccountId) -> Result<(), XcRegionsError> {
			self.env()
				.call_runtime(&RuntimeCall::Uniques(UniquesCall::Transfer {
					collection: REGIONS_COLLECTION_ID,
					item: region_id,
					dest: dest.into(),
				}))
				.map_err(|_| XcRegionsError::RuntimeError)?;

			Ok(())
		}

		/// Returns whether the region exists on this chain or not.
		fn _uniques_exists(&self, region_id: RawRegionId) -> bool {
			self._uniques_item(region_id).is_some()
		}

		/// Returns the details of an item within a collection.
		fn _uniques_item(&self, item_id: RawRegionId) -> Option<ItemDetails> {
			self.env().extension().item(REGIONS_COLLECTION_ID, item_id).ok()?
		}

		/// The owner of the specific item.
		fn _uniques_owner(&self, region_id: RawRegionId) -> Option<AccountId> {
			self.env().extension().owner(REGIONS_COLLECTION_ID, region_id).ok()?
		}
	}

	// Implelementation of internal functions used only for integration tests.
	#[cfg(test)]
	impl XcRegions {
		fn _transfer(
			&mut self,
			region_id: RawRegionId,
			dest: AccountId,
		) -> Result<(), XcRegionsError> {
			self.burn((REGIONS_COLLECTION_ID, region_id)).unwrap();
			self.mint((REGIONS_COLLECTION_ID, region_id), dest).unwrap();
			Ok(())
		}

		/// Returns whether the region exists on this chain or not.
		pub fn _uniques_exists(&self, region_id: RawRegionId) -> bool {
			self._uniques_item(region_id).is_some()
		}

		/// Returns the details of an item within a collection.
		pub fn _uniques_item(&self, item_id: RawRegionId) -> Option<ItemDetails> {
			self.items.get((REGIONS_COLLECTION_ID, item_id))
		}

		/// The owner of the specific item.
		pub fn _uniques_owner(&self, region_id: RawRegionId) -> Option<AccountId> {
			self.items.get((REGIONS_COLLECTION_ID, region_id)).map(|a| a.owner)
		}

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
		use super::*;
		use crate::{
			traits::regionmetadata_external::RegionMetadata, types::VersionedRegion,
			REGIONS_COLLECTION_ID,
		};
		use environment::ExtendedEnvironment;
		use ink_e2e::{subxt::dynamic::Value, MessageBuilder};
		use openbrush::contracts::psp34::psp34_external::PSP34;
		use primitives::address_of;

		type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

		#[ink_e2e::test(environment = ExtendedEnvironment)]
		async fn init_non_existing_region_fails(
			mut client: ink_e2e::Client<C, E>,
		) -> E2EResult<()> {
			let constructor = XcRegionsRef::new();
			let contract_acc_id = client
				.instantiate("xc-regions", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			let raw_region_id = 0u128;
			let region = Region::default();

			let init = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.init(raw_region_id, region.clone()));
			let init_result = client.call_dry_run(&ink_e2e::alice(), &init, 0, None).await;
			assert_eq!(init_result.return_value(), Err(XcRegionsError::CannotInitialize));

			Ok(())
		}

		#[ink_e2e::test(environment = ExtendedEnvironment)]
		async fn init_works(mut client: E2EBackend) -> E2EResult<()> {
			let constructor = XcRegionsRef::new();
			let contract_acc_id = client
				.instantiate("xc-regions", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			let raw_region_id = 0u128;
			let region = Region::default();

			// Create region: collection
			let call_data = vec![
				Value::u128(REGIONS_COLLECTION_ID.into()),
				Value::unnamed_variant("Id", [Value::from_bytes(&address_of!(Alice))]),
			];
			client
				.runtime_call(&ink_e2e::alice(), "Uniques", "create", call_data)
				.await
				.expect("creating a collection failed");

			// Mint region:
			let call_data = vec![
				Value::u128(REGIONS_COLLECTION_ID.into()),
				Value::u128(raw_region_id.into()),
				Value::unnamed_variant("Id", [Value::from_bytes(&address_of!(Alice))]),
			];
			client
				.runtime_call(&ink_e2e::alice(), "Uniques", "mint", call_data)
				.await
				.expect("minting a region failed");

			// Approve transfer region:
			let call_data = vec![
				Value::u128(REGIONS_COLLECTION_ID.into()),
				Value::u128(raw_region_id.into()),
				Value::unnamed_variant("Id", [Value::from_bytes(contract_acc_id)]),
			];
			client
				.runtime_call(&ink_e2e::alice(), "Uniques", "approve_transfer", call_data)
				.await
				.expect("approving transfer failed");

			let init = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.init(raw_region_id, region.clone()));
			let init_result = client.call(&ink_e2e::alice(), init, 0, None).await;
			assert!(init_result.is_ok(), "Init should work");

			// Ensure the state is properly updated:

			// Alice receives the wrapped region:
			let balance_of = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.balance_of(address_of!(Alice)));
			let balance_of_res = client.call_dry_run(&ink_e2e::alice(), &balance_of, 0, None).await;
			assert_eq!(balance_of_res.return_value(), 1);

			let owner_of = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.owner_of(Id::U128(0)));
			let owner_of_res = client.call_dry_run(&ink_e2e::alice(), &owner_of, 0, None).await;
			assert_eq!(owner_of_res.return_value(), Some(address_of!(Alice)));

			// The metadata is properly stored:
			let get_metadata =
				MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
					contract_acc_id.clone(),
				)
				.call(|xc_regions| xc_regions.get_metadata(raw_region_id));
			let get_metadata_res =
				client.call_dry_run(&ink_e2e::alice(), &get_metadata, 0, None).await;

			assert_eq!(get_metadata_res.return_value(), Ok(VersionedRegion { version: 0, region }));

			Ok(())
		}

		#[ink_e2e::test(environment = ExtendedEnvironment)]
		async fn remove_works(mut client: E2EBackend) -> E2EResult<()> {
			let constructor = XcRegionsRef::new();
			let contract_acc_id = client
				.instantiate("xc-regions", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			let raw_region_id = 0u128;
			let region = Region::default();

			// Create region: collection
			let call_data = vec![
				Value::u128(REGIONS_COLLECTION_ID.into()),
				Value::unnamed_variant("Id", [Value::from_bytes(&address_of!(Alice))]),
			];
			client
				.runtime_call(&ink_e2e::alice(), "Uniques", "create", call_data)
				.await
				.expect("creating a collection failed");

			// Mint region:
			let call_data = vec![
				Value::u128(REGIONS_COLLECTION_ID.into()),
				Value::u128(raw_region_id.into()),
				Value::unnamed_variant("Id", [Value::from_bytes(&address_of!(Alice))]),
			];
			client
				.runtime_call(&ink_e2e::alice(), "Uniques", "mint", call_data)
				.await
				.expect("minting a region failed");

			// Approve transfer region:
			let call_data = vec![
				Value::u128(REGIONS_COLLECTION_ID.into()),
				Value::u128(raw_region_id.into()),
				Value::unnamed_variant("Id", [Value::from_bytes(contract_acc_id)]),
			];
			client
				.runtime_call(&ink_e2e::alice(), "Uniques", "approve_transfer", call_data)
				.await
				.expect("approving transfer failed");

			let init = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.init(raw_region_id, region.clone()));
			let init_result = client.call(&ink_e2e::alice(), init, 0, None).await;
			assert!(init_result.is_ok(), "Init should succeed");

			let remove = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.remove(raw_region_id));

			let remove_result = client.call(&ink_e2e::alice(), remove, 0, None).await;
			assert!(remove_result.is_ok(), "Remove should work");

			// Ensure the state is properly updated:

			// Alice no longer holds the wrapped region:
			let balance_of = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.balance_of(address_of!(Alice)));
			let balance_of_res = client.call_dry_run(&ink_e2e::alice(), &balance_of, 0, None).await;
			assert_eq!(balance_of_res.return_value(), 0);

			let owner_of = MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
				contract_acc_id.clone(),
			)
			.call(|xc_regions| xc_regions.owner_of(Id::U128(0)));
			let owner_of_res = client.call_dry_run(&ink_e2e::alice(), &owner_of, 0, None).await;
			assert_eq!(owner_of_res.return_value(), None);

			// The metadata should be removed:
			let get_metadata =
				MessageBuilder::<ExtendedEnvironment, XcRegionsRef>::from_account_id(
					contract_acc_id.clone(),
				)
				.call(|xc_regions| xc_regions.get_metadata(raw_region_id));
			let get_metadata_res =
				client.call_dry_run(&ink_e2e::alice(), &get_metadata, 0, None).await;

			assert_eq!(get_metadata_res.return_value(), Err(XcRegionsError::MetadataNotFound));

			Ok(())
		}
	}
}
