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
      (await marketFactory.new(xcRegions.address, LISTING_DEPOIST)).address,
      alice,
      api,
    );

    if (!(await api.query.uniques.class(REGION_COLLECTION_ID)).toHuman()) {
      await createRegionCollection(api, alice);
    }
  });

  it('Purchasing works', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 10,
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
    expect((await market.query.regionPrice(id)).value.unwrap().ok.toNumber()).to.be.equal(
      timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    const aliceBalance = await balanceOf(api, alice.address);
    const bobBalance = await balanceOf(api, bob.address);

    const result = await market
      .withSigner(bob)
      .tx.purchaseRegion(id, 0, { value: timeslicePrice * (region.getEnd() - region.getBegin()) });
    expectEvent(result, 'RegionPurchased', {
      regionId: id.toPrimitive().u128,
      buyer: bob.address,
      totalPrice: (timeslicePrice * (region.getEnd() - region.getBegin())).toString(),
    });

    // Bob receives the region:
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.be.equal(bob.address);

    // Bob's balance is reduced:
    expect(await balanceOf(api, bob.address)).to.be.lessThan(
      bobBalance - timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    // Alice's balance is increased.
    expect(await balanceOf(api, alice.address)).to.be.greaterThan(aliceBalance);

    // Ensure the region is removed from sale:
    expect(market.query.listedRegions()).to.eventually.be.equal([]);
    expect((await market.query.listedRegion(id)).value.unwrap().ok).to.be.equal(null);
  });

  it('Purchasing fails when insufficient value is sent', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 11,
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
    expect((await market.query.regionPrice(id)).value.unwrap().ok.toNumber()).to.be.equal(
      timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    // Sending less tokens than supposed:
    const result = await market.withSigner(bob).query.purchaseRegion(id, 0, {
      value: timeslicePrice * (region.getEnd() - region.getBegin() - 1),
    });
    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.InsufficientFunds());
  });

  it('Purchasing fails when region is not listed', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 12,
      mask: CoreMask.completeMask(),
    };
    const regionRecord: RegionRecord = {
      end: 60,
      owner: alice.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);
    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });

    const timeslicePrice = 5 * Math.pow(10, 12);

    const result = await market.withSigner(bob).query.purchaseRegion(id, 0, {
      value: timeslicePrice * (region.getEnd() - region.getBegin()),
    });
    expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.RegionNotListed());
  });

  it('Purchasing sends tokens to sale recepient instead of seller account', async () => {
    const regionId: RegionId = {
      begin: 30,
      core: 13,
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
      .tx.listRegion(id, timeslicePrice, charlie.address, { value: LISTING_DEPOIST });

    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);

    const charlieBalance = await balanceOf(api, charlie.address);
    const bobBalance = await balanceOf(api, bob.address);

    const result = await market
      .withSigner(bob)
      .tx.purchaseRegion(id, 0, { value: timeslicePrice * (region.getEnd() - region.getBegin()) });
    expectEvent(result, 'RegionPurchased', {
      regionId: id.toPrimitive().u128,
      buyer: bob.address,
      totalPrice: (timeslicePrice * (region.getEnd() - region.getBegin())).toString(),
    });

    // Bob receives the region:
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.be.equal(bob.address);

    // Bob's balance is reduced:
    expect(await balanceOf(api, bob.address)).to.be.lessThan(
      bobBalance - timeslicePrice * (region.getEnd() - region.getBegin()),
    );
    // Charlie's balance is increased.
    expect(await balanceOf(api, charlie.address)).to.be.equal(
      charlieBalance + timeslicePrice * (region.getEnd() - region.getBegin()),
    );

    // Ensure the region is removed from sale:
    expect(market.query.listedRegions()).to.eventually.be.equal([]);
    expect((await market.query.listedRegion(id)).value.unwrap().ok).to.be.equal(null);
  });
});
