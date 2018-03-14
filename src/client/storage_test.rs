use futures::Future;
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use client::ClientStorage;
use client::mock_client::MockClient;
use client::storage::{MockMemoryStorage, MockStorageKey};

#[test]
fn mock_client() {
    let pool = CpuPool::new(4);

    rspec::run(&given("MockStoage", 0, |ctx| {
        ctx.when("Adding Entries to MockStorage", |ctx| {
            ctx.it("adds the cleint to storage", move |_| {
                let mut storage = MockMemoryStorage::new();
                assert_that(&*storage.read().unwrap()).is_empty();
                pool.spawn(storage.set(MockStorageKey::state("foo"), MockClient::new().unwrap()))
                    .wait()
                    .unwrap();
                assert_that(&*storage.read().unwrap()).contains_key(MockStorageKey::state("foo"));
            });
        });
    }));
}
