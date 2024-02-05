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

	let market = CoretimeMarket::new(charlie, 0, TIMESLICE_PERIOD);
	// Works for regions which haven't yet started.

	// complete coremask, so 80 active bits.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
			Listing {
				seller: charlie,
				timeslice_price: 10,
				sale_recepient: charlie,
				metadata_version: 0,
			}
		),
		Ok(80) // 8 * 10
	);

	// 40 active bits, means the region only 'occupies' half of the core.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(0, 40) },
			Listing {
				seller: charlie,
				timeslice_price: 10,
				sale_recepient: charlie,
				metadata_version: 0,
			}
		),
		Ok(40) // (10 / 2) * 8
	);

	// Works for regions which started.
	advance_n_blocks(timeslice_to_block_number(4)); // the current timeslice will be 4.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::complete() },
			Listing {
				seller: charlie,
				timeslice_price: 10,
				sale_recepient: charlie,
				metadata_version: 0,
			}
		),
		// 1/4th of the region is wasted, so the price is decreased proportionally.
		Ok(60) // 10 * 6
	);

	// Expired region has no value:
	advance_n_blocks(timeslice_to_block_number(6)); // The current timeslice will be 10.
	assert_eq!(
		market.calculate_region_price(
			Region { begin: 2, end: 10, core: 0, mask: CoreMask::from_chunk(40, 80) },
			Listing {
				seller: charlie,
				timeslice_price: 10,
				sale_recepient: charlie,
				metadata_version: 0,
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
