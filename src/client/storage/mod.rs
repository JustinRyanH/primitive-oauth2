// #[cfg(test)]
// mod spec;

use std::sync::{Arc, RwLock};
use std::{borrow::Cow, collections::HashMap, ops::Deref};

use client::mock_client::MockClient;
use client::ClientStorage;
use errors::{ErrorKind, OAuthResult};

#[derive(Debug, Clone)]
pub struct MockMemoryStorage<'a>(pub Arc<RwLock<HashMap<Cow<'a, str>, MockClient>>>);

impl<'a> MockMemoryStorage<'a> {
    pub fn new() -> MockMemoryStorage<'a> {
        MockMemoryStorage(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn len(&self) -> OAuthResult<usize> {
        match self.0.read() {
            Ok(v) => Ok(v.len()),
            Err(e) => Err(e.into()),
        }
    }
}

impl<'a> Deref for MockMemoryStorage<'a> {
    type Target = RwLock<HashMap<Cow<'a, str>, MockClient>>;

    fn deref(&self) -> &RwLock<HashMap<Cow<'a, str>, MockClient>> {
        &self.0
    }
}

impl<'a> ClientStorage<'a, MockClient> for MockMemoryStorage<'a> {
    type Error = ErrorKind;

    fn set<K>(&mut self, lookup: K, value: MockClient) -> OAuthResult<Option<MockClient>>
    where
        K: Into<Cow<'a, str>>,
    {
        match self.0.write() {
            Ok(ref mut hash) => Ok(hash.insert(lookup.into(), value)),
            Err(e) => Err(e.into()),
        }
    }

    fn get<K>(&self, lookup: K) -> OAuthResult<MockClient>
    where
        K: Into<Cow<'a, str>>,
    {
        match self.0.read() {
            Ok(hash) => hash.get(&lookup.into())
                .map(|c| c.clone())
                .ok_or(ErrorKind::msg("No Client stored from the given lookup")),
            Err(e) => Err(e.into()),
        }
    }

    fn drop<K>(&mut self, _lookup: K) -> OAuthResult<MockClient>
    where
        K: Into<Cow<'a, str>>,
    {
        unimplemented!()
    }

    fn has<K>(&self, _lookup: K) -> OAuthResult<bool>
    where
        K: Into<Cow<'a, str>>,
    {
        unimplemented!()
    }
}
