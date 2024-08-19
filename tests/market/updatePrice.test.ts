import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { expect, use } from 'chai';
import { KeyringPair } from '@polkadot/keyring/types';
import XcRegions_Factory from '../../types/constructors/xc_regions';
import Market_Factory from '../../types/constructors/coretime_market';
import XcRegions from '../../types/contracts/xc_regions';
import Market from '../../types/contracts/coretime_market';
import chaiAsPromised from 'chai-as-promised';
import { CoreMask, Id, Region, RegionId, RegionRecord } from 'coretime-utils';
import {
  approveTransfer,
  balanceOf,
  createRegionCollection,
  expectEvent,
  expectOnSale,
  initRegion,
  mintRegion,
} from '../common';
import { MarketErrorBuilder } from '../../types/types-returns/coretime_market';

use(chaiAsPromised);

const REGION_COLLECTION_ID = 42;
const LISTING_DEPOIST = 0;
// In reality this is 80, however we use 8 for testing.
const TIMESLICE_PERIOD = 8;

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519', ss58Format: 5 });

describe('Coretime market purchases', () => {
  let api: ApiPromise;
  let alice: KeyringPair;
  let bob: KeyringPair;
  let charlie: KeyringPair;

  let xcRegions: XcRegions;
  let market: Market;

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true, types: { Id } });

    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
    charlie = keyring.addFromUri('//Charlie');

    const xcRegionsFactory = new XcRegions_Factory(api, alice);
    xcRegions = new XcRegions((await xcRegionsFactory.new()).address, alice, api);

    const marketFactory = new Market_Factory(api, alice);
    market = new Market(
      (await marketFactory.new(xcRegions.address, LISTING_DEPOIST, TIMESLICE_PERIOD)).address,
      alice,
      api,
    );

    if (!(await api.query.uniques.class(REGION_COLLECTION_ID)).toHuman()) {
      await createRegionCollection(api, alice);
    }
  });

  it('Updating price works', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 40,
      mask: CoreMask.completeMask(),
    };
    const regionRecord: RegionRecord = {
      end: 60,
      owner: alice.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);

    await initRegion(api, xcRegions, alice, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(alice).tx.approve(market.address, id, true);

    const timeslicePrice = 5 * Math.pow(10, 12);
    await market
      .withSigner(alice)
      .tx.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });

    await expectOnSale(market, id, alice, timeslicePrice);
    expect((await market.query.regionPrice(id)).value.unwrap().unwrap().toNumber()).to.be.equal(
      timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    const newTimeslicePrice = 6 * Math.pow(10, 12);

    const result = await market.withSigner(alice).tx.updateRegionPrice(id, newTimeslicePrice);
    expectEvent(result, 'RegionPriceUpdated', {
      regionId: id.toPrimitive().u128,
      newTimeslicePrice: newTimeslicePrice.toString(),
    });
    await expectOnSale(market, id, alice, newTimeslicePrice);
    expect((await market.query.regionPrice(id)).value.unwrap().unwrap().toNumber()).to.be.equal(
      newTimeslicePrice * (region.getEnd() - region.getBegin()),
    );
  });

  it('Cannot update price for unlisted region', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 41,
      mask: CoreMask.completeMask(),
    };
    const regionRecord: RegionRecord = {
      end: 60,
      owner: alice.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);

    await initRegion(api, xcRegions, alice, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(alice).tx.approve(market.address, id, true);

    const newTimeslicePrice = 6 * Math.pow(10, 12);

    const result = await market.withSigner(alice).query.updateRegionPrice(id, newTimeslicePrice);

    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.RegionNotListed());
  });

  it('Only owner can update the price', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 42,
      mask: CoreMask.completeMask(),
    };
    const regionRecord: RegionRecord = {
      end: 60,
      owner: alice.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);

    await initRegion(api, xcRegions, alice, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(alice).tx.approve(market.address, id, true);

    const timeslicePrice = 7 * Math.pow(10, 12);
    await market
      .withSigner(alice)
      .tx.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });

    await expectOnSale(market, id, alice, timeslicePrice);
    expect((await market.query.regionPrice(id)).value.unwrap().unwrap().toNumber()).to.be.equal(
      timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    const newTimeslicePrice = 6 * Math.pow(10, 12);

    const result = await market.withSigner(bob).query.updateRegionPrice(id, newTimeslicePrice);

    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.NotAllowed());
  });
});
