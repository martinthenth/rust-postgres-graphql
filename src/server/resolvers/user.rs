use crate::core::users::create_user;
use async_graphql::Context;
use async_graphql::InputObject;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::ID;
use deadpool_diesel::postgres::Pool;

struct User {
    id: ID,
    first_name: String,
    last_name: String,
    email_address: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
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
    async fn email_address(&self) -> Result<Option<&String>> {
        Ok(Some(&self.email_address))
    }
    async fn created_at(&self) -> Result<Option<&String>> {
        Ok(Some(&self.created_at))
    }
    async fn updated_at(&self) -> Result<Option<&String>> {
        Ok(Some(&self.updated_at))
    }
    async fn deleted_at(&self) -> Result<&Option<String>> {
        Ok(&self.deleted_at)
    }
}

#[derive(InputObject)]
struct UserInput {
    first_name: Option<String>,
    last_name: Option<String>,
    email_address: Option<String>,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Create a user.
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: Option<UserInput>,
    ) -> Result<Option<User>> {
        // TODO: Validate input parameters
        // TODO: Rename `database` to `pool` or something
        let database = ctx.data::<Pool>().unwrap();
        // TODO: Properly handle errors with `Result`
        let conn = database.get().await.unwrap();
        // TODO: Get params from graphql
        let user = conn
            .interact(|conn| {
                create_user(
                    conn,
                    "Martin".to_string(),
                    "Nijboer".to_string(),
                    "martin@example.com".to_string(),
                )
            })
            .await
            .unwrap();
        let user = User {
            id: ID::from("123"),
            first_name: user.first_name,
            last_name: user.last_name,
            email_address: user.email_address,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
            // TODO: If not `None`, then to ISO string
            deleted_at: None,
        };

        Ok(Some(user))
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get a user.
    async fn user(&self, _ctx: &Context<'_>, id: Option<ID>) -> Result<Option<User>> {
        let user = User {
            id: async_graphql::ID::from("123"),
            first_name: String::from("Martin"),
            last_name: String::from("Nijboer"),
            email_address: String::from("martin@example.com"),
            created_at: String::from("time"),
            updated_at: String::from("time"),
            deleted_at: None,
        };

        Ok(Some(user))
    }
}
