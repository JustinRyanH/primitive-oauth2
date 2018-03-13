use std::collections::HashMap;
use std::ops::Deref;

use futures::future::FutureResult;

use errors::Error;
use client::ClientStorage;
use client::mock_client::MockClient;

#[derive(Debug, Clone)]
pub struct MockMemoryStorage(HashMap<MockStorageKey, MockClient>);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum MockStorageKey {
    State(String),
    Uid(usize),
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
        _lookup: MockStorageKey,
        _value: MockClient,
    ) -> FutureResult<MockClient, Error> {
        unimplemented!()
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
