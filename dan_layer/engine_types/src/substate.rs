//   Copyright 2022. The Tari Project
//
//   Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//   following conditions are met:
//
//   1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//   disclaimer.
//
//   2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//   following disclaimer in the documentation and/or other materials provided with the distribution.
//
//   3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//   products derived from this software without specific prior written permission.
//
//   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//   DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//   SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//   USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use serde::{Deserialize, Serialize};
use tari_template_abi::{decode, encode, Decode, Encode};
use tari_template_lib::models::{ComponentAddress, ComponentInstance, ResourceAddress};

use crate::resource::Resource;

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
pub struct Substate {
    substate: SubstateValue,
    version: u32,
}

impl Substate {
    pub fn new<T: Into<SubstateValue>>(substate: T) -> Self {
        Self {
            substate: substate.into(),
            version: 0,
        }
    }

    pub fn substate_value(&self) -> &SubstateValue {
        &self.substate
    }

    pub fn into_substate(self) -> SubstateValue {
        self.substate
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        encode(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        decode(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize)]
pub enum SubstateAddress {
    Component(ComponentAddress),
    Resource(ResourceAddress),
}
impl SubstateAddress {
    pub fn into_shard_id(self) -> [u8; 32] {
        match self {
            SubstateAddress::Component(addr) => addr.into_array(),
            SubstateAddress::Resource(addr) => addr.into_array(),
        }
    }
}

// TODO: ComponentAddress and ResourceAddress should probably be newtypes
// impl From<ComponentAddress> for SubstateAddress {
//     fn from(address: ComponentAddress) -> Self {
//         Self::Component(address)
//     }
// }

// impl From<ResourceAddress> for SubstateAddress {
//     fn from(address: ResourceAddress) -> Self {
//         Self::Resource(address)
//     }
// }

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
pub enum SubstateValue {
    Component(ComponentInstance),
    Resource(Resource),
}

impl From<ComponentInstance> for SubstateValue {
    fn from(component: ComponentInstance) -> Self {
        Self::Component(component)
    }
}

impl From<Resource> for SubstateValue {
    fn from(resource: Resource) -> Self {
        Self::Resource(resource)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubstateDiff {
    up_substates: Vec<(SubstateAddress, Substate)>,
    down_substates: Vec<SubstateAddress>,
}

impl SubstateDiff {
    pub fn new() -> Self {
        Self {
            up_substates: Vec::new(),
            down_substates: Vec::new(),
        }
    }

    pub fn up(&mut self, address: SubstateAddress, value: Substate) {
        self.up_substates.push((address, value));
    }

    pub fn down(&mut self, address: SubstateAddress) {
        self.down_substates.push(address);
    }

    pub fn up_iter(&self) -> impl Iterator<Item = &(SubstateAddress, Substate)> + '_ {
        self.up_substates.iter()
    }

    pub fn down_iter(&self) -> impl Iterator<Item = &SubstateAddress> + '_ {
        self.down_substates.iter()
    }
}