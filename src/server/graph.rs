use crate::core::users::create_user;
use async_graphql::Context;
use async_graphql::EmptySubscription;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::Schema;
use deadpool_diesel::postgres::Pool;

pub type GraphSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Token(pub String);

pub struct Query;

pub fn create_schema(database: Pool) -> GraphSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(database)
        .finish()
}

#[Object]
impl Query {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
    async fn howdy(&self) -> &'static str {
        "partner"
    }
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    /// Get a user.
    async fn user(&self, _ctx: &Context<'_>) -> Result<Option<User>> {
        let user = User {
            id: async_graphql::ID::from("123"),
            first_name: String::from("Martin"),
            last_name: String::from("Nijboer"),
        };

        Ok(Some(user))
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a user.
    async fn create_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        // TODO: Rename `database` to `pool` or something
        let database = ctx.data::<Pool>().unwrap();
        // TODO: Properly handle errors with `Result`
        let conn = database.get().await.unwrap();
        let user = conn
            .interact(|conn| create_user(conn, "Martin".to_string(), "Nijboer".to_string()))
            .await
            .unwrap();
        let user = User {
            id: async_graphql::ID::from("123"),
            first_name: user.first_name,
            last_name: user.last_name,
        };

        Ok(Some(user))
    }
}

pub struct User {
    id: async_graphql::ID,
    first_name: String,
    last_name: String,
}

#[Object]
impl User {
    async fn id(&self) -> Result<Option<&String>> {
        Ok(Some(&self.id))
    }
    async fn first_name(&self) -> Result<Option<&String>> {
        Ok(Some(&self.first_name))
    }
    async fn last_name(&self) -> Result<Option<&String>> {
        Ok(Some(&self.last_name))
    }
}
