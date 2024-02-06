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

#[cfg(test)]
mod tests;

mod types;

#[openbrush::contract(env = environment::ExtendedEnvironment)]
pub mod coretime_market {
	use crate::types::{Config, Listing, MarketError};
	use block_number_extension::BlockNumberProviderExtension;
	use environment::ExtendedEnvironment;
	use ink::{
		codegen::{EmitEvent, Env},
		prelude::vec::Vec,
		reflect::ContractEventBase,
		EnvAccess,
	};
	use openbrush::{contracts::traits::psp34::Id, storage::Mapping, traits::Storage};
	use primitives::{
		coretime::{RawRegionId, Region, Timeslice, CORE_MASK_BIT_LEN},
		ensure, Version,
	};
	use sp_arithmetic::{traits::SaturatedConversion, FixedPointNumber, FixedU128};
	use xc_regions::{traits::RegionMetadataRef, PSP34Ref};

	#[ink(storage)]
	#[derive(Storage)]
	pub struct CoretimeMarket {
		/// A mapping that holds information about each region listed on sale.
		pub listings: Mapping<RawRegionId, Listing>,
		/// A vector containing all the regions listed on sale.
		pub listed_regions: Vec<RawRegionId>,
		/// The configuration of the market. Set on contract initialization. Can't be changed
		/// afterwards.
		pub config: Config,
	}

	#[ink(event)]
	pub struct RegionListed {
		/// The identifier of the region that got listed on sale.
		#[ink(topic)]
		pub(crate) region_id: RawRegionId,
		/// The per timeslice price of the listed region.
		pub(crate) timeslice_price: Balance,
		/// The seller of the region
		pub(crate) seller: AccountId,
		/// The sale revenue recipient.
		pub(crate) sale_recepient: AccountId,
		/// The metadata version of the region.
		pub(crate) metadata_version: Version,
	}

	#[ink(event)]
	pub struct RegionUnlisted {
		/// The identifier of the region that got unlisted from sale.
		#[ink(topic)]
		pub(crate) region_id: RawRegionId,
		/// The account that removed the region from sale.
		pub(crate) caller: AccountId,
	}

	#[ink(event)]
	pub struct RegionPurchased {
		/// The identifier of the region that got purchased.
		#[ink(topic)]
		pub(crate) region_id: RawRegionId,
		/// The buyer of the region
		pub(crate) buyer: AccountId,
		/// The total price paid for the listed region.
		pub(crate) total_price: Balance,
	}

	#[ink(event)]
	pub struct RegionPriceUpdated {
		/// The identifier of the region that got its price updated.
		#[ink(topic)]
		pub(crate) region_id: RawRegionId,
		/// The new per timeslice price.
		pub(crate) new_timeslice_price: Balance,
	}

	impl CoretimeMarket {
		#[ink(constructor)]
		pub fn new(
			xc_regions_contract: AccountId,
			listing_deposit: Balance,
			timeslice_period: BlockNumber,
		) -> Self {
			Self {
				listings: Default::default(),
				listed_regions: Default::default(),
				config: Config { xc_regions_contract, listing_deposit, timeslice_period },
			}
		}

		#[ink(message)]
		pub fn xc_regions_contract(&self) -> AccountId {
			self.config.xc_regions_contract
		}

		#[ink(message)]
		pub fn listed_regions(&self) -> Vec<RawRegionId> {
			self.listed_regions.clone()
		}

		#[ink(message)]
		pub fn listed_region(&self, id: Id) -> Result<Option<Listing>, MarketError> {
			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };
			Ok(self.listings.get(&region_id))
		}

		#[ink(message)]
		pub fn region_price(&self, id: Id) -> Result<Balance, MarketError> {
			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };

			let metadata =
				RegionMetadataRef::get_metadata(&self.config.xc_regions_contract, region_id)
					.map_err(MarketError::XcRegionsMetadataError)?;
			let listing = self.listings.get(&region_id).ok_or(MarketError::RegionNotListed)?;

