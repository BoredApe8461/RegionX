# RegionX

## 1. Introduction

[RegionX](https://regionx.tech/) is a project dedicated to developing components for the new [Agile Coretime](https://github.com/polkadot-fellows/RFCs/blob/main/text/0001-agile-coretime.md) model. The goal of the project is to enable developer teams, researchers, and speculators to start trading, tracking, and analyzing the product Polkadot offers - Coretime. 

This repository is establishing the smart contract components for creating a secondary Coretime market. This infrastructure is meant to be leveraged by any end-user product built around Coretime.

The repository currently contains two crucial components in the form of ink! contracts that are necessary for the creation of a flexible and decentralized Coretime market.

## 2. Core components

### 2.1 Cross-Chain Regions

From a procurement perspective, regions can be seen as NFT tokens representing ownership of Coretime. Each region is characterized by a defined set of attributes that encompass all its properties. The following is the list of these attributes:

- `begin`: Specifies the starting point of time from which a task assigned to the region can be scheduled on a core.
- `end`: Specifies the deadline until when a task assigned to the region can be scheduled on a core.
- `length`: The duration of a region. Always equals to `end - begin`.
- `core`: The core index to which the region belongs.
- `part`: The maximum core resources the region can utilize within a relay chain block.
- `owner`: The owner of the region.
- `paid`: The payment for the region on the bulk market. Defined only for renewable regions. This is used to calculate the renewal price in the next bulk sale.

The module containing all region-related logic is the [pallet-broker](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/broker). Deployed as part of the Coretime chain runtime, this pallet serves to handle all the [Agile Coretime](https://github.com/polkadot-fellows/RFCs/blob/main/text/0001-agile-coretime.md) logic in Polkadot.

Given that the Coretime chain does not have any kind of smart-contract support it is not possible to create a secondary market on the Coretime chain itself. 
For this reason we have to store all the secondary market logic on a separate parachain which supports contract deployment.
As cross-chain NFT transfers are not currently supported between parachains, we are implementing additional custom logic within the Cross-Chain Regions (`xcRegions` for short) contract.

Before discussing the specifics of `xcRegions`, let's first outline why NFT transfers are not supported compared to regular token transfers. 

Within `xcm-pallet` we have the `reserve_transfer_assets` extrinsic which is meant to be used for performing reserve based transfers between chains. The only asset related information these reserve transfers carry between chains is the asset identifier. This is sufficient for fungible assets but not feasible for non-fungible assets due to the additional metadata they carry.

Luckily enough the identifier associated with each region isn't just an incremental  `u128`. Each `RegionId` actually contains the following information encoded into a `u128`:

```rust
/// Self-describing identity for a Region of Bulk Coretime.
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RegionId {
	/// The timeslice at which this Region begins.
	pub begin: Timeslice,
	/// The index of the Polakdot Core on which this Region will be scheduled.
	pub core: CoreIndex,
	/// The regularity parts in which this Region will be scheduled.
	pub mask: CoreMask,
}
```

This solves part of the problem, but we are still missing the rest of the metadata. However, for creating a Coretime market, we don't need all the metadata of a region. The only missing piece from the market's perspective is the `end` of the region.

The workaround to solve this problem is to add a function to the `xcRegions` contract, allowing the owner of the region to set the `end` attribute. An obvious question here is how do we verify the validity of this data? The answer is, we don't.

Instead of verifying this data from the contract code, we offload this process to the frontend implementations. When querying cross-chain regions, the frontend will, for each of them, need to make an additional query to the Coretime chain to confirm whether the `end` stored on-chain is valid, i.e., matches the `end` property on the Coretime chain.

### 2.2 Coretime Marketplace

## 3. Develop


## 3. Develop

1. Make sure to have the latest [cargo contract](https://crates.io/crates/cargo-contract).
2. Clone the GitHub repository: 
```
https://github.com/RegionX-Labs/RegionX.git
```
 3. Compile and run unit tests
```
cd RegionX/
cargo build
cargo test
```
3. Build the contracts:
```
# To build the xc-regions contract:
cd contracts/xc-regions/
cargo contract build --release

# To build the xc-regions contract:
cd contracts/coretime-market/
cargo contract build --release
```
