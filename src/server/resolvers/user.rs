use crate::core::users;
use crate::core::users::CreateUserAttrs;
use crate::server::schema::user::User;
use crate::server::schema::user::UserInput;
use async_graphql::Error;
use async_graphql::ID;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;

pub async fn user(pool: &Pool, id: Option<ID>) -> Result<Option<User>, Error> {
    // TODO: Validate input parameters
    // TODO: Properly handle errors with `Result`
    let conn = pool.get().await.unwrap();
    // TODO: Remove blanket unwrap for a better solution
    // TODO: Add `UUID` scalar type
    // let id = uuid::Uuid::from_str(&id.unwrap()).unwrap();
    let user = conn
        .interact(|conn| users::get_user(conn, Uuid::now_v7()))
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

pub async fn create_user(pool: &Pool, input: Option<UserInput>) -> Result<Option<User>, Error> {
    // TODO: Validate input parameters
    // TODO: Properly handle errors with `Result`
    let conn = pool.get().await.unwrap();
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
        .interact(|conn| users::create_user(conn, params))
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

#[cfg(test)]
mod tests {
    use crate::{config::get_config, core::repo::connect_database, server::schema::create_schema};
    use async_graphql::{Request, Variables};
    use serde_json::json;

    // TODO: Add a shared test setup, maybe with a database transaction.

    // TODO: Test the resolver, not the schema

    #[tokio::test]
    async fn test_user_success() {
        assert_eq!(true, true)
    }

    #[tokio::test]
    async fn test_user_integration() {
        let config = get_config();
        let pool = connect_database(&config.database_url);
        let schema = create_schema(pool);
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
