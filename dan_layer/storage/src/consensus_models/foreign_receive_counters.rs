//    Copyright 2023 The Tari Project
//    SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

use tari_dan_common_types::{optional::Optional, shard::Shard, ShardGroup};

use crate::{StateStoreReadTransaction, StateStoreWriteTransaction, StorageError};

#[derive(Debug, Clone)]
pub struct ForeignReceiveCounters {
    pub counters: HashMap<Shard, u64>,
}

impl Default for ForeignReceiveCounters {
    fn default() -> Self {
        Self::new()
    }
}

impl ForeignReceiveCounters {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn increment_group(&mut self, shard_group: ShardGroup) {
        for shard in shard_group.shard_iter() {
            *self.counters.entry(shard).or_default() += 1;
        }
    }

    /// Returns the counter for the provided shard. If the count does not exist, 0 is returned.
    pub fn get_count(&self, shard: &Shard) -> u64 {
        self.counters.get(shard).copied().unwrap_or_default()
    }
}

impl ForeignReceiveCounters {
    pub fn save<TTx: StateStoreWriteTransaction + ?Sized>(&self, tx: &mut TTx) -> Result<(), StorageError> {
        tx.foreign_receive_counters_set(self)?;
        Ok(())
    }

    pub fn get_or_default<TTx: StateStoreReadTransaction + ?Sized>(tx: &TTx) -> Result<Self, StorageError> {
        Ok(tx.foreign_receive_counters_get().optional()?.unwrap_or_default())
    }
}
