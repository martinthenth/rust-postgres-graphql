# rust-postgres-graphql-example

TODO: Write a nice `README.md` that explains the technical choices

- can be used as a microservice or monolithic backend api
- has separation of concerns (business-logic vs web-logic)
- is an example for how to implement a web app in Rust

## Why Rust?

- type safety
- easy to maintain
- syntax looks better than Go (looking at you, `if err != nil {`)
- performance (nice to have, but not required for web apps)

## Why Postgres?

- stable technology
- open source
- relational database model
- can be used for almost everything
- excellent performance
- well understood solutions for sharding and extending

## Why GraphQL?

- frontend-backend contract
- api discoverability
- easy to set up
- well understood technology (advantages and inherent problems with solutions)

### Nullability of fields

- recommend "nullable everything" for backward compatibility with clients
- easy to write and easy to maintain
- best practice for frontends, in case some fields return an erro
- easy to handle in typescript clients
- error propagation to parent fields blocks features in frontends

## Engineering decisions

- when reading posts, there are different choices (like merging business and web logic)

### Separating business-logic and web-logic

- can add new apis later, like an `rpc` api
- keeps web logic out of the business logic (web logic calls business logic)

### Separating GraphQL schema and resolvers

- smaller modules
- easier to test (for example to pass a database connection instead of getting the pool from the graphql resolver context)

### Testing resolvers instead of GraphQL queries

- tests are smaller
- it tests the smallest unit of code, the resolver (which is all you care about)
- testing graphql queries are integraion tests (it tests the graphql schema)

### Testing with a database

- database transaction to keep the database clean
- test domain logic against the database ("create user" should create a user and not "mock creating a user")
- is fast, almost as fast as mocks (database calls take ~1ms)
