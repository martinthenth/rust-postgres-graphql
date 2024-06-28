use crate::server::resolvers;
use async_graphql::Context;
use async_graphql::InputObject;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::ID;
use deadpool_diesel::postgres::Pool;

// TODO: Combine `SimpleObject` with `complex()`
pub struct User {
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
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
pub struct UserInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_address: Option<String>,
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
        resolvers::user::create_user(ctx.data::<Pool>().unwrap(), input).await
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get a user.
    async fn user(&self, ctx: &Context<'_>, id: Option<ID>) -> Result<Option<User>> {
        resolvers::user::user(ctx.data::<Pool>().unwrap(), id).await
    }
}
