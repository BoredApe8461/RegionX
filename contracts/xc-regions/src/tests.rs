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
	mock::{get_default_accounts, region_id},
	traits::RegionMetadata,
	types::XcRegionsError,
	xc_regions::{RegionInitialized, XcRegions},
	REGIONS_COLLECTION_ID,
};
use ink::env::{
	test::{set_caller, DefaultAccounts},
	DefaultEnvironment,
};
use openbrush::contracts::psp34::{Id, PSP34};
use primitives::{
	assert_ok,
	coretime::{RawRegionId, Region},
	uniques::ItemDetails,
	Version,
};

type Event = <XcRegions as ::ink::reflect::ContractEventBase>::Type;

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
	let DefaultAccounts::<DefaultEnvironment> { alice, bob, .. } = get_default_accounts();
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

#[ink::test]
fn init_works() {
	let DefaultAccounts::<DefaultEnvironment> { alice, bob, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();

	// 1. Cannot initialize a region that doesn't exist:
	assert_eq!(xc_regions.init(0, Region::default()), Err(XcRegionsError::CannotInitialize));

	// 2. Cannot initialize a region that is not owned by the caller
	assert_ok!(xc_regions.mint(region_id(0), alice));
	set_caller::<DefaultEnvironment>(bob);
	assert_eq!(xc_regions.init(0, Region::default()), Err(XcRegionsError::CannotInitialize));

	set_caller::<DefaultEnvironment>(alice);
	// 3. Initialization doesn't work with incorrect metadata:
	let invalid_metadata = Region { begin: 1, end: 2, core: 0, mask: Default::default() };
	assert_eq!(xc_regions.init(0, invalid_metadata), Err(XcRegionsError::InvalidMetadata));

	// 4. Initialization works with correct metadata and the right caller:
	assert_ok!(xc_regions.init(0, Region::default()));

	assert_eq!(xc_regions.regions.get(0), Some(Region::default()));
	assert_eq!(xc_regions.metadata_versions.get(0), Some(0));

	let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
	assert_init_event(&emitted_events[0], 0, Region::default(), 0);

	// TODO: test version incrementation in a separate test.
}

// TODO: can probably make this a macro for all events to avoid code duplication.
fn assert_init_event(
	event: &ink::env::test::EmittedEvent,
	expected_region_id: RawRegionId,
	expected_metadata: Region,
	expected_version: Version,
) {
	let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
		.expect("encountered invalid contract event data buffer");
	if let Event::RegionInitialized(RegionInitialized { region_id, metadata, version }) =
		decoded_event
	{
		assert_eq!(
			region_id, expected_region_id,
			"encountered invalid RegionInitialized.region_id"
		);
		assert_eq!(metadata, expected_metadata, "encountered invalid RegionInitialized.metadata");
		assert_eq!(version, expected_version, "encountered invalid RegionInitialized.version");
	} else {
		panic!("encountered unexpected event kind: expected a RegionInitialized event")
	}
}
