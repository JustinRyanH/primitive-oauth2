use futures::Future;
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use std::collections::HashMap;

use client::ClientStorage;
use client::mock_client::MockClient;
use client::storage::{MockMemoryStorage, MockStorageKey};

#[test]
fn mock_client() {
    let pool = CpuPool::new(4);

    rspec::run(&given("MockStoage", MockMemoryStorage::new(), |ctx| {
        ctx.when("Adding Entries to MockStorage", |ctx| {
            ctx.it("adds the cleint to storage", move |env| {
                let mut storage = env.clone();
                assert_that(&*storage).is_empty();
                pool.spawn(storage.set(MockStorageKey::state("foo"), MockClient::new().unwrap()))
                    .wait()
                    .unwrap();
                assert_that(&*storage).contains_key(MockStorageKey::state("foo"));
            });
        });
    }));
}
