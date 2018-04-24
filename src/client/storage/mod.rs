// #[cfg(test)]
// mod spec;

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use client::mock_client::MockClient;
use client::ClientStorage;
use errors::{ErrorKind, OAuthResult};

#[derive(Debug, Clone)]
pub struct MockMemoryStorage(pub Arc<RwLock<HashMap<MockStorageKey, MockClient>>>);

impl MockMemoryStorage {
    pub fn new() -> MockMemoryStorage {
        MockMemoryStorage(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn len(&self) -> OAuthResult<usize> {
        match self.0.read() {
            Ok(v) => Ok(v.len()),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum MockStorageKey {
    State(String),
    Uid(usize),
}

impl MockStorageKey {
    pub fn state<S: Into<String>>(s: S) -> MockStorageKey {
        MockStorageKey::State(s.into())
    }

    pub fn uid(n: usize) -> MockStorageKey {
        MockStorageKey::Uid(n)
    }
}

impl Deref for MockMemoryStorage {
    type Target = RwLock<HashMap<MockStorageKey, MockClient>>;

    fn deref(&self) -> &RwLock<HashMap<MockStorageKey, MockClient>> {
        &self.0
    }
}

impl ClientStorage<MockClient> for MockMemoryStorage {
    type Error = ErrorKind;
    type Lookup = MockStorageKey;

    fn set(
        &mut self,
        lookup: MockStorageKey,
        value: MockClient,
    ) -> OAuthResult<Option<MockClient>> {
        match self.0.write() {
            Ok(ref mut hash) => Ok(hash.insert(lookup, value)),
            Err(e) => Err(e.into()),
        }
    }

    fn get(&self, lookup: MockStorageKey) -> OAuthResult<MockClient> {
        match self.0.read() {
            Ok(hash) => hash.get(&lookup)
                .map(|c| c.clone())
                .ok_or(ErrorKind::msg("No Client stored from the given lookup")),
            Err(e) => Err(e.into()),
        }
    }

    fn drop(&mut self, _lookup: MockStorageKey) -> OAuthResult<MockClient> {
        unimplemented!()
    }

    fn has(&self, _lookup: MockStorageKey) -> OAuthResult<bool> {
        unimplemented!()
    }
}
