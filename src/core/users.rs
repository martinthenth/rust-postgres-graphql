use crate::core::models::User;
use crate::core::schema::users;
use crate::core::schema::users::dsl::*;
use chrono::Utc;
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

/// Get a user.
pub fn get_user(conn: &mut PgConnection, user_id: Uuid) -> User {
    users
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .unwrap()
}

/// Create a user.
pub fn create_user(conn: &mut PgConnection, params: CreateUserAttrs) -> User {
    // TODO: Check whether the values should be borrowed???
    let timestamp = Utc::now().naive_utc();
    let new_user = User {
        id: Uuid::now_v7(),
        first_name: params.first_name,
        last_name: params.last_name,
        email_address: params.email_address,
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
    fn test_get_user_success() {
        // TODO: Implement test.
        assert_eq!(true, true)
    }

    #[test]
    fn test_create_user_success() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let params = CreateUserAttrs {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            email_address: "jane@doe.com".to_string(),
        };
        let user = create_user(&mut conn, params);

        assert_eq!(user.first_name, "Jane");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.email_address, "jane@doe.com");
        assert_eq!(user.created_at, user.updated_at);
        assert_eq!(user.deleted_at, None);
    }
}
