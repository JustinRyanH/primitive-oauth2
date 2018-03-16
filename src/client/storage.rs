use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::ops::Deref;

use futures::future::{result, FutureResult};

use errors::{Error, Result};
use client::ClientStorage;
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

    fn set(
        &mut self,
        lookup: MockStorageKey,
        value: MockClient,
    ) -> FutureResult<Option<MockClient>, Error> {
        match self.0.write() {
            Ok(ref mut hash) => result(Ok(hash.insert(lookup, value))),
            Err(e) => result(Err(e.into())),
        }
    }

    fn get(&self, _lookup: MockStorageKey) -> FutureResult<MockClient, Self::Error> {
        unimplemented!()
    }
    fn drop(&mut self, _lookup: MockStorageKey) -> FutureResult<MockClient, Self::Error> {
        unimplemented!()
    }

    fn has(&self, _lookup: MockStorageKey) -> FutureResult<bool, Self::Error> {
        unimplemented!()
    }
}
