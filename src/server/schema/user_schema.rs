use crate::server::resolvers::user_resolver::create_user;
use crate::server::resolvers::user_resolver::user;
use async_graphql::ComplexObject;
use async_graphql::Context;
use async_graphql::Enum;
use async_graphql::InputObject;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::SimpleObject;
use async_graphql::TypeDirective;
use chrono::DateTime;
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;

/// The system role.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Role {
    Admin,
    User,
    #[graphql(name = "SELF")]
    Me,
}

#[TypeDirective(location = "FieldDefinition")]
fn authorize(role: Vec<Role>) {}

#[TypeDirective(location = "InputFieldDefinition")]
fn validate(required: bool) {}

#[derive(Debug, PartialEq, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_address: Option<String>,
    #[graphql(directive = authorize::apply(vec![Role::Admin, Role::Me, Role::User]))]
    pub created_at: Option<DateTime<Utc>>,
    #[graphql(directive = authorize::apply(vec![Role::Admin, Role::Me]))]
    pub updated_at: Option<DateTime<Utc>>,
    #[graphql(directive = authorize::apply(vec![Role::Admin, Role::Me]))]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl User {
    async fn full_name(&self) -> Result<Option<String>> {
        if let (Some(first_name), Some(last_name)) = (&self.first_name, &self.last_name) {
            Ok(Some(format!("{} {}", first_name, last_name)))
        } else {
            Ok(None)
        }
    }
}

#[derive(InputObject)]
pub struct UserInput {
    #[graphql(directive = validate::apply(true))]
    pub first_name: Option<String>,
    #[graphql(directive = validate::apply(true))]
    pub last_name: Option<String>,
    #[graphql(directive = validate::apply(true))]
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
        create_user(ctx.data::<Pool>().unwrap(), input).await
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get a user.
    async fn user(&self, ctx: &Context<'_>, id: Option<Uuid>) -> Result<Option<User>> {
        user(ctx.data::<Pool>().unwrap(), id).await
    }
}
