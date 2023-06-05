# UNKSO Colossus

TODO: This README

## Crates

- api
  - HTTP layer
  - REST endpoints
  - GQL endpoints?
  - HTTP types (only send data that needs to go to the client, etc)
  - Data services (wraps repositories and returns data, etc)
    - faux mocking of Services
- domain
  - Database layer
  - sqlx for querying
    - 1:N relationships baked into models
    - M:N relationships defined by pivot tables
  - Repository types
    - Basic CRUD operations
    - faux for mocking in unit tests
- libservices
  - Types for Services (traits, errors, etc)
- libservices_codegen
  - Derive for Service traits to attach to Rocket fairings
