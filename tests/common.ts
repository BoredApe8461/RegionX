import { ApiPromise } from '@polkadot/api';
import { expect, use } from 'chai';
import { KeyringPair } from '@polkadot/keyring/types';
import XcRegions from '../types/contracts/xc_regions';
import Market from '../types/contracts/coretime_market';
import { Region } from 'coretime-utils';

const REGION_COLLECTION_ID = 42;

export async function createRegionCollection(api: ApiPromise, caller: KeyringPair): Promise<void> {
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

export async function initRegion(
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

export async function mintRegion(api: ApiPromise, caller: KeyringPair, region: Region): Promise<void> {
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

export async function approveTransfer(
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

export async function expectOnSale(market: Market, id: any, seller: KeyringPair, bitPrice: number) {
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

export async function balanceOf(api: ApiPromise, acc: string): Promise<number> {
    const account: any = (await api.query.system.account(acc)).toHuman();
    return parseHNString(account.data.free);
}

export function parseHNString(str: string): number {
  return parseInt(str.replace(/,/g, ''));
};
