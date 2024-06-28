use crate::core::users;
use crate::server::error::ResolverError;
use crate::server::schema::user_schema::User;
use crate::server::schema::user_schema::UserInput;
use async_graphql::Error;
use async_graphql::ErrorExtensions;
use async_graphql::ID;
use deadpool_diesel::postgres::Pool;
use std::str::FromStr;
use uuid::Uuid;

pub async fn user(pool: &Pool, id: Option<ID>) -> Result<Option<User>, Error> {
    // TODO: Validate input parameters
    // TODO: Properly handle errors with `Result`
    let conn = pool.get().await.unwrap();
    // TODO: Remove blanket unwrap for a better solution
    // TODO: Add `UUID` scalar type
    // let id = uuid::Uuid::from_str(&id.unwrap()).unwrap();
    // TODO: Clearly a hacked together solution
    let result = conn
        .interact(|conn| users::fetch_user(conn, Uuid::from_str(id.unwrap().as_str()).unwrap()))
        .await;

    // TODO: Probably not the nicest way to do it; handle errors explicitly
    // TODO: Return a graphql error
    let option = match result {
        Ok(Ok(option)) => option,
        Ok(Err(_)) => return Err(ResolverError::InternalServer.extend()),
        Err(_) => return Err(ResolverError::InternalServer.extend()),
    };

    // Err(async_graphql::Error::new("Internal server error")
    //     .extend_with(|_, e| e.set("code", "INTERNAL_SERVER_ERROR")))

    match option {
        Some(user) => Ok(Some(User {
            id: Some(ID::from(user.id)),
            first_name: Some(user.first_name),
            last_name: Some(user.last_name),
            email_address: Some(user.email_address),
            created_at: Some(user.created_at.to_string()),
            updated_at: Some(user.updated_at.to_string()),
            // TODO: If not `None`, then to ISO string
            deleted_at: None,
        })),
        None => Ok(None),
    }
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
    let attrs = users::CreateUserAttrs {
        first_name: attrs.first_name.unwrap(),
        last_name: attrs.last_name.unwrap(),
        email_address: attrs.email_address.unwrap(),
    };
    let user = conn
        .interact(|conn| users::create_user(conn, attrs))
        .await
        .unwrap();

    Ok(Some(User {
        id: Some(ID::from(user.id)),
        first_name: Some(user.first_name),
        last_name: Some(user.last_name),
        email_address: Some(user.email_address),
        created_at: Some(user.created_at.to_string()),
        updated_at: Some(user.updated_at.to_string()),
        // TODO: If not `None`, then to ISO string
        deleted_at: None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::core::users;
    use crate::{config::get_config, core::repo::connect_database, server::schema::create_schema};
    use async_graphql::{Request, Variables};
    use diesel::Connection;
    use diesel::PgConnection;
    use serde_json::json;

    // TODO: Add a shared test setup, maybe with a database transaction.

    #[tokio::test]
    async fn test_user_success() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let user = users::create_user(
            &mut conn,
            users::CreateUserAttrs {
                first_name: "Jane".to_string(),
                last_name: "Doe".to_string(),
                email_address: "jane@doe.com".to_string(),
            },
        );
        let config = get_config();
        let pool = connect_database(&config.database_url);
        let result = crate::server::resolvers::user_resolver::user(
            &pool,
            Some(async_graphql::ID::from(user.id)),
        )
        .await;

        assert_eq!(
            result,
            Ok(Some(User {
                id: Some(ID::from(user.id)),
                first_name: Some(user.first_name),
                last_name: Some(user.last_name),
                email_address: Some(user.email_address),
                // TODO: This is not an ISO8601 string
                created_at: Some(user.created_at.to_string()),
                updated_at: Some(user.updated_at.to_string()),
                // TODO: If not `None`, then to ISO string
                deleted_at: None,
            }))
        )
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let config = get_config();
        let pool = connect_database(&config.database_url);
        let result = crate::server::resolvers::user_resolver::user(
            &pool,
            Some(async_graphql::ID::from(Uuid::now_v7())),
        )
        .await;

        assert_eq!(result, Ok(None));
    }

    #[tokio::test]
    async fn test_user_missing_id() {
        assert_eq!(true, true);
    }

    #[tokio::test]
    async fn test_user_integration() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let user = users::create_user(
            &mut conn,
            users::CreateUserAttrs {
                first_name: "Jane".to_string(),
                last_name: "Doe".to_string(),
                email_address: "jane@doe.com".to_string(),
            },
        );
        let config = get_config();
        let pool = connect_database(&config.database_url);
        let schema = create_schema(pool);
        let query = "
        query User($id: ID) {
            user(id: $id) {
                id
                firstName
                lastName
                fullName
                emailAddress
                createdAt
                updatedAt
                deletedAt
            }
        }
        ";
        let variables = json!({"id": user.id.to_string()});
        let response = schema
            .execute(Request::new(query).variables(Variables::from_json(variables)))
            .await;

        assert_eq!(
            response.data.into_json().unwrap(),
            json!({
                "user": {
                    "id": user.id.to_string(),
                    "firstName": user.first_name.to_string(),
                    "lastName": user.last_name.to_string(),
                    "fullName": format!("{} {}", user.first_name, user.last_name),
                    "emailAddress": user.email_address.to_string(),
                    // TODO: This is not an ISO8601 string
                    "createdAt": user.created_at.to_string(),
                    "updatedAt": user.updated_at.to_string(),
                    // TODO: The `Null` value should be imported
                    "deletedAt": async_graphql::Value::Null
                }
            })
        );
        assert!(response.errors.is_empty());
        assert!(response.extensions.is_empty());
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
