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

use crate::{mock::MockExtension, REGIONS_COLLECTION_ID};
use ink::env::{
	test::{default_accounts, DefaultAccounts},
	DefaultEnvironment,
};
use primitives::{
	assert_ok,
	coretime::RegionId,
	uniques::{CollectionId, ItemDetails},
};

#[ink::test]
fn chain_extensions_helper_functions_work() {
	let mut mock = MockExtension::default();
	let DefaultAccounts::<DefaultEnvironment> { alice, .. } = get_default_accounts();

	// State should be empty since we haven't yet minted any regions.
	assert!(mock.items.get(region_id(0)).is_none());
	assert!(mock.account.get(alice).is_none());

	// 1. Ensure mint works:

	assert_ok!(mock.mint(region_id(0), alice));
	// Can't mint the same region twice:
	assert!(mock.mint(region_id(0), alice).is_err());

	assert_eq!(
		mock.items.get(region_id(0)),
		Some(ItemDetails {
			owner: alice,
			approved: None,
			is_frozen: false,
			deposit: Default::default()
		})
	);
	assert_eq!(mock.account.get(alice), Some(vec![region_id(0)]));

	// 2. Ensure burn works:

	// Mint one more new region:
	assert_ok!(mock.mint(region_id(1), alice));

	assert_ok!(mock.burn(region_id(0)));
	assert!(mock.items.get(region_id(0)).is_none());
	assert_eq!(mock.account.get(alice), Some(vec![region_id(1)]));

	assert_ok!(mock.burn(region_id(1)));
	assert!(mock.items.get(region_id(1)).is_none());
	assert!(mock.account.get(alice).is_none());

	assert!(mock.burn(region_id(1)).is_err());
}

fn region_id(region_id: RegionId) -> (CollectionId, RegionId) {
	(REGIONS_COLLECTION_ID, region_id)
}

fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
	default_accounts::<DefaultEnvironment>()
}
