use crate::{
	coretime_market::CoretimeMarket,
	types::{Listing, Moment},
};
use ink::env::{
	test::{default_accounts, set_caller, DefaultAccounts},
	DefaultEnvironment,
};
use openbrush::traits::BlockNumber;
use primitives::coretime::{CoreMask, Region, Timeslice, TIMESLICE_DURATION_IN_BLOCKS};

#[test]
fn calculate_region_price_works() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, .. } = get_default_accounts();

	// Works for regions which haven't yet started.

	// complete coremask, so 80 active bits.
	assert_eq!(
		CoretimeMarket::calculate_region_price(
			50, // current block number
			Listing {
				seller: charlie,
				region: Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: Moment { block_number: 0, timeslice: 0 }
			}
		),
		Ok(400) // 80 bits * 5
	);

	// 40 active bits
	assert_eq!(
		CoretimeMarket::calculate_region_price(
			50, // current block number
			Listing {
				seller: charlie,
				region: Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(0, 40) },
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: Moment { block_number: 0, timeslice: 0 }
			}
		),
		Ok(200) // 40 bits * 5
	);

	// Works for regions which started.
	assert_eq!(
		CoretimeMarket::calculate_region_price(
			timeslice_to_block_number(4), // current block number
			Listing {
				seller: charlie,
				region: Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: Moment { block_number: 0, timeslice: 0 }
			}
		),
		Ok(300) // 1/4th of the region is wasted, so the price is decreased proportionally.
	);

	// Wasting inactive bits shouldn't affect the price:
	assert_eq!(
		CoretimeMarket::calculate_region_price(
			timeslice_to_block_number(4), // current block number
			Listing {
				seller: charlie,
				region: Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: Moment { block_number: 0, timeslice: 0 }
			}
		),
		Ok(200)
	);

	// Timeline:
	//
	// 0 -- 2 --------6--------- 10
	// |    |         |           |
	// |    |         |           +-- Region end
	// |    |         +-------------- Active bits start from here
	// |    +------------------------ Region begin
	// +----------------------------- Timeslice 0
	//
	// 40 active bits out of which half is wasted.
	assert_eq!(
		CoretimeMarket::calculate_region_price(
			timeslice_to_block_number(8), // current block number
			Listing {
				seller: charlie,
				region: Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: Moment { block_number: 0, timeslice: 0 }
			}
		),
		Ok(100)
	);

	// Expired region has no value:
	assert_eq!(
		CoretimeMarket::calculate_region_price(
			timeslice_to_block_number(10), // current block number
			Listing {
				seller: charlie,
				region: Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: Moment { block_number: 0, timeslice: 0 }
			}
		),
		Ok(0)
	);
}

fn timeslice_to_block_number(timeslice: Timeslice) -> BlockNumber {
	timeslice * TIMESLICE_DURATION_IN_BLOCKS
}

fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
	default_accounts::<DefaultEnvironment>()
}
