import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface Id {
	u8 ? : (number | string | BN),
	u16 ? : (number | string | BN),
	u32 ? : (number | string | BN),
	u64 ? : (number | string | BN),
	u128 ? : (string | number | BN),
	bytes ? : Array<(number | string | BN)>
}

export class IdBuilder {
	static U8(value: (number | string | BN)): Id {
		return {
			u8: value,
		};
	}
	static U16(value: (number | string | BN)): Id {
		return {
			u16: value,
		};
	}
	static U32(value: (number | string | BN)): Id {
		return {
			u32: value,
		};
	}
	static U64(value: (number | string | BN)): Id {
		return {
			u64: value,
		};
	}
	static U128(value: (string | number | BN)): Id {
		return {
			u128: value,
		};
	}
	static Bytes(value: Array<(number | string | BN)>): Id {
		return {
			bytes: value,
		};
	}
}

export type Listing = {
	seller: AccountId,
	bitPrice: (string | number | BN),
	saleRecepient: AccountId,
	metadataVersion: (number | string | BN),
	listedAt: (number | string | BN)
}

export interface MarketError {
	arithmeticError ? : null,
	invalidRegionId ? : null,
	invalidTimeslice ? : null,
	regionExpired ? : null,
	missingDeposit ? : null,
	regionNotListed ? : null,
	insufficientFunds ? : null,
	metadataNotMatching ? : null,
	transferFailed ? : null,
	xcRegionsPsp34Error ? : PSP34Error,
	xcRegionsMetadataError ? : XcRegionsError
}

export class MarketErrorBuilder {
	static ArithmeticError(): MarketError {
		return {
			arithmeticError: null,
		};
	}
	static InvalidRegionId(): MarketError {
		return {
			invalidRegionId: null,
		};
	}
	static InvalidTimeslice(): MarketError {
		return {
			invalidTimeslice: null,
		};
	}
	static RegionExpired(): MarketError {
		return {
			regionExpired: null,
		};
	}
	static MissingDeposit(): MarketError {
		return {
			missingDeposit: null,
		};
	}
	static RegionNotListed(): MarketError {
		return {
			regionNotListed: null,
		};
	}
	static InsufficientFunds(): MarketError {
		return {
			insufficientFunds: null,
		};
	}
	static MetadataNotMatching(): MarketError {
		return {
			metadataNotMatching: null,
		};
	}
	static TransferFailed(): MarketError {
		return {
			transferFailed: null,
		};
	}
	static XcRegionsPsp34Error(value: PSP34Error): MarketError {
		return {
			xcRegionsPsp34Error: value,
		};
	}
	static XcRegionsMetadataError(value: XcRegionsError): MarketError {
		return {
			xcRegionsMetadataError: value,
		};
	}
}

export interface PSP34Error {
	custom ? : string,
	selfApprove ? : null,
	notApproved ? : null,
	tokenExists ? : null,
	tokenNotExists ? : null,
	safeTransferCheckFailed ? : string
}

export class PSP34ErrorBuilder {
	static Custom(value: string): PSP34Error {
		return {
			custom: value,
		};
	}
	static SelfApprove(): PSP34Error {
		return {
			selfApprove: null,
		};
	}
	static NotApproved(): PSP34Error {
		return {
			notApproved: null,
		};
	}
	static TokenExists(): PSP34Error {
		return {
			tokenExists: null,
		};
	}
	static TokenNotExists(): PSP34Error {
		return {
			tokenNotExists: null,
		};
	}
	static SafeTransferCheckFailed(value: string): PSP34Error {
		return {
			safeTransferCheckFailed: value,
		};
	}
}

export interface XcRegionsError {
	cannotInitialize ? : null,
	cannotRemove ? : null,
	metadataNotFound ? : null,
	invalidMetadata ? : null,
	versionNotFound ? : null,
	runtimeError ? : null,
	psp34 ? : PSP34Error
}

export class XcRegionsErrorBuilder {
	static CannotInitialize(): XcRegionsError {
		return {
			cannotInitialize: null,
		};
	}
	static CannotRemove(): XcRegionsError {
		return {
			cannotRemove: null,
		};
	}
	static MetadataNotFound(): XcRegionsError {
		return {
			metadataNotFound: null,
		};
	}
	static InvalidMetadata(): XcRegionsError {
		return {
			invalidMetadata: null,
		};
	}
	static VersionNotFound(): XcRegionsError {
		return {
			versionNotFound: null,
		};
	}
	static RuntimeError(): XcRegionsError {
		return {
			runtimeError: null,
		};
	}
	static Psp34(value: PSP34Error): XcRegionsError {
		return {
			psp34: value,
		};
	}
}

