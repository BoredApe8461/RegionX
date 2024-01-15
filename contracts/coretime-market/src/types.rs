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

use openbrush::traits::AccountId;
use primitives::Balance;

#[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketError {
	Foo,
}

impl core::fmt::Display for MarketError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			MarketError::Foo => write!(f, "Foo"),
		}
	}
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Listing {
	/// The `AccountId` selling the specific region.
	pub seller: AccountId,
	/// The bit price of a region.
	pub bit_price: Balance,
	/// The `AccountId` receiving the payment from the sale.
	///
	/// If not set specified otherwise this should be the `seller` account.
	pub sale_recepient: AccountId,
}
