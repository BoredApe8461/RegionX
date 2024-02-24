use xc_regions::types::XcRegionsError;

#[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketError {
	/// An arithmetic error occured.
	ArithmeticError,
	/// The provided identifier is not a valid region id.
	InvalidRegionId,
	/// The specified region is expired.
	RegionExpired,
	/// The caller made the call without sending the required deposit amount.
	MissingDeposit,
	/// Caller tried to perform an action on a region that is not listed.
	RegionNotListed,
	/// The caller tried to purchase a region without sending enough tokens.
	InsufficientFunds,
	/// The metadata of the region doesn't match with what the caller expected.
	MetadataNotMatching,
	/// Failed to transfer the tokens to the seller.
	TransferFailed,
	/// The caller tried to perform an operation that they have no permission for.
	NotAllowed,
	/// An error occured when calling the xc-regions contract through the metadata interface.
	XcRegionsMetadataError(XcRegionsError),
}

impl core::fmt::Display for MarketError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			MarketError::ArithmeticError => write!(f, "ArithmeticError"),
			MarketError::InvalidRegionId => write!(f, "InvalidRegionId"),
			MarketError::RegionExpired => write!(f, "RegionExpired"),
			MarketError::MissingDeposit => write!(f, "MissingDeposit"),
			MarketError::RegionNotListed => write!(f, "RegionNotListed"),
			MarketError::InsufficientFunds => write!(f, "InsufficientFunds"),
			MarketError::MetadataNotMatching => write!(f, "MetadataNotMatching"),
			MarketError::TransferFailed => write!(f, "TransferFailed"),
			MarketError::NotAllowed => write!(f, "NotAllowed"),
			MarketError::XcRegionsMetadataError(e) => write!(f, "{}", e),
		}
	}
}
