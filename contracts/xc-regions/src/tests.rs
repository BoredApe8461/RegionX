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

use crate::{
	mock::{get_default_accounts, region_id, register_chain_extensions, MockExtension},
	xc_regions::XcRegions,
	REGIONS_COLLECTION_ID,
};
use ink::env::{test::DefaultAccounts, DefaultEnvironment};
use openbrush::contracts::psp34::{Id, PSP34};
use primitives::{assert_ok, uniques::ItemDetails};

#[ink::test]
fn mock_environemnt_helper_functions_work() {
	let DefaultAccounts::<DefaultEnvironment> { alice, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();

	// State should be empty since we haven't yet minted any regions.
	assert!(xc_regions.items.get(region_id(0)).is_none());
	assert!(xc_regions.account.get(alice).is_none());

	// 1. Ensure mint works:

	assert_ok!(xc_regions.mint(region_id(0), alice));
	// Can't mint the same region twice:
	assert!(xc_regions.mint(region_id(0), alice).is_err());

	assert_eq!(
		xc_regions.items.get(region_id(0)),
		Some(ItemDetails {
			owner: alice,
			approved: None,
			is_frozen: false,
			deposit: Default::default()
		})
	);
	assert_eq!(xc_regions.account.get(alice), Some(vec![region_id(0)]));

	// 2. Ensure burn works:

	// Mint one more new region:
	assert_ok!(xc_regions.mint(region_id(1), alice));

	assert_ok!(xc_regions.burn(region_id(0)));
	assert!(xc_regions.items.get(region_id(0)).is_none());
	assert_eq!(xc_regions.account.get(alice), Some(vec![region_id(1)]));

	assert_ok!(xc_regions.burn(region_id(1)));
	assert!(xc_regions.items.get(region_id(1)).is_none());
	assert!(xc_regions.account.get(alice).is_none());

	assert!(xc_regions.burn(region_id(1)).is_err());
}

#[ink::test]
fn psp34_implementation_works() {
	let mock = MockExtension::default();
	let DefaultAccounts::<DefaultEnvironment> { alice, bob, .. } = get_default_accounts();

	register_chain_extensions(mock);
	let mut xc_regions = XcRegions::new();

	// Initialize some state:
	assert_ok!(xc_regions.mint(region_id(0), alice));
	assert_ok!(xc_regions.mint(region_id(1), bob));

	// 1. Ensure `collection_id` works:
	assert_eq!(xc_regions.collection_id(), Id::U32(REGIONS_COLLECTION_ID));

	// 2. Ensure `owner_of` works:
	assert_eq!(xc_regions.owner_of(Id::U128(0)), Some(alice));
	assert_eq!(xc_regions.owner_of(Id::U128(1)), Some(bob));
	assert_eq!(xc_regions.owner_of(Id::U128(3)), None);

	// 3. Ensure `balance_of` works:
	assert_eq!(xc_regions.balance_of(alice), 1);
	assert_ok!(xc_regions.burn(region_id(1)));
	assert_eq!(xc_regions.balance_of(bob), 0);
}
