pub type RegionId = u128;

pub type Timeslice = u32;

/// Index of a Polkadot Core.
pub type CoreIndex = u16;

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct CoreMask([u8; 10]);

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Region {
    begin: Timeslice,
    end: Timeslice,
    core: CoreIndex,
    mask: CoreMask,
}
