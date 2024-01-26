import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/coretime_market';

export interface RegionListed {
	id: ReturnTypes.Id;
	bitPrice: ReturnNumber;
	seller: ReturnTypes.AccountId;
	saleRecipient: ReturnTypes.AccountId;
	metadataVersion: number;
}

export interface RegionPurchased {
	id: ReturnTypes.Id;
	buyer: ReturnTypes.AccountId;
	totalPrice: ReturnNumber;
}

