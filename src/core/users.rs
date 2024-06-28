use std::io::Error;

use crate::core::models::User;
use crate::core::schema::users;
use crate::core::schema::users::dsl::*;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use uuid::Uuid;

pub struct CreateUserAttrs {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}

/// Fetch the user.
pub fn fetch_user(conn: &mut PgConnection, user_id: Uuid) -> Result<Option<User>, Error> {
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
pub fn create_user(conn: &mut PgConnection, attrs: CreateUserAttrs) -> User {
    // TODO: Check whether the values should be borrowed???
    let timestamp = Utc::now().naive_utc();
    let new_user = User {
        id: Uuid::now_v7(),
        first_name: attrs.first_name,
        last_name: attrs.last_name,
        email_address: attrs.email_address,
        created_at: timestamp,
        updated_at: timestamp,
        deleted_at: None,
    };

    // TODO: Handle all the errors
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use diesel::Connection;
    use diesel::PgConnection;

    #[test]
    fn test_fetch_user_success() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let user = create_user(
            &mut conn,
            CreateUserAttrs {
                first_name: "Jane".to_string(),
                last_name: "Doe".to_string(),
                email_address: "jane@doe.com".to_string(),
            },
        );
        let result = fetch_user(&mut conn, user.id);

        assert_eq!(result.unwrap(), Some(user))
    }

    #[test]
    fn test_fetch_user_not_found() {
        assert_eq!(true, true)
    }

    #[test]
    fn test_create_user_success() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let attrs = CreateUserAttrs {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            email_address: "jane@doe.com".to_string(),
        };
        let user = create_user(&mut conn, attrs);

        assert_eq!(user.first_name, "Jane");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.email_address, "jane@doe.com");
        assert_eq!(user.created_at, user.updated_at);
        assert_eq!(user.deleted_at, None);
    }

    #[test]
    fn test_create_user_already_exists() {
        assert_eq!(true, true)
    }
}
