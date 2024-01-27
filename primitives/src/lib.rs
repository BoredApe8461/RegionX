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

pub mod coretime;
pub mod macros;
pub mod uniques;

/// Balance of an account.
pub type Balance = u128;

/// The type used for versioning metadata.
pub type Version = u32;

#[derive(scale::Encode, scale::Decode)]
pub enum RuntimeCall {
	// NOTE: on shibuya this is 37. in local-runtime this is 30.
	#[codec(index = 30)]
	Uniques(uniques::UniquesCall),
}
