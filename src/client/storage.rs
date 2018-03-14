use std::collections::HashMap;
use std::ops::Deref;

use futures::future::{result, FutureResult};

use errors::Error;
use client::ClientStorage;
use client::mock_client::MockClient;

#[derive(Debug, Clone)]
pub struct MockMemoryStorage(pub HashMap<MockStorageKey, MockClient>);

impl MockMemoryStorage {
    pub fn new() -> MockMemoryStorage {
        MockMemoryStorage(HashMap::new())
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
    type Target = HashMap<MockStorageKey, MockClient>;

    fn deref(&self) -> &HashMap<MockStorageKey, MockClient> {
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
    ) -> FutureResult<MockClient, Error> {
        result(MockClient::new())
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
