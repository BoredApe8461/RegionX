/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/coretime_market';
import type * as ReturnTypes from '../types-returns/coretime_market';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* xcRegionsContract
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"xcRegionsContract" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "xcRegionsContract", [], __options , (result) => { return handleReturnType(result, getTypeDescription(10, 'coretime_market')); });
	}

	/**
	* listedRegions
	*
	* @returns { Result<Array<ReturnNumber>, ReturnTypes.LangError> }
	*/
	"listedRegions" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnNumber>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "listedRegions", [], __options , (result) => { return handleReturnType(result, getTypeDescription(11, 'coretime_market')); });
	}

	/**
	* regionPrice
	*
	* @param { ArgumentTypes.Id } id,
	* @returns { Result<Result<ReturnNumber, ReturnTypes.MarketError>, ReturnTypes.LangError> }
	*/
	"regionPrice" (
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnNumber, ReturnTypes.MarketError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "regionPrice", [id], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'coretime_market')); });
	}

	/**
	* listRegion
	*
	* @param { ArgumentTypes.Id } id,
	* @param { (string | number | BN) } bitPrice,
	* @param { ArgumentTypes.AccountId | null } saleRecipient,
	* @param { (number | string | BN) } currentTimeslice,
	* @param { (number | string | BN) } currentTimesliceStart,
	* @returns { Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> }
	*/
	"listRegion" (
		id: ArgumentTypes.Id,
		bitPrice: (string | number | BN),
		saleRecipient: ArgumentTypes.AccountId | null,
		currentTimeslice: (number | string | BN),
		currentTimesliceStart: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "listRegion", [id, bitPrice, saleRecipient, currentTimeslice, currentTimesliceStart], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'coretime_market')); });
	}

	/**
	* unlistRegion
	*
	* @param { (string | number | BN) } regionId,
	* @returns { Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> }
	*/
	"unlistRegion" (
		regionId: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "unlistRegion", [regionId], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'coretime_market')); });
	}

	/**
	* updateRegionPrice
	*
	* @param { (string | number | BN) } regionId,
	* @param { (string | number | BN) } newBitPrice,
	* @returns { Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> }
	*/
	"updateRegionPrice" (
		regionId: (string | number | BN),
		newBitPrice: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateRegionPrice", [regionId, newBitPrice], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'coretime_market')); });
	}

	/**
	* purchaseRegion
	*
	* @param { ArgumentTypes.Id } id,
	* @param { (number | string | BN) } metadataVersion,
	* @returns { Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> }
	*/
	"purchaseRegion" (
		id: ArgumentTypes.Id,
		metadataVersion: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.MarketError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "purchaseRegion", [id, metadataVersion], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'coretime_market')); });
	}

}