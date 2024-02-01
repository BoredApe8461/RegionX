import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/coretime_market';

export interface RegionListed {
	regionId: ReturnNumber;
	timeslicePrice: ReturnNumber;
	seller: ReturnTypes.AccountId;
	saleRecepient: ReturnTypes.AccountId;
	metadataVersion: number;
}

export interface RegionPurchased {
	regionId: ReturnNumber;
	buyer: ReturnTypes.AccountId;
	totalPrice: ReturnNumber;
}

