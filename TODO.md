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
- [/] Create an Ext Oauth2 Client that is Future, and 
  one with regular result
- [ ] MockClient without state
- [X] MockServer can create each of the potential error responses for Redirect
- [X] MockServer can return a token

Phase 2.1 - Mock Server
----
- [x] Create Mock Auth client to define a significant portion of the error vector
- [ ] Remove the generic From Error into two Different FromError methods 
- [ ] Implement Json Responses (SerDe)

Phase 2.2 - The Big Clean Up
----
- [X] Drop Rspec in favor of layers of modules
- [ ] Streamline the Authenticator
- [ ] Streamline the Authenticator Specs
- [ ] Streamline Mock Client
- [ ] Streamline Mock Client Spec
- [ ] Rename MockStorage into MemoryStory
- [ ] Create NullStorage

Phase 3
----
- [ ] Create Generic Client
- [ ] Use Hyper to implement Generic Client
- [ ] Use Implement a Actix Web with Generic Client
- [ ] Generic Client allows allows to specify extra options
- [ ] Move Mock Client to use Generic Client
