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
pub mod xc_regions {
    use openbrush::{contracts::psp34::extensions::metadata::*, traits::Storage};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct XcRegions {
        #[storage_field]
        metadata: Data,
    }

    impl PSP34 for XcRegions {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            todo!()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            todo!()
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            todo!()
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
            todo!()
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            todo!()
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
            todo!()
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            todo!()
        }
    }

    impl XcRegions {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
    }
}
