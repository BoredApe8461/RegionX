pub type RegionId = u128;

pub type Timeslice = u32;

/// Index of a Polkadot Core.
pub type CoreIndex = u16;

pub struct CoreMask([u8; 10]);

pub struct RegionMetadata {
    begin: Timeslice,
    end: Timeslice,
    core: CoreIndex,
    mask: CoreMask,
}
