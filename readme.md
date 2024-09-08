
# Rust Backend Setup
Rust Backend Setup, aims to provide a starting point for developing backend applications in Rust. It does so by providing implementations for common needed functionality, that can be easily modified based on specific needs. All functionality provided is implemented from a Domain Driven approach. 

**Key highlights:**
- Custom PASETO-based Authentication with Sessions, Access & Refresh tokens
  - PASETO tokens are used for secure access and refresh tokens. Access tokens are short-lived, while refresh tokens allow seamless token renewal without re-authentication.
  - Sessions provide a way to handle authenticated users. Different states of a Session are fully typesafe, making it near impossible to model illegal states of a [Session](./crates/domain/src/sessions/user_session.rs).    
- Custom Attributed-Based-Access-Control
  - [Policies](app/src/policy/policy.rs) provide a easy way to determine whenever a user is authorized to either perform an action or access a specific resource. Upon authorization a policy returns a contract allowing the caller to perform only the action for which it was authorized by the Policy.
  - Policies can easily be accessed with the use of a [custom Axum Extractor](app/src/extractors/user/user_with_policy.rs). For reference, the [get_user_details](app/src/handlers/v1/users/get_user_details.rs) handler, uses the [ReadUserDetailsPolicy](app/src/policy/policies/read_user_details_policy.rs) to ensure any returned the details of a user have been accessed in an authorized manner. 
- Decoupled & Isolated Domain Entities
  - Domain Entities and Properties are isolated within its own crate, thereby strongly insinuating that Domain Entities and Properties, should be solely focused on solving business requirements.
  - The Domain Entities are not concerned with how any of their properties are stored within the database, thereby allowing both Domain Entities and Database Schemas to define data structures that best address their own specific concerns in the most efficient way possible.
- Tracing & OpenTelemetry integration (WIP)
- Integration testing with custom Client
  - [TestApp](app/tests/util/test_app.rs) and [TestUser](app/tests/util/test_user/test_user.rs) allows for easy setup of the application in order to test specific scenarios.
  - Global Test Coverage currently stands at 78%, with Domain coverage standing at 90%.

# Setup
## For local development
```sh
# Checkout project
git clone git@github.com:quintvanzeumeren/rust-backend-setup.git
cd rust-backend-setup

# Use docker to start postgres database in the background
docker compose up -d
 
# Migrate database:
sh scripts/init_db.sh

# Build dependencies
# NOTE: SQLx requires a live connection to a database containing the schema in 
# order to check the queries on compile time.
cargo build

# Run Application
cargo run
```

# Contributing
Any kind of feedback or contribution is appreciated. Don’t hesitate to raise issues or submit pull requests for new features, bug fixes, or questions you might have.

# To-Do

## Open Telemetry 
  - [ ] Sync trace ids between handlers and axum extractors. Possible solution is generate a trace id within a middleware prior to it hitting request handlers, and extractors   
  - [ ] Prior to returning any error responses to the client, we should log/trace the error to OpenTelemetry




