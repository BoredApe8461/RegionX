# RegionX

## 1. Introduction

[RegionX](https://regionx.tech/) is a project dedicated to developing components for the new [Agile Coretime](https://github.com/polkadot-fellows/RFCs/blob/main/text/0001-agile-coretime.md) model. The goal of the project is to enable developer teams, researchers, and speculators to start trading, tracking, and analyzing the product Polkadot offers - Coretime. 

This repository is establishing the smart contract components for creating a secondary Coretime market. This infrastructure is meant to be leveraged by any end-user product built around Coretime.

The repository currently contains two crucial components in the form of ink! contracts that are necessary for the creation of a flexible and decentralized Coretime market.

## 2. Core components

### 2.1 Cross-Chain Regions

From a procurement perspective, regions can be seen as NFT tokens representing ownership of Coretime. Each region is characterized by a defined set of attributes that encompass all its properties. The following is the list of these attributes:

-   `begin`: Specifies the starting point of time from which a task assigned to the region can be scheduled on a core.
-   `end`: Specifies the deadline until a task assigned to the region can be scheduled on a core.
-   `length`: The duration of a region. Always equals to `end - begin`.
-   `core`: The core index to which the region belongs.
-   `mask`: The regularity parts in which this Region will be scheduled.
-   `owner`: The owner of the region.
-   `paid`: The payment for the region on the bulk market. Defined only for renewable regions. This is used to calculate the renewal price in the next bulk sale.

The module containing all region-related logic is the [pallet-broker](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/broker). Deployed as part of the Coretime chain runtime, this pallet serves to handle all the Bulk Coretime procurement logic in Polkadot.

Given that the Coretime chain does not have any kind of smart-contract support it is not possible to create a secondary market on the Coretime chain itself. 
For this reason we have to store all the secondary market logic on a separate parachain which supports contract deployment.
Given that cross-chain NFT metadata transfer is still something that hasn't been resolved in the ecosystem we are implementing additional custom logic that will enable the transfer of regions acrross chains. 

An explanation of our solution for transferring the metadata of regions acrross chains can be found on the RegionX wiki: [Cross-Chain Regions](https://regionx.gitbook.io/wiki/advanced/cross-chain-regions).

### 2.2 Coretime Marketplace

## 3. Develop

1. Make sure to have the latest [cargo contract](https://crates.io/crates/cargo-contract).
2. Clone the GitHub repository: 
```sh
git clone https://github.com/RegionX-Labs/RegionX.git
```
 3. Compile and run unit tests
```sh
cd RegionX/
cargo build
cargo test
```
3. Build the contracts:
```sh
# To build the xc-regions contract:
cd contracts/xc-regions/
cargo contract build --release

# To build the xc-regions contract:
cd contracts/coretime-market/
cargo contract build --release
```

4. Running e2e-tests
   
	Considering that the xc-regions contract necessitates the underlying chain to implement the uniques pallet for running e2e tests, it is required to specify a custom contracts node. For this purpose, we utilize the Astar local node from [Coretime-Mock](https://github.com/RegionX-Labs/Coretime-Mock) directory:
	```sh
	export CONTRACTS_NODE="~/Coretime-Mock/bin/astar-collator"
	```
	Once that is configured, we can proceed to run the e2e tests:"
	```sh
	cargo test --features e2e-tests
	```

## 4. Deploy

For the xc-regions contract to function correctly, the chain on which it is deployed must implement the uniques pallet. Given that the pallet index of the uniques pallet can vary across different chains, it's crucial to correctly configure this index before building and deploying the contract. To achieve this, the following steps should be taken:

1. Determine the index of the uniques pallet 
2.  Go to the `primitives/lib.rs` file:
3. Configure the index correctly: 
	```rust
	#[derive(scale::Encode, scale::Decode)]
	pub enum RuntimeCall {
		// E.g: on shibuya this is 37. in local-runtime this is 30.
		#[codec(index = <CORRECT PALLET INDEX>)]
		Uniques(uniques::UniquesCall),
	}
	```
Once this is correctly configured, the contract can then be deployed.
