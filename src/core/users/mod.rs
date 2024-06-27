use crate::core::models::User;
use crate::core::schema::users;
use chrono::Utc;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use uuid::Uuid;

/// Create a user.
pub fn create_user(conn: &mut PgConnection, first_name: String, last_name: String) -> User {
    // TODO: Check whether the values should be borrowed???
    let timestamp = Utc::now().naive_utc();
    let new_user = User {
        id: Uuid::now_v7(),
        first_name,
        last_name,
        banned_at: None,
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
    fn test_create_user() {
        let config = config::get_config();
        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        let user = create_user(&mut conn, "Jane".to_string(), "Doe".to_string());

        assert_eq!(user.first_name, "Jane");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.banned_at, None);
        assert_eq!(user.created_at, user.updated_at);
        assert_eq!(user.deleted_at, None);
    }
}
