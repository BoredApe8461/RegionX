#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FixedMetadataExtensionError {}

// TODO: make this an openbrush::trait_definition
pub trait FixedMetadataExtension<Id, Metadata> {
    fn init(&mut self, id: Id, metadata: Metadata) -> Result<(), FixedMetadataExtensionError>;

    fn get_metadata(&self, id: Id) -> Result<Metadata, FixedMetadataExtensionError>;

    fn destroy(&mut self, id: Id) -> Result<(), FixedMetadataExtensionError>;
}
