
# Rust backend Setup


## Todos

- [x] Implement unit test for the implementations of the Permission trait
- [x] Add admin, who can do everything as root, except for managing admins, or root.
- [x] Add handler to create user (root can create admins only, admin can create normal users only)
- [ ] Add handler to delete user (root can delete anyone, except himself, admin can delete normal users only)
- [ ] Update add_member permissions so that root can add anyone, admin can only add himself or other non admin users
- [ ] Add proper logging/tracing with OpenTelemetry prior to returning a response to the client