use crate::core::models::User;
use crate::{config, core::users};
use diesel::Connection;
use diesel::PgConnection;

pub fn insert_user() -> User {
    let config = config::get_config();
    let mut conn = PgConnection::establish(&config.database_url).unwrap();

    users::create_user(
        &mut conn,
        users::CreateUserAttrs {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            email_address: "jane@doe.com".to_string(),
        },
    )
    .unwrap()
    .unwrap()
}
