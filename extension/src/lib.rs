#[obce::definition(id = 123)]
pub trait Trait {
    fn method(&self) -> u32;
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
#[obce::ink_lang::extension]
pub struct Extension;

// impl uniques_extension::UniquesExtension for Extension {}
impl Trait for Extension {}
