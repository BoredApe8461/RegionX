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

mod types;

#[openbrush::contract]
pub mod coretime_market {
	use crate::types::{Listing, MarketError};
	use openbrush::{storage::Mapping, traits::Storage};
	use primitives::{coretime::RawRegionId, Version};

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct CoretimeMarket {
		/// A mapping that holds information about each region listed for sale.
		pub listings: Mapping<RawRegionId, Listing>,
	}

	impl CoretimeMarket {
		#[ink(constructor)]
		pub fn new() -> Self {
			Default::default()
		}

		/// A function for listing a region on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region that the caller intends to
		///   list for sale.
		/// - `bit_price`: The price for the smallest unit of the region. This is the price for a
		///   single bit of the region's coremask, i.e., 1/80th of the total price.
		#[ink(message)]
		pub fn list_region(
			&self,
			_region_id: RawRegionId,
			_bit_price: Balance,
		) -> Result<(), MarketError> {
			todo!()
		}

		/// A function for unlisting a region on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region that the caller intends to
		///   unlist from sale.
		#[ink(message)]
		pub fn unlist_region(&self, _region_id: RawRegionId) -> Result<(), MarketError> {
			todo!()
		}

		/// A function for updating a listed region's bit price.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region being listed for sale.
		/// - `bit_price`: The new price for the smallest unit of the region. This is the price for
		///   a single bit of the region's coremask, i.e., 1/80th of the total price.
		#[ink(message)]
		pub fn update_region_price(
			&self,
			_region_id: RawRegionId,
			_new_bit_price: Balance,
		) -> Result<(), MarketError> {
			todo!()
		}

		/// A function for purchasing a region listed on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The `u128` encoded identifier of the region being listed for sale.
		/// - `metadata_version`: The required metadata version for the region. If the
		///   `metadata_version` does not match the current version stored in the xc-regions
		///   contract the purchase will fail.
		#[ink(message)]
		pub fn purchase_region(
			&self,
			_region_id: RawRegionId,
			_metadata_version: Version,
		) -> Result<(), MarketError> {
			todo!()
		}
	}
}
