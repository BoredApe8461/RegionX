use crate::types::{Region, RegionId};

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RegionMetadataError {}

#[openbrush::wrapper]
pub type RegionMetadataRef = dyn RegionMetadata;

#[openbrush::trait_definition]
pub trait RegionMetadata {
    #[ink(message)]
    fn init(&mut self, id: RegionId, metadata: Region) -> Result<(), RegionMetadataError>;

    #[ink(message)]
    fn get_metadata(&self, id: RegionId) -> Result<Region, RegionMetadataError>;

    #[ink(message)]
    fn destroy(&mut self, id: RegionId) -> Result<(), RegionMetadataError>;
}
