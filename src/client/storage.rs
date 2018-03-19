use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::ops::Deref;

use futures::future::{err as FutErr, ok as FutOk};
use futures::IntoFuture;

use errors::{Error, Result};
use client::{AsyncPacker, ClientStorage, FutResult};
use client::mock_client::MockClient;

#[derive(Debug, Clone)]
pub struct MockMemoryStorage(pub Arc<RwLock<HashMap<MockStorageKey, MockClient>>>);

impl MockMemoryStorage {
    pub fn new() -> MockMemoryStorage {
        MockMemoryStorage(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn len(&self) -> Result<usize> {
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
    type Error = Error;
    type Lookup = MockStorageKey;

    fn set(&mut self, lookup: MockStorageKey, value: MockClient) -> FutResult<Option<MockClient>> {
        match self.0.write() {
            Ok(ref mut hash) => FutOk(hash.insert(lookup, value)),
            Err(e) => FutErr(e.into()),
        }.pack()
    }

    fn get(&self, lookup: MockStorageKey) -> FutResult<MockClient> {
        match self.0.read() {
            Ok(hash) => hash.get(&lookup)
                .map(|c| c.clone())
                .ok_or(Error::msg("No Client stored from the given lookup"))
                .into_future(),
            Err(e) => FutErr(e.into()),
        }.pack()
    }
    fn drop(&mut self, _lookup: MockStorageKey) -> FutResult<MockClient> {
        unimplemented!()
    }

    fn has(&self, _lookup: MockStorageKey) -> FutResult<bool> {
        unimplemented!()
    }
}
