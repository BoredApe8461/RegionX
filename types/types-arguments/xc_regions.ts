import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export type Region = {
	begin: (number | string | BN),
	end: (number | string | BN),
	core: (number | string | BN),
	mask: CoreMask
}

export type CoreMask = Array<(number | string | BN)>;

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

export type VersionedRegion = {
	version: (number | string | BN),
	region: Region
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

