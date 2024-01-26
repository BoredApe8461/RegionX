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
	traits::RegionMetadata,
	types::{VersionedRegion, XcRegionsError},
	xc_regions::{RegionInitialized, RegionRemoved, XcRegions},
	REGIONS_COLLECTION_ID,
};
use ink::env::{
	test::{default_accounts, set_caller, DefaultAccounts},
	DefaultEnvironment,
};
use openbrush::contracts::psp34::{Id, PSP34};
use primitives::{
	assert_ok,
	coretime::{RawRegionId, Region},
	uniques::{CollectionId, ItemDetails},
	Version,
};

type Event = <XcRegions as ::ink::reflect::ContractEventBase>::Type;

#[ink::test]
fn mock_environment_helper_functions_work() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();

	let region_id_0 = region_id(0);

	// State should be empty since we haven't yet minted any regions.
	assert!(xc_regions.items.get(region_id_0).is_none());
	assert!(xc_regions.account.get(charlie).is_none());

	// 1. Ensure mint works:
	assert_ok!(xc_regions.mint(region_id_0, charlie));

	// Can't mint the same region twice:
	assert!(xc_regions.mint(region_id_0, charlie).is_err());

	assert_eq!(
		xc_regions.items.get(region_id_0),
		Some(ItemDetails {
			owner: charlie,
			approved: None,
			is_frozen: false,
			deposit: Default::default()
		})
	);
	assert_eq!(xc_regions.account.get(charlie), Some(vec![region_id_0]));

	// 2. Ensure burn works:

	// Mint one more new region:
	let region_id_1 = region_id(1);
	assert_ok!(xc_regions.mint(region_id_1, charlie));

	assert_ok!(xc_regions.burn(region_id_0));
	assert!(xc_regions.items.get(region_id_0).is_none());
	assert_eq!(xc_regions.account.get(charlie), Some(vec![region_id_1]));

	assert_ok!(xc_regions.burn(region_id_1));
	assert!(xc_regions.items.get(region_id_1).is_none());
	assert!(xc_regions.account.get(charlie).is_none());

	assert!(xc_regions.burn(region_id_1).is_err());
}

#[ink::test]
fn init_works() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, bob, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();
	let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();

	// 1. Cannot initialize a region that doesn't exist:
	assert_eq!(xc_regions.init(0, Region::default()), Err(XcRegionsError::CannotInitialize));

	// 2. Cannot initialize a region that is not owned by the caller
	assert_ok!(xc_regions.mint(region_id(0), charlie));

	set_caller::<DefaultEnvironment>(bob);
	assert_eq!(xc_regions.init(0, Region::default()), Err(XcRegionsError::CannotInitialize));

	set_caller::<DefaultEnvironment>(charlie);
	// 3. Initialization doesn't work with incorrect metadata:
	let invalid_metadata = Region { begin: 1, end: 2, core: 0, mask: Default::default() };
	assert_eq!(xc_regions.init(0, invalid_metadata), Err(XcRegionsError::InvalidMetadata));

	// 4. Initialization works with correct metadata and the right caller:
	assert_ok!(xc_regions.init(0, Region::default()));

	// The region gets transferred to the contract:
	assert_eq!(xc_regions._uniques_owner(0), Some(contract));

	// Charlie receives a wrapped region:
	assert_eq!(xc_regions.owner_of(Id::U128(0)), Some(charlie));
	assert_eq!(xc_regions.balance_of(charlie), 1);

	assert_eq!(xc_regions.regions.get(0), Some(Region::default()));
	assert_eq!(xc_regions.metadata_versions.get(0), Some(0));

	let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
	assert_init_event(&emitted_events.last().unwrap(), 0, Region::default(), 0);

	// 5. Calling init for an already initialized region will fail.
	assert_eq!(xc_regions.init(0, Region::default()), Err(XcRegionsError::CannotInitialize));
}

#[ink::test]
fn remove_works() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();
	set_caller::<DefaultEnvironment>(charlie);

	let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();

	// Cannot remove a region that doesn't exist.
	assert_eq!(xc_regions.remove(0), Err(XcRegionsError::CannotRemove));

	// Minting and initializing a region:
	assert_ok!(xc_regions.mint(region_id(0), charlie));
	assert_ok!(xc_regions.init(0, Region::default()));

	// The region gets transferred to the contract:
	assert_eq!(xc_regions._uniques_owner(0), Some(contract));

	// Charlie receives a wrapped region:
	assert_eq!(xc_regions.owner_of(Id::U128(0)), Some(charlie));
	assert_eq!(xc_regions.balance_of(charlie), 1);

	assert_eq!(xc_regions.regions.get(0), Some(Region::default()));
	assert_eq!(xc_regions.metadata_versions.get(0), Some(0));

	// Removing a region works:
	assert_ok!(xc_regions.remove(0));

	// The region gets transferred back to Charlie and the wrapped region gets burned.
	assert_eq!(xc_regions._uniques_owner(0), Some(charlie));
	assert_eq!(xc_regions.owner_of(Id::U128(0)), None);
	assert_eq!(xc_regions.balance_of(charlie), 0);

	// The metadata should to be removed, however the metadata version should be retained in the
	// contract for this region.
	assert_eq!(xc_regions.regions.get(0), None);
	assert_eq!(xc_regions.metadata_versions.get(0), Some(0));

	let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
	assert_removed_event(&emitted_events.last().unwrap(), 0);
}

#[ink::test]
fn get_metadata_works() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();
	set_caller::<DefaultEnvironment>(charlie);

	// Cannot get the metadata of a region that doesn't exist:
	assert_eq!(xc_regions.get_metadata(0), Err(XcRegionsError::MetadataNotFound));

	// Minting a region without initializing it.
	assert_ok!(xc_regions.mint(region_id(0), charlie));
	assert_eq!(xc_regions.get_metadata(0), Err(XcRegionsError::MetadataNotFound));

	assert_ok!(xc_regions.init(0, Region::default()));
	assert_eq!(
		xc_regions.get_metadata(0),
		Ok(VersionedRegion { version: 0, region: Region::default() })
	);
}

#[ink::test]
fn metadata_version_gets_updated() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, .. } = get_default_accounts();
	let mut xc_regions = XcRegions::new();
	set_caller::<DefaultEnvironment>(charlie);

	assert_ok!(xc_regions.mint(region_id(0), charlie));
	assert_ok!(xc_regions.init(0, Region::default()));
	assert_eq!(
		xc_regions.get_metadata(0),
		Ok(VersionedRegion { version: 0, region: Region::default() })
	);

	assert_ok!(xc_regions.remove(0));

	assert_ok!(xc_regions.init(0, Region::default()));
	assert_eq!(
		xc_regions.get_metadata(0),
		Ok(VersionedRegion { version: 1, region: Region::default() })
	);
}

// Helper functions for test
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

fn assert_removed_event(event: &ink::env::test::EmittedEvent, expected_region_id: RawRegionId) {
	let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
		.expect("encountered invalid contract event data buffer");
	if let Event::RegionRemoved(RegionRemoved { region_id }) = decoded_event {
		assert_eq!(region_id, expected_region_id, "encountered invalid RegionRemoved.region_id");
	} else {
		panic!("encountered unexpected event kind: expected a RegionRemoved event")
	}
}

pub fn region_id(region_id: RawRegionId) -> (CollectionId, RawRegionId) {
	(REGIONS_COLLECTION_ID, region_id)
}

pub fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
	default_accounts::<DefaultEnvironment>()
}
