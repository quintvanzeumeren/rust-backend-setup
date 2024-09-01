
# Rust backend Setup


## Todos

### Handlers
- [x] Implement unit test for the implementations of the Permission trait
- [x] Add admin, who can do everything as root, except for managing admins, or root.
- [x] Add handler to create user (root can create admins only, admin can create normal users only)
- [ ] Add handler to delete user
  - [ ] root can delete anyone, except himself
  - [ ] admin can delete normal users only
- [ ] Refactor add_member permissions: 
  - [ ] root can add anyone, 
  - [ ] admin can only add himself or other non admin users

### Open Telemetry 
  - [ ] Sync trace ids between handlers and axum extractors. Possible solution is generate a trace id within a middleware prior to it hitting request handlers, and extractors   
  - [ ] Prior to returning any error responses to the client, we should log/trace the error to OpenTelemetry

### Simplify Permission, Policy, and Contract structs
- [ ] Remove to `Permission` trait by implementing the permissions logic into each corresponding `Policy` trait.
- [ ] Refactor `Policy` trait so that the `authorize` method is async. So that it can query any information it needs to determine if it can authorize the action

### Readme

- [ ] Introduction project
- [ ] Installation
- [ ] Design


