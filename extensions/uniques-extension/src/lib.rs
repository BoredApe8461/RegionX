// This file is part of RegionX.
//
// RegionX is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// RegionX is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with RegionX.  If not, see <https://www.gnu.org/licenses/>.

use obce::substrate::{
    frame_system::Config as SysConfig,
    pallet_contracts::Config as ContractConfig,
    sp_core::H256,
    sp_runtime::traits::StaticLookup,
    ChainExtensionEnvironment,
    ExtensionContext,
};
use scale::{Decode, Encode};
use primitives::RegionId;
use crate::extension::{UniquesError, UniquesExtension};

mod extension;

#[derive(Default)]
pub struct Extension {}

#[obce::implementation]
impl<'a, E, T, Env> UniquesExtension for ExtensionContext<'a, E, T, Env, Extension>
where
    T: SysConfig<Hash = H256> + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    Env: ChainExtensionEnvironment<E, T>
{
    fn collection_owner(&self, collection_id: RegionId) -> Result<(), UniquesError> {
        todo!()
    }
}
