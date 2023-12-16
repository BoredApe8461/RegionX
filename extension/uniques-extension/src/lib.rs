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
#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use openbrush::traits::AccountId;
use primitives::{
	coretime::RegionId,
	uniques::{CollectionId, ItemDetails},
};
use scale::{Decode, Encode};

/// These are only the functions are essential for the xc-regions contract. However, the underlying
/// chain extension is likely to implement many additional ones.
///
/// We will utilize the chain extension solely for state reads, as extrinsics can be executed
/// through `call_runtime`, which is more future-proof approach.
///
/// Once WASM view functions are supported, there will no longer be a need for a chain extension.
#[obce::definition(id = 123)]
pub trait UniquesExtension {
	/// The owner of the specific item.
	fn owner(
		&self,
		collection_id: CollectionId,
		item_id: RegionId,
	) -> Result<AccountId, UniquesError>;

	/// All items owned by `who`.
	fn owned(&self, who: AccountId) -> Result<Vec<(CollectionId, RegionId)>, UniquesError>;

	/// An item within a collection.
	//
	// Requires: https://github.com/paritytech/polkadot-sdk/pull/2727
	fn item(
		&self,
		collection_id: CollectionId,
		item_id: RegionId,
	) -> Result<Option<ItemDetails>, UniquesError>;
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UniquesError {
	/// The signing account has no permission to do the operation.
	NoPermission = 1,
	/// The given item ID is unknown.
	UnknownCollection = 2,
	/// The item ID has already been used for an item.
	AlreadyExists = 3,
	/// The owner turned out to be different to what was expected.
	WrongOwner = 4,
	/// Invalid witness data given.
	BadWitness = 5,
	/// The item ID is already taken.
	InUse = 6,
	/// The item or collection is frozen.
	Frozen = 7,
	/// The delegate turned out to be different to what was expected.
	WrongDelegate = 8,
	/// There is no delegate approved.
	NoDelegate = 9,
	/// No approval exists that would allow the transfer.
	Unapproved = 10,
	/// The named owner has not signed ownership of the collection is acceptable.
	Unaccepted = 11,
	/// The item is locked.
	Locked = 12,
	/// All items have been minted.
	MaxSupplyReached = 13,
	/// The max supply has already been set.
	MaxSupplyAlreadySet = 14,
	/// The provided max supply is less than the amount of items a collection already has.
	MaxSupplyTooSmall = 15,
	/// The given item ID is unknown.
	UnknownItem = 16,
	/// Item is not for sale.
	NotForSale = 17,
	/// The provided bid is too low.
	BidTooLow = 18,
	/// Origin Caller is not supported
	OriginCannotBeCaller = 98,
	/// Unknown error
	RuntimeError = 99,
	/// Unknow status code
	UnknownStatusCode,
	/// Encountered unexpected invalid SCALE encoding
	InvalidScaleEncoding,
}

impl ink::env::chain_extension::FromStatusCode for UniquesError {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
			1 => Err(Self::NoPermission),
			2 => Err(Self::UnknownCollection),
			3 => Err(Self::AlreadyExists),
			4 => Err(Self::WrongOwner),
			5 => Err(Self::BadWitness),
			6 => Err(Self::InUse),
			7 => Err(Self::Frozen),
			8 => Err(Self::WrongDelegate),
			9 => Err(Self::NoDelegate),
			10 => Err(Self::Unapproved),
			11 => Err(Self::Unaccepted),
			12 => Err(Self::Locked),
			13 => Err(Self::MaxSupplyReached),
			14 => Err(Self::MaxSupplyAlreadySet),
			15 => Err(Self::MaxSupplyTooSmall),
			16 => Err(Self::UnknownItem),
			17 => Err(Self::NotForSale),
			18 => Err(Self::BidTooLow),
			98 => Err(Self::OriginCannotBeCaller),
			99 => Err(Self::RuntimeError),
			_ => Err(Self::UnknownStatusCode),
		}
	}
}

impl From<scale::Error> for UniquesError {
	fn from(_: scale::Error) -> Self {
		UniquesError::InvalidScaleEncoding
	}
}
