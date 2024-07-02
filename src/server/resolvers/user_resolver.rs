use crate::core::users;
use crate::server::resolvers::errors::internal_server;
use crate::server::schema::user_schema::User;
use crate::server::schema::user_schema::UserInput;
use async_graphql::Error;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;

// struct UserParams {
//     id: Uuid,
// }

// struct CreateUserParams {
//     input: UserInput,
// }

pub async fn user(pool: &Pool, id: Option<Uuid>) -> Result<Option<User>, Error> {
    // TODO: Validate input parameters
    // TODO: Set directives for input objects
    let conn = pool.get().await.unwrap();
    let result = conn
        .interact(move |conn| users::get_user(conn, id.unwrap()))
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
            deleted_at: user.deleted_at.map(|datetime| datetime.and_utc()),
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
            deleted_at: user.deleted_at.map(|datetime| datetime.and_utc()),
        })),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::core::repo;
    use crate::server::resolvers::errors::unprocessable_content;
    use crate::server::resolvers::user_resolver;
    use crate::server::schema;
    use crate::test::factory;
    use async_graphql::Value::Null;

    // TODO: Add a shared test setup, maybe with a database transaction.

    #[tokio::test]
    async fn test_user() {
        let user = factory::insert_user();
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let result = user_resolver::user(&pool, Some(user.id)).await.unwrap();

        assert_eq!(
            result,
            Some(User {
                id: Some(user.id),
                first_name: Some(user.first_name),
                last_name: Some(user.last_name),
                email_address: Some(user.email_address),
                created_at: Some(user.created_at.and_utc()),
                updated_at: Some(user.updated_at.and_utc()),
                deleted_at: user.deleted_at.map(|datetime| datetime.and_utc()),
            })
        )
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let id = Uuid::now_v7();
        let result = user_resolver::user(&pool, Some(id)).await.unwrap();

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_user_missing_id() {
        // TODO: Add a function `connect_database` to `test` module and return test connection
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let result = user_resolver::user(&pool, None).await.unwrap_err();

        // TODO: Check the error object
        assert_eq!(result, unprocessable_content("reason".to_string()))
    }

    #[tokio::test]
    async fn test_user_integration() {
        let user = factory::insert_user();
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
                    "deletedAt": Null
                }
            })
        );
        assert!(response.errors.is_empty());
        assert!(response.extensions.is_empty());
    }

    #[tokio::test]
    async fn test_create_user() {
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let input = UserInput {
            first_name: Some("Jane".to_string()),
            last_name: Some("Doe".to_string()),
            email_address: Some("jane.doe@example.com".to_string()),
        };
        let result = user_resolver::create_user(&pool, Some(input))
            .await
            .unwrap();

        if let Some(user) = result {
            assert_eq!(user.first_name, Some("Jane".to_string()));
            assert_eq!(user.last_name, Some("Doe".to_string()));
            assert_eq!(user.email_address, Some("jane.doe@example.com".to_string()));
            assert_eq!(user.created_at, user.updated_at);
            assert_eq!(user.deleted_at, None);
        } else {
            assert_ne!(result, None);
        }
    }

    #[tokio::test]
    async fn test_create_user_missing_input() {
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let result = user_resolver::create_user(&pool, None).await.unwrap_err();

        // TODO: Check the error object
        // TODO: Maybe an error enum makes sense?
        assert_eq!(result, unprocessable_content("reason".to_string()))
    }

    #[tokio::test]
    async fn test_create_user_invalid_input() {
        let config = config::get_config();
        let pool = repo::connect_database(&config.database_url);
        let input = UserInput {
            first_name: Some("J".to_string()),
            last_name: Some("D".to_string()),
            email_address: Some("jane.doe@@example.com".to_string()),
        };
        let result = user_resolver::create_user(&pool, Some(input))
            .await
            .unwrap_err();

        // TODO: Check the error object
        assert_eq!(result, unprocessable_content("reason".to_string()))
    }
}
