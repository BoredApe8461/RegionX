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

use(chaiAsPromised);

const REGION_COLLECTION_ID = 42;
const LISTING_DEPOIST = 100;

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519', ss58Format: 5 });

describe('Coretime market listing', () => {
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
      await createRegionCollection(api, bob);
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
      owner: bob.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, bob, region);
    await approveTransfer(api, bob, region, xcRegions.address);

    await initRegion(api, xcRegions, bob, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(bob).tx.approve(market.address, id, true);

    const bitPrice = 50;
    await market
      .withSigner(bob)
      .tx.listRegion(id, bitPrice, bob.address, 0, 0, { value: LISTING_DEPOIST });

    await expectOnSale(market, id, bob, bitPrice);
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
      owner: bob.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, bob, region);
    await approveTransfer(api, bob, region, xcRegions.address);

    await initRegion(api, xcRegions, bob, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(bob).tx.approve(market.address, id, true);

    const bitPrice = 50;
    const result = market.withSigner(bob).query.listRegion(id, bitPrice, bob.address, 0, 0);
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
      owner: bob.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, bob, region);
    await approveTransfer(api, bob, region, xcRegions.address);

    await initRegion(api, xcRegions, bob, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });

    const bitPrice = 50;
    const result = await market
      .withSigner(bob)
      .query.listRegion(id, bitPrice, bob.address, 0, 0, { value: LISTING_DEPOIST });
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
      owner: bob.address,
      paid: null,
    };
    const region = new Region(regionId, regionRecord);

    await mintRegion(api, bob, region);
    await approveTransfer(api, bob, region, xcRegions.address);

    await initRegion(api, xcRegions, bob, region);

    const id: any = api.createType('Id', { U128: region.getEncodedRegionId(api) });
    await xcRegions.withSigner(bob).tx.approve(market.address, id, true);

    setTimeout(async () => {
      const bitPrice = 50;
      const result = await market
        .withSigner(bob)
        .query.listRegion(id, bitPrice, bob.address, 0, 0, { value: LISTING_DEPOIST });
      expect(result.value.unwrap().err).to.deep.equal(MarketErrorBuilder.RegionExpired());
    }, 6000);
  });
});

async function createRegionCollection(api: ApiPromise, caller: KeyringPair): Promise<void> {
  console.log(`Creating the region collection`);

  const createCollectionCall = api.tx.uniques.create(REGION_COLLECTION_ID, caller.address);

  const callTx = async (resolve: () => void, reject: ({ reason }) => void) => {
    const unsub = await createCollectionCall.signAndSend(caller, ({ status, events }) => {
      if (status.isInBlock) {
        unsub();
        events.forEach(({ event: { method, section } }) => {
          if (section == 'system' && method == 'ExtrinsicFailed')
            reject({ reason: 'Creating collection failed' });
        });
        resolve();
      }
    });
  };

  return new Promise(callTx);
}

async function initRegion(
  api: ApiPromise,
  xcRegions: XcRegions,
  caller: KeyringPair,
  region: Region,
) {
  await xcRegions.withSigner(caller).tx.init(region.getEncodedRegionId(api), {
    begin: region.getBegin(),
    core: region.getCore(),
    end: region.getEnd(),
    // @ts-ignore
    mask: region.getMask().getMask(),
  });
}

async function mintRegion(api: ApiPromise, caller: KeyringPair, region: Region): Promise<void> {
  console.log(`Minting a region`);

  const rawRegionId = region.getEncodedRegionId(api);
  const mintCall = api.tx.uniques.mint(REGION_COLLECTION_ID, rawRegionId, caller.address);

  const callTx = async (resolve: () => void, reject: ({ reason }) => void) => {
    const unsub = await mintCall.signAndSend(caller, ({ status, events }) => {
      if (status.isInBlock) {
        unsub();
        events.forEach(({ event: { method, section } }) => {
          if (section == 'system' && method == 'ExtrinsicFailed')
            reject({ reason: 'Minting failed' });
        });
        resolve();
      }
    });
  };

  return new Promise(callTx);
}

async function approveTransfer(
  api: ApiPromise,
  caller: KeyringPair,
  region: Region,
  delegate: string,
): Promise<void> {
  console.log(`Approving region to ${delegate}`);

  const rawRegionId = region.getEncodedRegionId(api);
  const approveCall = api.tx.uniques.approveTransfer(REGION_COLLECTION_ID, rawRegionId, delegate);

  const callTx = async (resolve: () => void, reject: ({ reason }) => void) => {
    const unsub = await approveCall.signAndSend(caller, ({ status, events }) => {
      if (status.isInBlock) {
        unsub();
        events.forEach(({ event: { method, section } }) => {
          if (section == 'system' && method == 'ExtrinsicFailed')
            reject({ reason: 'Approving region failed' });
        });
        resolve();
      }
    });
  };

  return new Promise(callTx);
}

async function expectOnSale(market: Market, id: any, seller: KeyringPair, bitPrice: number) {
  expect(market.query.listedRegions()).to.eventually.be.equal([id]);
  expect((await market.query.listedRegion(id)).value.unwrap().ok.bitPrice).to.be.equal(bitPrice);
  expect((await market.query.listedRegion(id)).value.unwrap().ok.metadataVersion).to.be.equal(0);
  expect((await market.query.listedRegion(id)).value.unwrap().ok.seller).to.be.equal(
    seller.address,
  );
  expect((await market.query.listedRegion(id)).value.unwrap().ok.saleRecipient).to.be.equal(
    seller.address,
  );
}
