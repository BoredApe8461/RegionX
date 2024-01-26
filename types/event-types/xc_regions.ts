import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/xc_regions';

export interface RegionInitialized {
	regionId: ReturnNumber;
	metadata: ReturnTypes.Region;
	version: number;
}

export interface RegionRemoved {
	regionId: ReturnNumber;
}

