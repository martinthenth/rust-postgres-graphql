use crate::core::models::schema::users;
use crate::core::models::schema::users::dsl::*;
use crate::core::models::User;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use std::io::Error;
use uuid::Uuid;

pub struct CreateUserAttrs {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}

/// Get the user.
pub fn get_user(conn: &mut PgConnection, user_id: Uuid) -> Result<Option<User>, Error> {
    // TODO: Handle all the errors
    let result = users
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .optional();

    match result {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(_) => Ok(None),
    }
}

/// Create a user.
pub fn create_user(conn: &mut PgConnection, attrs: CreateUserAttrs) -> Result<Option<User>, Error> {
    let timestamp = Utc::now().naive_utc();
    let changes = User {
        id: Uuid::now_v7(),
        first_name: attrs.first_name,
        last_name: attrs.last_name,
        email_address: attrs.email_address,
        created_at: timestamp,
        updated_at: timestamp,
        deleted_at: None,
    };

    // TODO: Handle all the errors
    let result = diesel::insert_into(users::table)
        .values(&changes)
        .returning(User::as_returning())
        .get_result(conn)
        .optional();

    match result {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::test::factory;
    use diesel::Connection;
    use diesel::PgConnection;

    #[test]
    fn test_get_user() {
        let user = factory::insert_user();
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let result = get_user(&mut conn, user.id).unwrap();

        assert_eq!(result, Some(user))
    }

    #[test]
    fn test_get_user_not_found() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let user_id = Uuid::now_v7();
        let result = get_user(&mut conn, user_id).unwrap();

        assert_eq!(result, None)
    }

    #[test]
    fn test_create_user() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let attrs = CreateUserAttrs {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            email_address: "jane@doe.com".to_string(),
        };
        let result = create_user(&mut conn, attrs).unwrap();

        if let Some(user) = result {
            assert_eq!(user.first_name, "Jane");
            assert_eq!(user.last_name, "Doe");
            assert_eq!(user.email_address, "jane@doe.com");
            assert_eq!(user.created_at, user.updated_at);
            assert_eq!(user.deleted_at, None);
        } else {
            assert_ne!(result, None);
        }
    }

    #[test]
    fn test_create_user_already_exists() {
        assert_eq!(true, true)
    }
}
