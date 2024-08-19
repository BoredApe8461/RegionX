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

use crate::types::{VersionedRegion, XcRegionsError};
use openbrush::contracts::traits::psp34::Id;
use primitives::coretime::Region;

#[openbrush::wrapper]
pub type RegionMetadataRef = dyn RegionMetadata;

/// This is based on: `<https://hackmd.io/@Szegoo/rkryxwdIp>`
#[openbrush::trait_definition]
pub trait RegionMetadata {
	#[ink(message)]
	fn init(&mut self, id: Id, metadata: Region) -> Result<(), XcRegionsError>;

	#[ink(message)]
	fn get_metadata(&self, id: Id) -> Result<VersionedRegion, XcRegionsError>;

	#[ink(message)]
	fn remove(&mut self, id: Id) -> Result<(), XcRegionsError>;
}
