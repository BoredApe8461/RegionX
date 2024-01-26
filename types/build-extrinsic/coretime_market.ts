/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/coretime_market';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * xcRegionsContract
	 *
	*/
	"xcRegionsContract" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "xcRegionsContract", [], __options);
	}

	/**
	 * listedRegions
	 *
	*/
	"listedRegions" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "listedRegions", [], __options);
	}

	/**
	 * regionPrice
	 *
	 * @param { ArgumentTypes.Id } id,
	*/
	"regionPrice" (
		id: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "regionPrice", [id], __options);
	}

	/**
	 * listRegion
	 *
	 * @param { ArgumentTypes.Id } id,
	 * @param { (string | number | BN) } bitPrice,
	 * @param { ArgumentTypes.AccountId | null } saleRecipient,
	 * @param { (number | string | BN) } currentTimeslice,
	 * @param { (number | string | BN) } currentTimesliceStart,
	*/
	"listRegion" (
		id: ArgumentTypes.Id,
		bitPrice: (string | number | BN),
		saleRecipient: ArgumentTypes.AccountId | null,
		currentTimeslice: (number | string | BN),
		currentTimesliceStart: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "listRegion", [id, bitPrice, saleRecipient, currentTimeslice, currentTimesliceStart], __options);
	}

	/**
	 * unlistRegion
	 *
	 * @param { (string | number | BN) } regionId,
	*/
	"unlistRegion" (
		regionId: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "unlistRegion", [regionId], __options);
	}

	/**
	 * updateRegionPrice
	 *
	 * @param { (string | number | BN) } regionId,
	 * @param { (string | number | BN) } newBitPrice,
	*/
	"updateRegionPrice" (
		regionId: (string | number | BN),
		newBitPrice: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateRegionPrice", [regionId, newBitPrice], __options);
	}

	/**
	 * purchaseRegion
	 *
	 * @param { ArgumentTypes.Id } id,
	 * @param { (number | string | BN) } metadataVersion,
	*/
	"purchaseRegion" (
		id: ArgumentTypes.Id,
		metadataVersion: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "purchaseRegion", [id, metadataVersion], __options);
	}

}