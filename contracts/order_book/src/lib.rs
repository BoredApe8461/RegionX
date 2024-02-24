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

//! Coretime market
//!
//! This is the contract implementation of a Coretime marketplace working on top of the `XcRegions`
//! contract.
//!
//! The contract employs a timeslice-based pricing model that determines the price of regions on
//! sale, based on the value of a single timeslice. This approach is useful as it allows us to
//! emulate the expiring nature of Coretime.
//!
//! ## Terminology:
//!
//! - Expired region: A region that can no longer be assigned to any particular task.
//! - Active region: A region which is currently able to perform a task. I.e. current timeslice >
//!   region.begin

#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

mod types;

#[openbrush::contract(env = environment::ExtendedEnvironment)]
pub mod coretime_market {
	use crate::types::MarketError;
	use block_number_extension::BlockNumberProviderExtension;
	use environment::ExtendedEnvironment;
	use ink::{
		codegen::{EmitEvent, Env},
		prelude::vec::Vec,
		reflect::ContractEventBase,
		EnvAccess,
	};
	use openbrush::{contracts::traits::psp34::Id, storage::Mapping, traits::Storage};
	use primitives::Version;
	use xc_regions::{traits::RegionMetadataRef, PSP34Ref};

	#[ink(storage)]
	#[derive(Storage)]
	pub struct CoretimeMarket {
		xc_regions_contract: AccountId,
	}

	impl CoretimeMarket {
		#[ink(constructor)]
		pub fn new(xc_regions_contract: AccountId) -> Self {
			Self { xc_regions_contract }
		}

		#[ink(message)]
		pub fn xc_regions_contract(&self) -> AccountId {
			self.xc_regions_contract
		}

		#[ink(message, payable)]
		pub fn list_region(&mut self, id: Id) -> Result<Version, MarketError> {
			let caller = self.env().caller();
			let market = self.env().account_id();

			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };

			// Ensure that the region exists and its metadata is set.
			let metadata = RegionMetadataRef::get_metadata(&self.xc_regions_contract, id.clone())
				.map_err(MarketError::XcRegionsMetadataError)?;

			Ok(metadata.version)
		}
	}
}
