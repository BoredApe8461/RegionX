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

use openbrush::{contracts::traits::psp34::PSP34Error, traits::AccountId};
use primitives::{Balance, Version};
use xc_regions::types::XcRegionsError;

#[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketError {
	/// An arithmetic error occured.
	ArithmeticError,
	/// The provided identifier is not a valid region id.
	InvalidRegionId,
	/// The specified region is expired.
	RegionExpired,
	/// The caller made the call without sending the required deposit amount.
	MissingDeposit,
	/// Caller tried to perform an action on a region that is not listed.
	RegionNotListed,
	/// The caller tried to purchase a region without sending enough tokens.
	InsufficientFunds,
	/// The metadata of the region doesn't match with what the caller expected.
	MetadataNotMatching,
	/// Failed to transfer the tokens to the seller.
	TransferFailed,
	/// The caller tried to perform an operation that they have no permission for.
	NotAllowed,
	/// An error occured when calling the xc-regions contract through the psp34 interface.
	XcRegionsPsp34Error(PSP34Error),
	/// An error occured when calling the xc-regions contract through the metadata interface.
	XcRegionsMetadataError(XcRegionsError),
}

impl core::fmt::Display for MarketError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			MarketError::ArithmeticError => write!(f, "ArithmeticError"),
			MarketError::InvalidRegionId => write!(f, "InvalidRegionId"),
			MarketError::RegionExpired => write!(f, "RegionExpired"),
			MarketError::MissingDeposit => write!(f, "MissingDeposit"),
			MarketError::RegionNotListed => write!(f, "RegionNotListed"),
			MarketError::InsufficientFunds => write!(f, "InsufficientFunds"),
			MarketError::MetadataNotMatching => write!(f, "MetadataNotMatching"),
			MarketError::TransferFailed => write!(f, "TransferFailed"),
			MarketError::NotAllowed => write!(f, "NotAllowed"),
			MarketError::XcRegionsPsp34Error(e) => write!(f, "{:?}", e),
			MarketError::XcRegionsMetadataError(e) => write!(f, "{}", e),
		}
	}
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Listing {
	/// The `AccountId` selling the specific region.
	pub seller: AccountId,
	/// The price per a single timeslice.
	pub timeslice_price: Balance,
	/// The `AccountId` receiving the payment from the sale.
	///
	/// If not set specified otherwise this should be the `seller` account.
	pub sale_recepient: AccountId,
	/// The metadata version of the region listed on sale. Used to prevent front running attacks.
	pub metadata_version: Version,
}
