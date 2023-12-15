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

#[openbrush::contract]
pub mod coretime_market {
	use openbrush::traits::Storage;

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct CoretimeMarket {
		foo: u8,
	}

	impl CoretimeMarket {
		#[ink(constructor)]
		pub fn new() -> Self {
			Default::default()
		}

		#[ink(message)]
		pub fn foo(&self) {}
	}
}
