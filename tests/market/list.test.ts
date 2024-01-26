import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { expect, use } from 'chai';
import { KeyringPair } from '@polkadot/keyring/types';
import XcRegions_Factory from '../../types/constructors/xc_regions';
import Market_Factory from '../../types/constructors/coretime_market';
import XcRegions from '../../types/contracts/xc_regions';
import Market from '../../types/contracts/coretime_market';
import chaiAsPromised from 'chai-as-promised';
import { CoreMask, Region, RegionId, RegionRecord } from 'coretime-utils';

use(chaiAsPromised);

const REGION_COLLECTION_ID = 42;

const wsProvider = new WsProvider('ws://127.0.0.1:9920');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

describe('Coretime market listing', () => {
  let api: ApiPromise;
  let alice: KeyringPair;
  let bob: KeyringPair;

  let xcRegions: XcRegions;
  let market: Market;

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });

    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');

    const xcRegionsFactory = new XcRegions_Factory(api, alice);
    xcRegions = new XcRegions((await xcRegionsFactory.new()).address, alice, api);

    const marketFactory = new Market_Factory(api, alice);
    market = new Market((await marketFactory.new(xcRegions.address, 0)).address, alice, api);
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

    await createRegionCollection(api, alice);
    await mintRegion(api, alice, region);
    await approveTransfer(api, alice, region, xcRegions.address);
  });
});

async function createRegionCollection(
  contractsApi: ApiPromise,
  caller: KeyringPair,
): Promise<void> {
  console.log(`Creating the region collection`);

  const createCollectionCall = contractsApi.tx.uniques.create(REGION_COLLECTION_ID, caller.address);

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
