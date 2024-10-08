import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { expect, use } from 'chai';
import { KeyringPair } from '@polkadot/keyring/types';
import XcRegions_Factory from '../../types/constructors/xc_regions';
import Market_Factory from '../../types/constructors/coretime_market';
import XcRegions from '../../types/contracts/xc_regions';
import Market from '../../types/contracts/coretime_market';
import chaiAsPromised from 'chai-as-promised';
import { CoreMask, Id, Region, RegionId, RegionRecord } from 'coretime-utils';
import { MarketErrorBuilder, PSP34ErrorBuilder } from '../../types/types-returns/coretime_market';
import {
  approveTransfer,

  balanceOf,
  createRegionCollection,
  expectEvent,
  expectOnSale,
  initRegion,
  mintRegion,
  wait,
} from '../common';

use(chaiAsPromised);

const REGION_COLLECTION_ID = 42;
const LISTING_DEPOIST = 100;
// In reality this is 80, however we use 8 for testing.
const TIMESLICE_PERIOD = 8;

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519', ss58Format: 5 });

describe('Coretime market listing', () => {
  let api: ApiPromise;
  let alice: KeyringPair;

  let xcRegions: XcRegions;
  let market: Market;

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true, types: { Id } });

    alice = keyring.addFromUri('//Alice');

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

  it('Listing works', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 0,
      mask: CoreMask.completeMask(),
    };
    const regionRecord: RegionRecord = {
      end: 60,
      owner: alice.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);
    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });

    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);

    await initRegion(api, xcRegions, alice, region);

    await xcRegions.withSigner(alice).tx.approve(market.address, id, true);

    const aliceBalance = await balanceOf(api, alice.address);

    const timeslicePrice = 50;
    const result = await market
      .withSigner(alice)
      .tx.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });
    expectEvent(result, 'RegionListed', {
      regionId: id.toPrimitive().u128,
      timeslicePrice: timeslicePrice.toString(),
      seller: alice.address,
      saleRecepient: alice.address.toString(),
      metadataVersion: 0,
    });

    await expectOnSale(market, id, alice, timeslicePrice);

    expect((await market.query.regionPrice(id)).value.unwrap().unwrap().toNumber()).to.be.equal(
      timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);
    expect(await balanceOf(api, alice.address)).to.be.lessThan(aliceBalance - LISTING_DEPOIST);
  });

  it('Listing requires listing deposit', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 1,
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

    const timeslicePrice = 50;

    const result = await market
      .withSigner(alice)
      .query.listRegion(id, timeslicePrice, alice.address);
    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.MissingDeposit());

  });

  it('Listing requires region to be approved to the market', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 2,
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

    const timeslicePrice = 50;
    const result = await market
      .withSigner(alice)
      .query.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });
    expect(result.value.unwrap().err).to.deep.equal(
      MarketErrorBuilder.XcRegionsPsp34Error(PSP34ErrorBuilder.NotApproved()),
    );
  });

  it('Listing expired region fails', async () => {
    const regionId: RegionId = {
      begin: 0,
      core: 3,
      mask: CoreMask.completeMask(),
    };
    const regionRecord: RegionRecord = {
      end: 1,
      owner: alice.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);

    await initRegion(api, xcRegions, alice, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(alice).tx.approve(market.address, id, true);


    // Wait for the region to expire.
    await wait(2000 * TIMESLICE_PERIOD);

    const timeslicePrice = 50;
    const result = await market
      .withSigner(alice)
      .query.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });
    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.RegionExpired());

  });
});
