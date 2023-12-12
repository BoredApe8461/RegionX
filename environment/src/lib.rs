use ink::env::{DefaultEnvironment, Environment};

/// Our custom environment diverges from the `DefaultEnvironment` in the event topics
/// limit.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ExtendedEnvironment {}

impl Environment for ExtendedEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    // TODO: add uniques chain extension
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}
