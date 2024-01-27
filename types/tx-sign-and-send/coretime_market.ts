/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/coretime_market';
import type BN from 'bn.js';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";
import EVENT_DATA_TYPE_DESCRIPTIONS from '../event-data/coretime_market.json';


export default class Methods {
	readonly __nativeContract : ContractPromise;
	readonly __keyringPair : KeyringPair;
	readonly __apiPromise: ApiPromise;

	constructor(
		apiPromise: ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
	}

	/**
	* xcRegionsContract
	*
	*/
	"xcRegionsContract" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "xcRegionsContract", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [], __options);
	}

	/**
	* listedRegions
	*
	*/
	"listedRegions" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "listedRegions", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [], __options);
	}

	/**
	* listedRegion
	*
	* @param { ArgumentTypes.Id } id,
	*/
	"listedRegion" (
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "listedRegion", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [id], __options);
	}

	/**
	* regionPrice
	*
	* @param { ArgumentTypes.Id } id,
	*/
	"regionPrice" (
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "regionPrice", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [id], __options);
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
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "listRegion", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [id, bitPrice, saleRecipient, currentTimeslice, currentTimesliceStart], __options);
	}

	/**
	* unlistRegion
	*
	* @param { (string | number | BN) } regionId,
	*/
	"unlistRegion" (
		regionId: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "unlistRegion", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [regionId], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateRegionPrice", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [regionId, newBitPrice], __options);
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
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "purchaseRegion", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [id, metadataVersion], __options);
	}

}