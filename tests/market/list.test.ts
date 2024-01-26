import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { expect, use } from 'chai';
import { KeyringPair } from '@polkadot/keyring/types';
import XcRegions_Factory from '../../types/constructors/xc_regions';
import Market_Factory from '../../types/constructors/coretime_market';
import XcRegions from '../../types/contracts/xc_regions';
import Market from '../../types/contracts/coretime_market';
import chaiAsPromised from 'chai-as-promised';

use(chaiAsPromised);

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
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

  it('Listing works', () => {
    
  });
});
