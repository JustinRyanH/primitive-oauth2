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
- [ ] Revert Oauth2 Client from FutureRes
- [ ] MockClient without state
- [ ] Time Down Mock Server specs to only be various happy cases and one manual error case

Phase 2.1 - Mock Server
----
- [x] Create Mock Auth client to define a significant portion of the error vector
- [X] Remove the generic From Error into two Different FromError methods
- [X] Implement Json Responses (SerDe)

Phase 2.1.1 - Mock Server Response Token Options
----
- [X] Implement Mock Server with Scope
- [X] Implement Mock Server with Token Timeout

Phase 2.1.2 - Mock Client - [Code Grant](https://tools.ietf.org/html/rfc6749#section-4.1)
---
- [ ] 4.1.2 Make Auth Request
- [ ] 4.1.2 Handle Auth Response
  - [ ]  4.1.2.0 Happy
  - [ ]  4.1.2.1 Error
- [ ] 4.1.3 Make Access Token Request
- [ ] 4.1.4 Handle Access Token Response


Phase 2.1.3 - Mock Client - [Implicit Grant](https://tools.ietf.org/html/rfc6749#section-4.2)
---
- [ ] 4.2.2 Make Auth Request
- [ ] 4.2.2 Handle Access Token Response

Phase 2.1.X - Server Auth Response Type (Implicit) [3.1.1.  Response Type](https://tools.ietf.org/html/rfc6749#section-3.1.1)
--- 

Phase 2.2 - The Big Clean Up
----
- [X] Drop Rspec in favor of layers of modules
- [ ] Rename MockStorage into MemoryStory
- [ ] Create NullStorage

Phase 3
----
- [ ] Create Generic Client
- [ ] Use Hyper to implement Generic Client
- [ ] Use Implement a Actix Web with Generic Client
- [ ] Generic Client allows allows to specify extra options
- [ ] Move Mock Client to use Generic Client

Side Tasks
---
- [ ] Drop `error_chain`, it's getting gross
- [ ] Replace all instances of String with `Cow<'a, str>`