			self.calculate_region_price(metadata.region, listing)
		}

		/// A function for listing a region on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region that the caller intends to
		///   list for sale.
		/// - `timeslice_price`: The price per a single timeslice.
		/// - `sale_recepient`: The `AccountId` receiving the payment from the sale. If not
		///   specified this will be the caller.
		///
		/// Before making this call, the caller must first approve their region to the market
		/// contract, as it will be transferred to the contract when listed for sale.
		///
		/// This call is payable because listing a region requires a deposit from the user. This
		/// deposit will be returned upon unlisting the region from sale. The rationale behind this
		/// requirement is to prevent the contract state from becoming bloated with regions that
		/// have expired.
		#[ink(message, payable)]
		pub fn list_region(
			&mut self,
			id: Id,
			timeslice_price: Balance,
			sale_recepient: Option<AccountId>,
		) -> Result<(), MarketError> {
			let caller = self.env().caller();
			let market = self.env().account_id();

			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };

			// Ensure that the region exists and its metadata is set.
			let metadata =
				RegionMetadataRef::get_metadata(&self.config.xc_regions_contract, region_id)
					.map_err(MarketError::XcRegionsMetadataError)?;

			let current_timeslice = self.current_timeslice();

			// It doesn't make sense to list a region that expired.
			ensure!(metadata.region.end > current_timeslice, MarketError::RegionExpired);

			ensure!(
				self.env().transferred_value() == self.config.listing_deposit,
				MarketError::MissingDeposit
			);

			// Transfer the region to the market.
			PSP34Ref::transfer(
				&self.config.xc_regions_contract,
				market,
				id.clone(),
				Default::default(),
			)
			.map_err(MarketError::XcRegionsPsp34Error)?;

			let sale_recepient = sale_recepient.unwrap_or(caller);

			self.listings.insert(
				&region_id,
				&Listing {
					seller: caller,
					timeslice_price,
					sale_recepient,
					metadata_version: metadata.version,
				},
			);
			self.listed_regions.push(region_id);

			self.emit_event(RegionListed {
				region_id,
				timeslice_price,
				seller: caller,
				sale_recepient,
				metadata_version: metadata.version,
			});

			Ok(())
		}

		/// A function for unlisting a region on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region that the caller intends to
		///   unlist from sale.
		///
		/// In case the region is expired, this is callable by anyone and the caller will receive
		/// the listing deposit as a reward.
		#[ink(message)]
		pub fn unlist_region(&mut self, id: Id) -> Result<(), MarketError> {
			let caller = self.env().caller();

			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };

			let listing = self.listings.get(&region_id).ok_or(MarketError::RegionNotListed)?;
			let metadata =
				RegionMetadataRef::get_metadata(&self.config.xc_regions_contract, region_id)
					.map_err(MarketError::XcRegionsMetadataError)?;

			let current_timeslice = self.current_timeslice();

			// If the region is expired this is callable by anyone, otherwise only the seller can
			// unlist the region from the market.
			ensure!(
				caller == listing.seller || current_timeslice > metadata.region.end,
				MarketError::NotAllowed
			);

			// Transfer the region to the seller.
			PSP34Ref::transfer(
				&self.config.xc_regions_contract,
				listing.seller,
				id.clone(),
				Default::default(),
			)
			.map_err(MarketError::XcRegionsPsp34Error)?;

			// Remove the region from sale:
			self.remove_from_sale(region_id)?;

			// Reward the caller with listing deposit.
			self.env()
				.transfer(caller, self.config.listing_deposit)
				.map_err(|_| MarketError::TransferFailed)?;

			self.emit_event(RegionUnlisted { region_id, caller });

			Ok(())
		}

		/// A function for updating a listed region's price.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region being listed for sale.
		/// - `timeslice_price`: The new per timeslice price of the region.
		#[ink(message)]
		pub fn update_region_price(
			&mut self,
			id: Id,
			new_timeslice_price: Balance,
		) -> Result<(), MarketError> {
			let caller = self.env().caller();

			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };

			let mut listing = self.listings.get(&region_id).ok_or(MarketError::RegionNotListed)?;

			ensure!(caller == listing.seller, MarketError::NotAllowed);

			listing.timeslice_price = new_timeslice_price;
			self.listings.insert(&region_id, &listing);

			self.emit_event(RegionPriceUpdated { region_id, new_timeslice_price });
			Ok(())
		}

		/// A function for purchasing a region listed on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region being listed for sale.
		/// - `metadata_version`: The required metadata version for the region. If the
		///   `metadata_version` does not match the current version stored in the xc-regions
		///   contract the purchase will fail.
		///
		/// IMPORTANT NOTE: The client is responsible for ensuring that the metadata of the listed
		/// region is correct.
		#[ink(message, payable)]
		pub fn purchase_region(
			&mut self,
			id: Id,
			metadata_version: Version,
		) -> Result<(), MarketError> {
			let caller = self.env().caller();
			let transferred_value = self.env().transferred_value();

			let Id::U128(region_id) = id else { return Err(MarketError::InvalidRegionId) };
			let listing = self.listings.get(&region_id).ok_or(MarketError::RegionNotListed)?;

			let metadata =
				RegionMetadataRef::get_metadata(&self.config.xc_regions_contract, region_id)
					.map_err(MarketError::XcRegionsMetadataError)?;

			let price = self.calculate_region_price(metadata.region, listing.clone())?;
			ensure!(transferred_value >= price, MarketError::InsufficientFunds);

			ensure!(listing.metadata_version == metadata_version, MarketError::MetadataNotMatching);

			// Transfer the region to the buyer.
			PSP34Ref::transfer(
				&self.config.xc_regions_contract,
				caller,
				id.clone(),
				Default::default(),
			)
			.map_err(MarketError::XcRegionsPsp34Error)?;

			// Remove the region from sale:
			self.remove_from_sale(region_id)?;

			// Transfer the tokens to the sale recipient.
			self.env()
				.transfer(listing.sale_recepient, price)
				.map_err(|_| MarketError::TransferFailed)?;

			self.emit_event(RegionPurchased { region_id, buyer: caller, total_price: price });

			Ok(())
		}
	}

	// Internal functions:
	impl CoretimeMarket {
		pub(crate) fn calculate_region_price(
			&self,
			region: Region,
			listing: Listing,
		) -> Result<Balance, MarketError> {
			let current_timeslice = self.current_timeslice();

			let duration = region.end.saturating_sub(region.begin);

			let core_occupancy =
				FixedU128::checked_from_rational(region.mask.count_ones(), CORE_MASK_BIT_LEN)
					.ok_or(MarketError::ArithmeticError)?;

			let per_timeslice_price = (core_occupancy * listing.timeslice_price.into())
				.into_inner()
				.saturating_div(FixedU128::accuracy());

			if current_timeslice < region.begin {
				// The region didn't start yet, so there is no value lost.
				let price = per_timeslice_price.saturating_mul(duration.into());

				return Ok(price);
			}

			let remaining_timeslices = region.end.saturating_sub(current_timeslice);
			let price = per_timeslice_price.saturating_mul(remaining_timeslices.into());

			Ok(price)
		}

		// Remove a region from sale
		fn remove_from_sale(&mut self, region_id: RawRegionId) -> Result<(), MarketError> {
			let region_index = self
				.listed_regions
				.iter()
				.position(|r| *r == region_id)
				.ok_or(MarketError::RegionNotListed)?;

			self.listed_regions.remove(region_index);
			self.listings.remove(&region_id);

			Ok(())
		}

		#[cfg(not(test))]
		pub(crate) fn current_timeslice(&self) -> Timeslice {
			let latest_rc_block =
				self.env().extension().relay_chain_block_number().unwrap_or_default();
			(latest_rc_block / self.config.timeslice_period).saturated_into()
		}

		#[cfg(test)]
		pub(crate) fn current_timeslice(&self) -> Timeslice {
			let latest_block = self.env().block_number();
			(latest_block / self.config.timeslice_period).saturated_into()
		}

		fn emit_event<Event: Into<<CoretimeMarket as ContractEventBase>::Type>>(&self, e: Event) {
			<EnvAccess<'_, ExtendedEnvironment> as EmitEvent<CoretimeMarket>>::emit_event::<Event>(
				self.env(),
				e,
			);
		}
	}

	#[cfg(all(test, feature = "e2e-tests"))]
	pub mod tests {
		use super::*;
		use environment::ExtendedEnvironment;
		use ink_e2e::MessageBuilder;
		use primitives::coretime::TIMESLICE_PERIOD;
		use xc_regions::xc_regions::XcRegionsRef;

		type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

		const REQUIRED_DEPOSIT: Balance = 1_000;

		#[ink_e2e::test(environment = ExtendedEnvironment)]
		async fn constructor_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
			let constructor = XcRegionsRef::new();
			let xc_regions_acc_id = client
				.instantiate("xc-regions", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			let constructor =
				CoretimeMarketRef::new(xc_regions_acc_id, REQUIRED_DEPOSIT, TIMESLICE_PERIOD);
			let market_acc_id = client
				.instantiate("coretime-market", &ink_e2e::alice(), constructor, 0, None)
				.await
				.expect("instantiate failed")
				.account_id;

			let xc_regions_contract =
				MessageBuilder::<ExtendedEnvironment, CoretimeMarketRef>::from_account_id(
					market_acc_id.clone(),
				)
				.call(|market| market.xc_regions_contract());
			let xc_regions_contract =
				client.call_dry_run(&ink_e2e::alice(), &xc_regions_contract, 0, None).await;
			assert_eq!(xc_regions_contract.return_value(), xc_regions_acc_id);

			// There should be no regions listed on sale:
			let listed_regions =
				MessageBuilder::<ExtendedEnvironment, CoretimeMarketRef>::from_account_id(
					market_acc_id.clone(),
				)
				.call(|market| market.listed_regions());
			let listed_regions =
				client.call_dry_run(&ink_e2e::alice(), &listed_regions, 0, None).await;
			assert_eq!(listed_regions.return_value(), vec![]);

			Ok(())
		}
	}
}
