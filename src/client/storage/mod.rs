#[cfg(test)]
mod spec;

use std::sync::{Arc, RwLock};
use std::{collections::HashMap, ops::Deref};

use client::mock_client::MockClient;
use client::ClientStorage;
use errors::{ErrorKind, OAuthResult};

#[derive(Debug, Clone)]
pub struct MemoryStorage(pub Arc<RwLock<HashMap<String, MockClient>>>);

impl<'a> MemoryStorage {
    pub fn new() -> MemoryStorage {
        MemoryStorage(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn len(&self) -> OAuthResult<usize> {
        match self.0.read() {
            Ok(v) => Ok(v.len()),
            Err(e) => Err(e.into()),
        }
    }
}

impl<'a> Deref for MemoryStorage {
    type Target = RwLock<HashMap<String, MockClient>>;

    fn deref(&self) -> &RwLock<HashMap<String, MockClient>> {
        &self.0
    }
}

impl<'a> ClientStorage<'a, MockClient> for MemoryStorage {
    type Error = ErrorKind;

    fn set<K>(&mut self, lookup: K, value: MockClient) -> OAuthResult<Option<MockClient>>
    where
        K: Into<String>,
    {
        match self.0.write() {
            Ok(mut hash) => Ok(hash.insert(lookup.into(), value)),
            Err(e) => Err(e.into()),
        }
    }

    fn get<K>(&self, lookup: K) -> OAuthResult<MockClient>
    where
        K: Into<String>,
    {
        match self.0.read() {
            Ok(hash) => hash.get(&lookup.into())
                .map(|c| c.clone())
                .ok_or(ErrorKind::msg("No Client stored from the given lookup")),
            Err(e) => Err(e.into()),
        }
    }

    fn drop<K>(&mut self, lookup: K) -> OAuthResult<MockClient>
    where
        K: Into<String>,
    {
        match self.0.write() {
            Ok(mut hash) => hash.remove(&lookup.into())
                .ok_or(ErrorKind::msg("No Client stored from the given lookup")),
            Err(e) => Err(e.into()),
        }
    }

    fn has<K>(&self, _lookup: K) -> OAuthResult<bool>
    where
        K: Into<String>,
    {
        unimplemented!()
    }
}
