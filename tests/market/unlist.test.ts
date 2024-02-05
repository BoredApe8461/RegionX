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
  wait,
} from '../common';
import { MarketErrorBuilder } from '../../types/types-returns/coretime_market';

use(chaiAsPromised);

const REGION_COLLECTION_ID = 42;
const LISTING_DEPOIST = 5 * Math.pow(10, 15);

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519', ss58Format: 5 });

describe('Coretime market unlisting', () => {
  let api: ApiPromise;
  let alice: KeyringPair;
  let bob: KeyringPair;

  let xcRegions: XcRegions;
  let market: Market;

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true, types: { Id } });

    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');

    const xcRegionsFactory = new XcRegions_Factory(api, alice);
    xcRegions = new XcRegions((await xcRegionsFactory.new()).address, alice, api);

    const marketFactory = new Market_Factory(api, alice);
    market = new Market(
      (await marketFactory.new(xcRegions.address, LISTING_DEPOIST)).address,
      alice,
      api,
    );

    if (!(await api.query.uniques.class(REGION_COLLECTION_ID)).toHuman()) {
      await createRegionCollection(api, alice);
    }
  });

  it('Unlisting works', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 20,
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

    const aliceBalance = await balanceOf(api, alice.address);

    const timeslicePrice = 5 * Math.pow(10, 12);
    await market
      .withSigner(alice)
      .tx.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });

    await expectOnSale(market, id, alice, timeslicePrice);
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    expect(await balanceOf(api, alice.address)).to.be.lessThan(aliceBalance - LISTING_DEPOIST);

    const result = await market.withSigner(alice).tx.unlistRegion(id);
    expectEvent(result, 'RegionUnlisted', {
      regionId: id.toPrimitive().u128,
      caller: alice.address,
    });

    // Ensure the region is removed from sale:
    expect(market.query.listedRegions()).to.eventually.be.equal([]);
    expect((await market.query.listedRegion(id)).value.unwrap().ok).to.be.equal(null);

    // Alice receives the region back:
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.be.equal(alice.address);
  });

  it('Unlisting not listed region fails', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 21,
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

    const result = await market.withSigner(alice).query.unlistRegion(id);
    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.RegionNotListed());
  });

  it('Only owner can unlist unexpired region', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 22,
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
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    const bobUnlistResult = await market.withSigner(bob).query.unlistRegion(id);
    expect(bobUnlistResult.value.unwrap().err).to.deep.equal(MarketErrorBuilder.NotAllowed());

    const aliceUnlistResult = await market.withSigner(alice).tx.unlistRegion(id);
    expectEvent(aliceUnlistResult, 'RegionUnlisted', {
      regionId: id.toPrimitive().u128,
      caller: alice.address,
    });

    // Ensure the region is removed from sale:
    expect(market.query.listedRegions()).to.eventually.be.equal([]);
    expect((await market.query.listedRegion(id)).value.unwrap().ok).to.be.equal(null);

    // Alice receives the region back:
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.be.equal(alice.address);
  });

  it('Anyone can unlist an expired region', async () => {
    const regionId: RegionId = {
      begin: 0,
      core: 23,
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

    const timeslicePrice = 5 * Math.pow(10, 12);
    await market
      .withSigner(alice)
      .tx.listRegion(id, timeslicePrice, alice.address, { value: LISTING_DEPOIST });

    await expectOnSale(market, id, alice, timeslicePrice);
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);
    /*

    const bobBalance = await balanceOf(api, bob.address);

    const result = await market.withSigner(bob).tx.unlistRegion(id);
    expectEvent(result, 'RegionUnlisted', {
      regionId: id.toPrimitive().u128,
      caller: bob.address,
    });

    // Ensure the region is removed from sale:
    expect(market.query.listedRegions()).to.eventually.be.equal([]);
    expect((await market.query.listedRegion(id)).value.unwrap().ok).to.be.equal(null);

    // Alice receives the region back:
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.be.equal(alice.address);

    // Bob receives the listing deposit:
    expect(await balanceOf(api, bob.address)).to.be.eq(bobBalance + LISTING_DEPOIST);
    */
  });
});
