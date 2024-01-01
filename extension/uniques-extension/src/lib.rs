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

use openbrush::traits::AccountId;
use primitives::{
	coretime::RawRegionId,
	uniques::{CollectionDetails, CollectionId, ItemDetails},
};
use scale::{Decode, Encode};

/// These are only the functions that are essential for the xc-regions contract. However, the
/// underlying chain extension is likely to implement many additional ones.
///
/// We will utilize the chain extension solely for state reads, as extrinsics can be executed
/// through `call_runtime`, which is more future-proof approach.
///
/// Once WASM view functions are supported, there will no longer be a need for a chain extension.
pub trait UniquesExtension {
	/// The owner of the specific item.
	fn owner(
		&self,
		collection_id: CollectionId,
		item_id: RawRegionId,
	) -> Result<Option<AccountId>, UniquesError> {
		::ink::env::chain_extension::ChainExtensionMethod::build(0x40001)
			.input::<(CollectionId, RawRegionId)>()
			.output::<Result<Option<AccountId>, UniquesError>, true>()
			.handle_error_code::<UniquesError>()
			.call(&(collection_id, item_id))
	}

	/// Returns the details of a collection.
	fn collection(
		&self,
		collection_id: CollectionId,
	) -> Result<Option<CollectionDetails>, UniquesError> {
		::ink::env::chain_extension::ChainExtensionMethod::build(0x40006)
			.input::<CollectionId>()
			.output::<Result<Option<CollectionDetails>, UniquesError>, true>()
			.handle_error_code::<UniquesError>()
			.call(&collection_id)
	}

	/// Returns the details of an item within a collection.
	fn item(
		&self,
		collection_id: CollectionId,
		item_id: RawRegionId,
	) -> Result<Option<ItemDetails>, UniquesError> {
		::ink::env::chain_extension::ChainExtensionMethod::build(0x40007)
			.input::<(CollectionId, RawRegionId)>()
			.output::<Result<Option<ItemDetails>, UniquesError>, true>()
			.handle_error_code::<UniquesError>()
			.call(&(collection_id, item_id))
	}
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UniquesError {
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
