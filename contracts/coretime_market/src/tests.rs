use crate::{coretime_market::CoretimeMarket, types::Listing};
use ink::env::{
	test::{default_accounts, DefaultAccounts},
	DefaultEnvironment,
};
use openbrush::traits::BlockNumber;
use primitives::coretime::{CoreMask, Region, Timeslice, TIMESLICE_PERIOD};

#[ink::test]
fn calculate_region_price_works() {
	let DefaultAccounts::<DefaultEnvironment> { charlie, .. } = get_default_accounts();

	let market = CoretimeMarket::new(charlie, 0);
	// Works for regions which haven't yet started.

	// complete coremask, so 80 active bits.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(400) // 80 bits * 5
	);

	// 40 active bits
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(0, 40) },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(200) // 40 bits * 5
	);

	// Works for regions which started.
	advance_n_blocks(timeslice_to_block_number(4));
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(300) // 1/4th of the region is wasted, so the price is decreased proportionally.
	);

	// Wasting inactive bits shouldn't affect the price:
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(200)
	);

	// `listed_at` affects the price.
	//
	// NOTE: This is not a realistic scenario since the provided current block number is less than
	// `listed_at`.
	advance_n_blocks(timeslice_to_block_number(2)); // The current timeslice will be 6.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(200) // 1/2 wasted.
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
	advance_n_blocks(timeslice_to_block_number(2)); // The current timeslice will be 8.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(100)
	);

	// Expired region has no value:
	advance_n_blocks(timeslice_to_block_number(2)); // The current timeslice will be 10.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
			Listing {
				seller: charlie,
				bit_price: 5,
				sale_recipient: charlie,
				metadata_version: 0,
				listed_at: 0
			}
		),
		Ok(0)
	);
}

fn advance_n_blocks(n: u32) {
	for _ in 0..n {
		advance_block();
	}
}
fn advance_block() {
	ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
}

fn timeslice_to_block_number(timeslice: Timeslice) -> BlockNumber {
	timeslice * TIMESLICE_PERIOD
}

fn get_default_accounts() -> DefaultAccounts<DefaultEnvironment> {
	default_accounts::<DefaultEnvironment>()
}
