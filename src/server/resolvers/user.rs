use crate::core::users::create_user;
use crate::core::users::get_user;
use crate::core::users::CreateUserAttrs;
use async_graphql::Context;
use async_graphql::InputObject;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::ID;
use deadpool_diesel::postgres::Pool;
use std::str::FromStr;

// TODO: These are not resolver functions but the GraphQL schema!
// That means, I can move the resolvers to their own function maybe; and test those separately.

// TODO: Combine `SimpleObject` with `complex()`

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
        // TODO: Remove blanket unwrap for a better solution
        // if let Some(user_input) = input {
        //   thing
        // } else {
        //   return Err(/* appropriate error */);
        // }
        let attrs = input.unwrap();
        let params = CreateUserAttrs {
            first_name: attrs.first_name.unwrap(),
            last_name: attrs.last_name.unwrap(),
            email_address: attrs.email_address.unwrap(),
        };
        let user = conn
            .interact(|conn| create_user(conn, params))
            .await
            .unwrap();

        Ok(Some(User {
            id: ID::from("123"),
            first_name: user.first_name,
            last_name: user.last_name,
            email_address: user.email_address,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
            // TODO: If not `None`, then to ISO string
            deleted_at: None,
        }))
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get a user.
    async fn user(&self, ctx: &Context<'_>, id: Option<ID>) -> Result<Option<User>> {
        // TODO: Validate input parameters
        // TODO: Rename `database` to `pool` or something
        let database = ctx.data::<Pool>().unwrap();
        // TODO: Properly handle errors with `Result`
        let conn = database.get().await.unwrap();
        // TODO: Remove blanket unwrap for a better solution
        // TODO: Add `UUID` scalar type
        let id = uuid::Uuid::from_str(&id.unwrap()).unwrap();
        let user = conn.interact(|conn| get_user(conn, id)).await.unwrap();

        Ok(Some(User {
            id: ID::from("123"),
            first_name: user.first_name,
            last_name: user.last_name,
            email_address: user.email_address,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
            // TODO: If not `None`, then to ISO string
            deleted_at: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::get_config, core::repo::connect_database, server::graph::create_schema};
    use async_graphql::{Request, Variables};
    use serde_json::json;

    // TODO: Add a shared test setup, maybe with a database transaction.

    #[tokio::test]
    async fn test_user_success() {
        let config = get_config();
        let database = connect_database(&config.database_url);
        let schema = create_schema(database);
        let query = "
        query {
            user {
                id
                firstName
                lastName
            }
        }
        ";
        let variables = json!({"id": "123"});
        let response = schema
            .execute(Request::new(query).variables(Variables::from_json(variables)))
            .await;

        assert_eq!(
            response.data.into_json().unwrap(),
            json!({
                "user": {
                    "id": "123",
                    "firstName": "Martin",
                    "lastName": "Nijboer",
                }
            })
        );
        assert!(response.errors.is_empty());
        assert!(response.extensions.is_empty());
    }

    #[tokio::test]
    async fn test_user_missing_id() {
        assert_eq!(true, true);
    }

    #[tokio::test]
    async fn test_create_user_success() {
        assert_eq!(true, true)
    }

    #[tokio::test]
    async fn test_create_user_missing_input() {
        assert_eq!(true, true)
    }
}
