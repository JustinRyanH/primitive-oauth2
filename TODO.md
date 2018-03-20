Phase 1
----
- [x] Seperate Authenticator's specs into their own file
- [x] rename `*_test.rs` into `*_spec.rs`
- [x] Refactor authenticator's tests to use Spectural
- [x] MockStorage Get Spec
- [x] MockStorage Set Spec
- [x] MockStorage Delete Spec

Phase 2
----
- [ ] MockClient without state
- [ ] MockServer can create each of the potential error responses for Redirect
- [ ] MockServer can return a token
- [ ] MockServer can return each of the Error Types for 
- [ ] The big cleanup (Document)

The Big Clean Up
----
- [ ] Drop Rspec in favor of layers of modules
- [ ] Streamline the Authenticator
- [ ] Streamline the Authenticator Specs
- [ ] Streamline Mock Client
- [ ] Streamline Mock Client Spec
- [ ] Rename MockStorage into MemoryStory
- [ ] Create NullStorage
- [ ] Move MockClient into Feature

Phase 3
----
- [ ] Create Generic Client
- [ ] Use Hyper to implement Generic Client
- [ ] Use Implement a Actix Web with Generic Client
- [ ] Generic Client allows allows to specify extra options
- [ ] Move Mock Client to use Generic Client