use futures::Future;
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use client::ClientStorage;
use client::mock_client::MockClient;
use client::storage::{MockMemoryStorage, MockStorageKey};

#[test]
fn mock_client() {
    #[derive(Debug, Clone)]
    struct Subject {
        pub pool: CpuPool,
    }

    impl Default for Subject {
        fn default() -> Subject {
            Subject {
                pool: CpuPool::new(4),
            }
        }
    }

    rspec::run(&given("MockStoage", Subject::default(), |ctx| {
        ctx.context("#set", |ctx| {
            ctx.it("adds the cleint to storage", move |subject| {
                let mut storage = MockMemoryStorage::new();
                assert_that(&*storage.read().unwrap()).is_empty();
                subject
                    .pool
                    .spawn(storage.set(MockStorageKey::state("foo"), MockClient::new().unwrap()))
                    .wait()
                    .unwrap();
                assert_that(&*storage.read().unwrap()).contains_key(MockStorageKey::state("foo"));
            });
        });

        ctx.context("#get", |ctx| {
            ctx.it("returns a client", move |subject| {
                let subject_client = MockClient::new().unwrap();
                let mut storage = MockMemoryStorage::new();
                assert_that(&*storage.read().unwrap()).is_empty();
                let put = subject
                    .pool
                    .spawn(storage.set(MockStorageKey::state("foo"), subject_client.clone()))
                    .wait();
                assert_that(&put).is_ok();
                let get = subject
                    .pool
                    .spawn(storage.get(MockStorageKey::state("foo")))
                    .wait();
                assert_that(&get).is_ok().is_equal_to(&subject_client);
            });
        });
    }));
}
