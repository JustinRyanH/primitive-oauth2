#[allow(unused_imports)]
use futures::Future;
use futures_cpupool::CpuPool;
use spectral::prelude::*;

#[allow(unused_imports)]
use client::ClientStorage;
use client::mock_client::MockClient;
use client::storage::{MockMemoryStorage, MockStorageKey};

mod given_a_memory_storage {
    use super::*;

    type Subject = MockMemoryStorage;

    #[derive(Debug, Clone)]
    struct Env {
        pub pool: CpuPool,
    }

    fn env() -> Env {
        Env {
            pool: CpuPool::new(1),
        }
    }

    mod when_store_is_empty {
        use super::*;
        fn subject() -> Subject {
            let storage = MockMemoryStorage::new();
            assert_that(&*storage.read().unwrap()).is_empty();
            storage
        }

        #[test]
        fn client_storage_set() {
            let mut subject = subject();
            let client = MockClient::new().unwrap();
            let action = subject.set(MockStorageKey::state("foo"), client);

            env().pool.spawn(action).wait().unwrap();
            assert_that(&*subject.read().unwrap()).contains_key(MockStorageKey::state("foo"));
        }
    }

    mod when_storage_has_data {
        use super::*;
        use std::ops::Deref;

        fn subject() -> Subject {
            let storage = MockMemoryStorage::new();
            storage.deref().write().unwrap().insert(
                MockStorageKey::state("EXAMPLE_STATE"),
                MockClient::new().unwrap(),
            );
            assert_that(&*storage.read().unwrap())
                .contains_key(MockStorageKey::state("EXAMPLE_STATE"));
            storage
        }

        #[test]
        fn client_storage_get() {
            let subject = subject();
            let action = subject.get(MockStorageKey::state("EXAMPLE_STATE"));

            assert_that(&env().pool.spawn(action).wait()).is_ok();
        }
    }

}
