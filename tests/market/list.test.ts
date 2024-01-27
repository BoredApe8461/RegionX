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
import { approveTransfer, createRegionCollection, expectOnSale, initRegion, mintRegion } from '../common';

use(chaiAsPromised);

const REGION_COLLECTION_ID = 42;
const LISTING_DEPOIST = 100;

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
      (await marketFactory.new(xcRegions.address, LISTING_DEPOIST)).address,
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

    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);

    await initRegion(api, xcRegions, alice, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(alice).tx.approve(market.address, id, true);

    const bitPrice = 50;
    await market
      .withSigner(alice)
      .tx.listRegion(id, bitPrice, alice.address, 0, 0, { value: LISTING_DEPOIST });

    await expectOnSale(market, id, alice, bitPrice);
    expect((await market.query.regionPrice(id)).value.unwrap().ok.toNumber()).to.be.equal(
      bitPrice * 80,
    );
    expect((await xcRegions.query.ownerOf(id)).value.unwrap()).to.deep.equal(market.address);
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

    const bitPrice = 50;
    const result = market.withSigner(alice).query.listRegion(id, bitPrice, alice.address, 0, 0);
    expect((await result).value.unwrap().err).to.deep.equal(MarketErrorBuilder.MissingDeposit());
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

    const bitPrice = 50;
    const result = await market
      .withSigner(alice)
      .query.listRegion(id, bitPrice, alice.address, 0, 0, { value: LISTING_DEPOIST });
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

    setTimeout(async () => {
      const bitPrice = 50;
      const result = await market
        .withSigner(alice)
        .query.listRegion(id, bitPrice, alice.address, 0, 0, { value: LISTING_DEPOIST });
      expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.RegionExpired());
    }, 6000);
  });
});
