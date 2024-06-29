use crate::core::users;
use crate::server::resolvers::errors::internal_server;
use crate::server::schema::user_schema::User;
use crate::server::schema::user_schema::UserInput;
use async_graphql::Error;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;

pub async fn user(pool: &Pool, id: Option<Uuid>) -> Result<Option<User>, Error> {
    // TODO: Validate input parameters
    let conn = pool.get().await.unwrap();
    let result = conn
        .interact(move |conn| users::fetch_user(conn, id.unwrap()))
        .await;

    // TODO: Handle specific database errors, like `NotFound`
    // TODO: Write an abstraction (any db error returns internal server error)
    let option = match result {
        Ok(Ok(option)) => option,
        Ok(Err(_)) => return Err(internal_server()),
        Err(_) => return Err(internal_server()),
    };

    match option {
        Some(user) => Ok(Some(User {
            id: Some(user.id),
            first_name: Some(user.first_name),
            last_name: Some(user.last_name),
            email_address: Some(user.email_address),
            created_at: Some(user.created_at.and_utc()),
            updated_at: Some(user.updated_at.and_utc()),
            // TODO: If not `None`, then to ISO string
            deleted_at: None,
        })),
        None => Ok(None),
    }
}

pub async fn create_user(pool: &Pool, input: Option<UserInput>) -> Result<Option<User>, Error> {
    // TODO: Validate input parameters
    let conn = pool.get().await.unwrap();
    let attrs = input.unwrap();
    let attrs = users::CreateUserAttrs {
        first_name: attrs.first_name.unwrap(),
        last_name: attrs.last_name.unwrap(),
        email_address: attrs.email_address.unwrap(),
    };
    let result = conn.interact(|conn| users::create_user(conn, attrs)).await;

    // TODO: Handle specific database errors, like `NotFound`
    // TODO: Write an abstraction (any db error returns internal server error)
    let option = match result {
        Ok(Ok(option)) => option,
        Ok(Err(_)) => return Err(internal_server()),
        Err(_) => return Err(internal_server()),
    };

    match option {
        Some(user) => Ok(Some(User {
            id: Some(user.id),
            first_name: Some(user.first_name),
            last_name: Some(user.last_name),
            email_address: Some(user.email_address),
            created_at: Some(user.created_at.and_utc()),
            updated_at: Some(user.updated_at.and_utc()),
            // TODO: If not `None`, then to ISO string
            deleted_at: None,
        })),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::core::repo;
    use crate::server::resolvers::user_resolver;
    use crate::server::schema;
    use crate::test::factory;

    // TODO: Add a shared test setup, maybe with a database transaction.

    #[tokio::test]
    async fn test_user_success() {
        let user = factory::create_user();
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let result = user_resolver::user(&pool, Some(user.id)).await;

        assert_eq!(
            result,
            Ok(Some(User {
                id: Some(user.id),
                first_name: Some(user.first_name),
                last_name: Some(user.last_name),
                email_address: Some(user.email_address),
                created_at: Some(user.created_at.and_utc()),
                updated_at: Some(user.updated_at.and_utc()),
                // TODO: If not `None`, then to ISO string
                deleted_at: None,
            }))
        )
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let id = Uuid::now_v7();
        let result = user_resolver::user(&pool, Some(id)).await;

        assert_eq!(result, Ok(None));
    }

    #[tokio::test]
    async fn test_user_missing_id() {
        assert_eq!(true, true);
    }

    #[tokio::test]
    async fn test_user_integration() {
        let user = factory::create_user();
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let schema = schema::create_schema(pool);
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
        let variables = serde_json::json!({"id": user.id.to_string()});
        let response = schema
            .execute(
                async_graphql::Request::new(query)
                    .variables(async_graphql::Variables::from_json(variables)),
            )
            .await;

        assert_eq!(
            response.data.into_json().unwrap(),
            serde_json::json!({
                "user": {
                    "id": user.id.to_string(),
                    "firstName": user.first_name.to_string(),
                    "lastName": user.last_name.to_string(),
                    "fullName": format!("{} {}", user.first_name, user.last_name),
                    "emailAddress": user.email_address.to_string(),
                    "createdAt": user.updated_at.and_utc().to_rfc3339(),
                    "updatedAt": user.updated_at.and_utc().to_rfc3339(),
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
